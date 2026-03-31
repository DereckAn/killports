export interface PortInfo {
  local_port: number;
  local_address: string;
  remote_port: number;
  remote_address: string;
  state: string;
  protocol: string;
  process_name: string | null;
  pid: number | null;
}

export interface PortFilters {
  search_query: string | null;
  protocol_filter: string | null;
  state_filter: string | null;
  address_type: string | null;
  hide_system_processes: boolean;
  hide_ephemeral_ports: boolean;
  port_range_min: number | null;
  port_range_max: number | null;
}
