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
  }

#[tauri::command]
pub fn get_active_ports() -> Result<Vec<PortInfo>, String> {
    use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};

    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;

    let sockets = get_sockets_info(af_flags, proto_flags).map_err(|e| format!("Failed to get socket info: {}", e))?;

    let mut sys = System::new_all();
    sys.refresh_all();

      let ports: Vec<PortInfo> = sockets
          .iter()
          .map(|socket| {
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

              let process_name = socket.associated_pids.first().and_then(|pid| {
                  sys.process(sysinfo::Pid::from_u32(*pid as u32))
                      .map(|process| format!("{} (PID: {})",
  process.name().to_str().unwrap_or("Unknown"), pid))
              });

              PortInfo {
                  local_port,
                  local_address: local_addr,
                  remote_port,
                  remote_address: remote_addr,
                  state,
                  protocol,
                  process_name,
              }
          })
          .collect();

      Ok(ports)
}