use sysinfo::System;

#[derive(serde::Serialize)]
pub struct PortInfo {
    pub local_port: u16,
    pub local_address: String,
    pub remote_port: u16,
    pub remote_address: String,
    pub state: String,
    pub protocol: String,
    pub process_name: Option<String>,
    pub pid: Option<u32>,
    pub command_line: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct PortFilters {
    pub search_query: Option<String>,
    pub protocol_filter: Option<Vec<String>>,
    pub state_filter: Option<Vec<String>>,
    pub address_type: Option<Vec<String>>,
    pub hide_system_processes: bool,
    pub hide_ephemeral_ports: bool,
    pub port_range_min: Option<u16>,
    pub port_range_max: Option<u16>,
}

#[tauri::command]
pub fn get_active_ports(filters: PortFilters) -> Result<Vec<PortInfo>, String> {
    use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};

    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;

    let sockets = get_sockets_info(af_flags, proto_flags)
        .map_err(|e| format!("Failed to get socket info: {}", e))?;

    let mut sys = System::new_all();
    sys.refresh_all();

    // System process names to filter
    let system_processes = vec![
        "system",
        "svchost.exe",
        "lsass.exe",
        "wininit.exe",
        "services.exe",
        "[system process]",
        "spoolsv.exe",
    ];

    let ports: Vec<PortInfo> = sockets
        .iter()
        .filter_map(|socket| {
            let (local_port, local_addr, remote_port, remote_addr, state, protocol) =
                match &socket.protocol_socket_info {
                    ProtocolSocketInfo::Tcp(tcp_info) => (
                        tcp_info.local_port,
                        tcp_info.local_addr.to_string(),
                        tcp_info.remote_port,
                        tcp_info.remote_addr.to_string(),
                        format!("{:?}", tcp_info.state),
                        "TCP".to_string(),
                    ),
                    ProtocolSocketInfo::Udp(udp_info) => (
                        udp_info.local_port,
                        udp_info.local_addr.to_string(),
                        0,
                        "0.0.0.0".to_string(),
                        "N/A".to_string(),
                        "UDP".to_string(),
                    ),
                };

            let pid = socket.associated_pids.first().map(|p| *p as u32);
            let (process_name, command_line) = pid
                .map(|p| {
                    if let Some(process) = sys.process(sysinfo::Pid::from_u32(p)) {
                        let name = process.name().to_str().unwrap_or("Unknown");
                        let name_str = format!("{} (PID: {})", name, p);
                        let cmd: Vec<&str> =
                            process.cmd().iter().filter_map(|s| s.to_str()).collect();
                        let cmd_str = if cmd.is_empty() {
                            None
                        } else {
                            Some(cmd.join(" "))
                        };
                        (Some(name_str), cmd_str)
                    } else {
                        (Some(format!("PID: {}", p)), None)
                    }
                })
                .unwrap_or((None, None));

            let port_info = PortInfo {
                local_port,
                local_address: local_addr.clone(),
                remote_port,
                remote_address: remote_addr,
                state: state.clone(),
                protocol: protocol.clone(),
                process_name: process_name.clone(),
                pid,
                command_line: command_line.clone(),
            };

            // Apply filters

            // Protocol filter (multi-select: OR logic)
            if let Some(ref protocols) = filters.protocol_filter {
                if !protocols.is_empty() && !protocols.contains(&protocol) {
                    return None;
                }
            }

            // State filter (multi-select: OR logic)
            if let Some(ref states) = filters.state_filter {
                if !states.is_empty() && !states.contains(&state) {
                    return None;
                }
            }

            // Address type filter (multi-select)
            if let Some(ref addr_types) = filters.address_type {
                if !addr_types.is_empty() {
                    let is_localhost =
                        local_addr.starts_with("127.0.0.1") || local_addr.starts_with("::1");
                    let matches = (is_localhost && addr_types.contains(&"localhost".to_string()))
                        || (!is_localhost && addr_types.contains(&"network".to_string()));
                    if !matches {
                        return None;
                    }
                }
            }

            // Hide system processes
            if filters.hide_system_processes {
                if let Some(ref proc_name) = process_name {
                    let proc_lower = proc_name.to_lowercase();
                    if system_processes
                        .iter()
                        .any(|sys_proc| proc_lower.contains(sys_proc))
                    {
                        return None;
                    }
                }
            }

            // Hide ephemeral ports (49152-65535)
            if filters.hide_ephemeral_ports && local_port >= 49152 {
                return None;
            }

            // Port range filter
            if let Some(min) = filters.port_range_min {
                if local_port < min {
                    return None;
                }
            }
            if let Some(max) = filters.port_range_max {
                if local_port > max {
                    return None;
                }
            }

            // Search query (searches in port, process name, IP address)
            if let Some(ref query) = filters.search_query {
                if !query.is_empty() {
                    let query_lower = query.to_lowercase();
                    let matches = local_port.to_string().contains(&query_lower)
                        || local_addr.to_lowercase().contains(&query_lower)
                        || process_name
                            .as_ref()
                            .map_or(false, |p| p.to_lowercase().contains(&query_lower))
                        || state.to_lowercase().contains(&query_lower);

                    if !matches {
                        return None;
                    }
                }
            }

            Some(port_info)
        })
        .collect();

    Ok(ports)
}

// Helper command to get unique states for the filter dropdown
#[tauri::command]
pub fn get_available_states() -> Result<Vec<String>, String> {
    Ok(vec![
        "Listen".to_string(),
        "Established".to_string(),
        "TimeWait".to_string(),
        "CloseWait".to_string(),
        "FinWait1".to_string(),
        "FinWait2".to_string(),
        "SynSent".to_string(),
        "SynReceived".to_string(),
        "N/A".to_string(),
    ])
}
