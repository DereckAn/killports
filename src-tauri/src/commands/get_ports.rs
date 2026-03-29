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
}

#[derive(serde::Deserialize)]
pub struct PortFilters {
    pub search_query: Option<String>,
    pub protocol_filter: Option<String>,
    pub state_filter: Option<String>,
    pub address_type: Option<String>,
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
            let process_name = pid.and_then(|p| {
                sys.process(sysinfo::Pid::from_u32(p))
                    .map(|process| {
                        let name = process.name().to_str().unwrap_or("Unknown");
                        format!("{} (PID: {})", name, p)
                    })
                    .or_else(|| Some(format!("PID: {}", p)))
            });

            let port_info = PortInfo {
                local_port,
                local_address: local_addr.clone(),
                remote_port,
                remote_address: remote_addr,
                state: state.clone(),
                protocol: protocol.clone(),
                process_name: process_name.clone(),
                pid,
            };

            // Apply filters

            // Protocol filter
            if let Some(ref prot) = filters.protocol_filter {
                if !prot.is_empty() && protocol != *prot {
                    return None;
                }
            }

            // State filter
            if let Some(ref st) = filters.state_filter {
                if !st.is_empty() && state != *st {
                    return None;
                }
            }

            // Address type filter
            if let Some(ref addr_type) = filters.address_type {
                match addr_type.as_str() {
                    "localhost" => {
                        if !local_addr.starts_with("127.0.0.1") && !local_addr.starts_with("::1") {
                            return None;
                        }
                    }
                    "network" => {
                        if local_addr.starts_with("127.0.0.1") || local_addr.starts_with("::1") {
                            return None;
                        }
                    }
                    _ => {} // "all" or empty - no filter
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
