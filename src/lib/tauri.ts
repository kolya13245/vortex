import { invoke } from "@tauri-apps/api/core";
import type {
  CoreStatus,
  CoreType,
  TrafficStats,
  UserSettings,
  Subscription,
} from "@/types/core";

export const getCoreStatus = () => invoke<CoreStatus>("get_core_status");
export const getSettings = () => invoke<UserSettings>("get_settings");
export const updateSettings = (settings: UserSettings) =>
  invoke<void>("update_settings", { settings });
export const getHwid = () => invoke<string>("get_hwid");
export const getPlatform = () => invoke<string>("get_platform");
export const connectCore = () => invoke<void>("connect");
export const disconnectCore = () => invoke<void>("disconnect");
export const getTraffic = () => invoke<TrafficStats>("get_traffic");
export const getSubscriptions = () =>
  invoke<Subscription[]>("get_subscriptions");
export const addSubscription = (name: string, url: string) =>
  invoke<Subscription>("add_subscription", { name, url });
export const removeSubscription = (id: string) =>
  invoke<void>("remove_subscription", { id });
export const updateSubscription = (id: string) =>
  invoke<Subscription>("update_subscription", { id });
export const switchCore = (coreType: CoreType) =>
  invoke<void>("switch_core", { coreType });
export const getLogs = () => invoke<string[]>("get_logs");
