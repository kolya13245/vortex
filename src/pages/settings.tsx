import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { useSetAtom } from "jotai";
import { getSettings, updateSettings, getHwid, getPlatform } from "@/lib/tauri";
import { coreTypeAtom } from "@/stores/atoms";
import { Copy, Check } from "lucide-react";
import { useState } from "react";
import { cn } from "@/lib/utils";
import type { CoreType, UserSettings } from "@/types/core";

export function SettingsPage() {
  const queryClient = useQueryClient();
  const setCoreType = useSetAtom(coreTypeAtom);

  const { data: settings } = useQuery({
    queryKey: ["settings"],
    queryFn: getSettings,
  });

  const { data: hwid } = useQuery({
    queryKey: ["hwid"],
    queryFn: getHwid,
  });

  const { data: platform } = useQuery({
    queryKey: ["platform"],
    queryFn: getPlatform,
  });

  const updateMut = useMutation({
    mutationFn: (s: UserSettings) => updateSettings(s),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["settings"] }),
  });

  const handleCoreChange = (core: CoreType) => {
    if (!settings) return;
    const updated = { ...settings, core_type: core };
    setCoreType(core);
    updateMut.mutate(updated);
  };

  const handleToggle = (key: keyof UserSettings) => {
    if (!settings) return;
    const updated = { ...settings, [key]: !settings[key] };
    updateMut.mutate(updated);
  };

  if (!settings) return null;

  return (
    <div className="max-w-2xl">
      <h1 className="text-2xl font-semibold text-[--color-text-primary] mb-6">Settings</h1>

      {/* Core Selection */}
      <Section title="Core">
        <SettingRow label="Proxy Core" description="Select which proxy engine to use">
          <div className="glass rounded-lg flex overflow-hidden">
            <button
              onClick={() => handleCoreChange("mihomo")}
              className={cn(
                "px-4 py-2 text-sm font-medium transition-colors",
                settings.core_type === "mihomo"
                  ? "bg-accent/10 text-accent"
                  : "text-[--color-text-secondary] hover:text-[--color-text-primary]",
              )}
            >
              Mihomo
            </button>
            <button
              onClick={() => handleCoreChange("xray")}
              className={cn(
                "px-4 py-2 text-sm font-medium transition-colors",
                settings.core_type === "xray"
                  ? "bg-accent/10 text-accent"
                  : "text-[--color-text-secondary] hover:text-[--color-text-primary]",
              )}
            >
              Xray
            </button>
          </div>
        </SettingRow>

        <SettingRow
          label="TUN Mode"
          description="Route all system traffic through proxy (requires admin)"
        >
          <Toggle checked={settings.tun_enabled} onChange={() => handleToggle("tun_enabled")} />
        </SettingRow>

        <SettingRow label="System Proxy" description="Set system-wide proxy settings">
          <Toggle
            checked={settings.system_proxy}
            onChange={() => handleToggle("system_proxy")}
          />
        </SettingRow>

        <SettingRow label="Allow LAN" description="Allow connections from local network">
          <Toggle checked={settings.allow_lan} onChange={() => handleToggle("allow_lan")} />
        </SettingRow>

        <SettingRow label="Mixed Port" description="HTTP + SOCKS proxy port">
          <input
            type="number"
            value={settings.mixed_port}
            onChange={(e) => {
              const updated = { ...settings, mixed_port: parseInt(e.target.value) || 7890 };
              updateMut.mutate(updated);
            }}
            className="w-24 bg-white/[0.05] border border-white/[0.10] rounded-lg px-3 py-1.5 text-sm font-mono text-[--color-text-primary] text-right focus:border-accent/50 focus:outline-none"
          />
        </SettingRow>
      </Section>

      {/* DNS */}
      <Section title="DNS">
        <SettingRow label="DNS Mode" description="Mihomo DNS resolution mode">
          <select
            value={settings.dns_mode}
            onChange={(e) => {
              const updated = { ...settings, dns_mode: e.target.value };
              updateMut.mutate(updated);
            }}
            className="bg-white/[0.05] border border-white/[0.10] rounded-lg px-3 py-1.5 text-sm text-[--color-text-primary] focus:border-accent/50 focus:outline-none"
          >
            <option value="fake-ip">Fake IP</option>
            <option value="redir-host">Redir Host</option>
            <option value="normal">Normal</option>
          </select>
        </SettingRow>
      </Section>

      {/* About */}
      <Section title="About">
        <SettingRow label="HWID" description="Hardware ID sent to subscription providers">
          <CopyField value={hwid ?? "..."} />
        </SettingRow>

        <SettingRow label="Platform" description="Current operating system">
          <span className="text-sm font-mono text-[--color-text-primary] capitalize">
            {platform ?? "..."}
          </span>
        </SettingRow>

        <SettingRow label="Version" description="Vortex version">
          <span className="text-sm font-mono text-[--color-text-primary]">0.1.0</span>
        </SettingRow>
      </Section>
    </div>
  );
}

function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <div className="mb-8">
      <h2 className="text-xs font-medium uppercase tracking-wider text-[--color-text-secondary] mb-3">
        {title}
      </h2>
      <div className="glass rounded-xl divide-y divide-white/[0.06]">{children}</div>
    </div>
  );
}

function SettingRow({
  label,
  description,
  children,
}: {
  label: string;
  description?: string;
  children: React.ReactNode;
}) {
  return (
    <div className="flex items-center justify-between px-4 py-3">
      <div>
        <p className="text-sm text-[--color-text-primary]">{label}</p>
        {description && <p className="text-xs text-[--color-text-tertiary] mt-0.5">{description}</p>}
      </div>
      {children}
    </div>
  );
}

function Toggle({ checked, onChange }: { checked: boolean; onChange: () => void }) {
  return (
    <button
      onClick={onChange}
      className={cn(
        "relative w-10 h-5 rounded-full transition-colors duration-200",
        checked ? "bg-accent" : "bg-white/[0.12]",
      )}
    >
      <span
        className={cn(
          "absolute top-0.5 w-4 h-4 rounded-full bg-white shadow-sm transition-transform duration-200",
          checked ? "translate-x-[22px]" : "translate-x-0.5",
        )}
      />
    </button>
  );
}

function CopyField({ value }: { value: string }) {
  const [copied, setCopied] = useState(false);

  const copy = () => {
    navigator.clipboard.writeText(value);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="flex items-center gap-2">
      <span className="text-xs font-mono text-[--color-text-secondary] max-w-[180px] truncate">
        {value}
      </span>
      <button
        onClick={copy}
        className="p-1 rounded text-[--color-text-tertiary] hover:text-[--color-text-primary] transition-colors"
      >
        {copied ? <Check className="w-3.5 h-3.5 text-accent" /> : <Copy className="w-3.5 h-3.5" />}
      </button>
    </div>
  );
}
