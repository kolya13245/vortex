export type CoreType = "mihomo" | "xray";
export type ConnectionStatus = "disconnected" | "connecting" | "connected" | "error";

export interface CoreStatus {
  core_type: CoreType;
  connection_status: ConnectionStatus;
  uptime_secs: number;
  current_proxy: string | null;
}

export interface TrafficStats {
  download_speed: number;
  upload_speed: number;
  download_total: number;
  upload_total: number;
}

export interface UserSettings {
  core_type: CoreType;
  tun_enabled: boolean;
  system_proxy: boolean;
  mixed_port: number;
  allow_lan: boolean;
  theme: string;
  language: string;
  mihomo_api_port: number;
  mihomo_api_secret: string;
  dns_mode: string;
  dns_servers: string[];
}

export interface ServerNode {
  name: string;
  protocol: string;
  address: string;
  port: number;
}

export interface Subscription {
  id: string;
  name: string;
  url: string;
  core_type: CoreType;
  node_count: number;
  nodes: ServerNode[];
  traffic_used: number | null;
  traffic_total: number | null;
  expire: string | null;
  last_updated: string | null;
}
