import { Cable, X } from "lucide-react";
import { useAtomValue, useSetAtom } from "jotai";
import { coreTypeAtom, activePageAtom } from "@/stores/atoms";
import { formatBytes } from "@/lib/utils";

interface MockConnection {
  id: string;
  host: string;
  network: string;
  type: string;
  chains: string;
  rule: string;
  download: number;
  upload: number;
  time: string;
}

const mockConnections: MockConnection[] = [
  { id: "1", host: "google.com:443", network: "TCP", type: "HTTPS", chains: "HK 01", rule: "GEOSITE:google", download: 52400, upload: 8200, time: "2m 30s" },
  { id: "2", host: "api.github.com:443", network: "TCP", type: "HTTPS", chains: "US West 01", rule: "DOMAIN-SUFFIX:github.com", download: 128000, upload: 24000, time: "1m 15s" },
  { id: "3", host: "cdn.jsdelivr.net:443", network: "TCP", type: "HTTPS", chains: "DIRECT", rule: "GEOIP:CN", download: 1024000, upload: 5120, time: "45s" },
  { id: "4", host: "8.8.8.8:53", network: "UDP", type: "DNS", chains: "HK 01", rule: "DST-PORT:53", download: 512, upload: 256, time: "5s" },
];

export function ConnectionsPage() {
  const coreType = useAtomValue(coreTypeAtom);
  const setActivePage = useSetAtom(activePageAtom);

  if (coreType === "xray") {
    return (
      <div className="flex flex-col items-center justify-center py-20">
        <Cable className="w-12 h-12 text-[--color-text-tertiary] mb-4" />
        <p className="text-[--color-text-secondary] text-sm mb-4">
          Connection tracking is available with Mihomo core
        </p>
        <button
          onClick={() => setActivePage("settings")}
          className="bg-accent hover:bg-accent-hover text-[--color-text-inverse] px-4 py-2 rounded-lg text-sm font-medium transition-colors"
        >
          Switch to Mihomo
        </button>
      </div>
    );
  }

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <div className="flex items-center gap-3">
          <h1 className="text-2xl font-semibold text-[--color-text-primary]">Connections</h1>
          <span className="px-2 py-0.5 rounded-md text-xs bg-accent/10 text-accent font-mono">
            {mockConnections.length}
          </span>
        </div>
        <button className="glass rounded-lg px-3 py-1.5 text-sm text-[--color-text-secondary] hover:text-danger transition-colors flex items-center gap-1.5">
          <X className="w-3.5 h-3.5" />
          Close All
        </button>
      </div>

      <div className="glass rounded-xl overflow-hidden">
        <table className="w-full text-sm">
          <thead>
            <tr className="border-b border-white/[0.06]">
              {["Host", "Network", "Rule", "Chains", "Download", "Upload", "Time", ""].map(
                (h) => (
                  <th
                    key={h}
                    className="text-left px-4 py-3 text-[--color-text-secondary] text-xs font-medium uppercase tracking-wider"
                  >
                    {h}
                  </th>
                ),
              )}
            </tr>
          </thead>
          <tbody>
            {mockConnections.map((conn) => (
              <tr
                key={conn.id}
                className="border-b border-white/[0.04] hover:bg-white/[0.03] transition-colors group"
              >
                <td className="px-4 py-2.5 text-[--color-text-primary] font-mono text-xs">
                  {conn.host}
                </td>
                <td className="px-4 py-2.5">
                  <span className="px-1.5 py-0.5 rounded text-xs bg-white/[0.06] text-[--color-text-secondary]">
                    {conn.network}
                  </span>
                </td>
                <td className="px-4 py-2.5 text-xs text-[--color-text-secondary] max-w-[180px] truncate">
                  {conn.rule}
                </td>
                <td className="px-4 py-2.5 text-xs text-accent">{conn.chains}</td>
                <td className="px-4 py-2.5 font-mono text-xs text-[--color-text-primary]">
                  {formatBytes(conn.download)}
                </td>
                <td className="px-4 py-2.5 font-mono text-xs text-[--color-text-primary]">
                  {formatBytes(conn.upload)}
                </td>
                <td className="px-4 py-2.5 font-mono text-xs text-[--color-text-secondary]">
                  {conn.time}
                </td>
                <td className="px-4 py-2.5">
                  <button className="p-1 rounded text-[--color-text-tertiary] hover:text-danger opacity-0 group-hover:opacity-100 transition-all">
                    <X className="w-3.5 h-3.5" />
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
