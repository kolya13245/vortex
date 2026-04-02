import { Power, ShieldCheck, ArrowDown, ArrowUp, Activity, Zap, Loader, AlertTriangle } from "lucide-react";
import { useAtom, useAtomValue } from "jotai";
import { useState } from "react";
import { connectionStatusAtom, coreTypeAtom } from "@/stores/atoms";
import { cn, formatSpeed, formatBytes } from "@/lib/utils";
import { useQuery } from "@tanstack/react-query";
import { getCoreStatus, getTraffic, connectCore, disconnectCore } from "@/lib/tauri";

export function DashboardPage() {
  const [connectionStatus, setConnectionStatus] = useAtom(connectionStatusAtom);
  const coreType = useAtomValue(coreTypeAtom);
  const [errorMsg, setErrorMsg] = useState<string | null>(null);

  const { data: traffic } = useQuery({
    queryKey: ["traffic"],
    queryFn: getTraffic,
    refetchInterval: 1000,
  });

  useQuery({
    queryKey: ["core-status"],
    queryFn: async () => {
      const status = await getCoreStatus();
      setConnectionStatus(status.connection_status);
      return status;
    },
    refetchInterval: 2000,
  });

  const handleToggle = async () => {
    setErrorMsg(null);
    if (connectionStatus === "connected") {
      await disconnectCore();
      setConnectionStatus("disconnected");
    } else {
      setConnectionStatus("connecting");
      try {
        await connectCore();
        setConnectionStatus("connected");
      } catch (e) {
        setConnectionStatus("error");
        setErrorMsg(String(e));
      }
    }
  };

  const isConnected = connectionStatus === "connected";
  const isConnecting = connectionStatus === "connecting";

  return (
    <div className="flex flex-col items-center">
      {/* Status Hero */}
      <div className="flex flex-col items-center py-8">
        {/* Connection Orb */}
        <div className="relative flex items-center justify-center">
          {/* Outer glow */}
          <div
            className={cn(
              "absolute w-32 h-32 rounded-full blur-2xl transition-all duration-700",
              isConnected && "bg-accent/10 animate-orb-glow",
              isConnecting && "bg-[rgb(240,180,41)]/10 animate-orb-pulse",
              connectionStatus === "error" && "bg-danger/10 animate-orb-pulse-fast",
              connectionStatus === "disconnected" && "bg-white/[0.02]",
            )}
          />

          {/* Glass ring */}
          <div
            className={cn(
              "relative w-24 h-24 rounded-full glass flex items-center justify-center transition-all duration-500",
              isConnected && "border-2 border-accent/30 shadow-[0_0_30px_rgba(0,212,170,0.12),inset_0_0_20px_rgba(0,212,170,0.05)]",
              isConnecting && "border-2 border-[rgb(240,180,41)]/30",
              connectionStatus === "error" && "border-2 border-danger/30",
              connectionStatus === "disconnected" && "border-2 border-white/[0.10]",
            )}
          >
            <div
              className={cn(
                "w-16 h-16 rounded-full flex items-center justify-center transition-colors duration-500",
                isConnected && "bg-accent/10",
                isConnecting && "bg-[rgb(240,180,41)]/10",
                connectionStatus === "error" && "bg-danger/10",
                connectionStatus === "disconnected" && "bg-white/[0.05]",
              )}
            >
              {isConnecting ? (
                <Loader className="w-6 h-6 text-[rgb(240,180,41)] animate-spin" />
              ) : isConnected ? (
                <ShieldCheck className="w-6 h-6 text-accent" />
              ) : (
                <Power className="w-6 h-6 text-[--color-text-tertiary]" />
              )}
            </div>
          </div>
        </div>

        {/* Status text */}
        <p
          className={cn(
            "mt-4 text-sm font-medium",
            isConnected && "text-accent",
            isConnecting && "text-[rgb(240,180,41)]",
            connectionStatus === "error" && "text-danger",
            connectionStatus === "disconnected" && "text-[--color-text-secondary]",
          )}
        >
          {isConnected
            ? "Connected"
            : isConnecting
              ? "Connecting..."
              : connectionStatus === "error"
                ? "Connection Error"
                : "Disconnected"}
        </p>

        {/* Connect button */}
        <button
          onClick={handleToggle}
          disabled={isConnecting}
          className={cn(
            "mt-4 px-8 py-2.5 rounded-lg text-sm font-medium transition-all duration-150",
            isConnected
              ? "glass text-[--color-text-secondary] hover:text-danger hover:border-danger/30"
              : "bg-accent hover:bg-accent-hover text-[--color-text-inverse] shadow-[0_0_16px_rgba(0,212,170,0.15)] hover:shadow-[0_0_24px_rgba(0,212,170,0.25)]",
            isConnecting && "opacity-50 cursor-not-allowed",
          )}
        >
          {isConnected ? "Disconnect" : isConnecting ? "Connecting..." : connectionStatus === "error" ? "Retry" : "Connect"}
        </button>

        {/* Error message */}
        {errorMsg && (
          <div className="mt-3 flex items-center gap-2 glass rounded-lg px-3 py-2 max-w-sm">
            <AlertTriangle className="w-4 h-4 text-danger flex-shrink-0" />
            <p className="text-xs text-danger">{errorMsg}</p>
          </div>
        )}
      </div>

      {/* Stats Grid */}
      <div className="grid grid-cols-2 gap-3 mt-2 w-full max-w-md">
        <div className="glass rounded-xl p-4">
          <div className="flex items-center gap-2 text-[--color-text-secondary]">
            <ArrowDown className="w-4 h-4" />
            <span className="text-xs">Download</span>
          </div>
          <p className="mt-2 font-mono text-xl font-bold text-[--color-text-primary]">
            {formatSpeed(traffic?.download_speed ?? 0)}
          </p>
        </div>

        <div className="glass rounded-xl p-4">
          <div className="flex items-center gap-2 text-[--color-text-secondary]">
            <ArrowUp className="w-4 h-4" />
            <span className="text-xs">Upload</span>
          </div>
          <p className="mt-2 font-mono text-xl font-bold text-[--color-text-primary]">
            {formatSpeed(traffic?.upload_speed ?? 0)}
          </p>
        </div>

        <div className="glass rounded-xl p-4">
          <div className="flex items-center gap-2 text-[--color-text-secondary]">
            <Activity className="w-4 h-4" />
            <span className="text-xs">Session</span>
          </div>
          <p className="mt-2 font-mono text-sm text-[--color-text-primary]">
            &darr; {formatBytes(traffic?.download_total ?? 0)} / &uarr;{" "}
            {formatBytes(traffic?.upload_total ?? 0)}
          </p>
        </div>

        <div className="glass rounded-xl p-4">
          <div className="flex items-center gap-2 text-[--color-text-secondary]">
            <Zap className="w-4 h-4" />
            <span className="text-xs">Core</span>
          </div>
          <div className="mt-2">
            <span className="inline-flex items-center px-2 py-0.5 rounded-md text-xs font-mono bg-accent/10 text-accent">
              {coreType === "mihomo" ? "Mihomo" : "Xray"}
            </span>
          </div>
        </div>
      </div>
    </div>
  );
}
