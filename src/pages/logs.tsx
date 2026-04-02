import { useState, useEffect, useRef } from "react";
import { ArrowDownToLine, Pause, Search } from "lucide-react";
import { cn } from "@/lib/utils";

interface LogEntry {
  time: string;
  level: "info" | "warn" | "error" | "debug";
  message: string;
}

const mockLogs: LogEntry[] = [
  { time: "15:42:01", level: "info", message: "Vortex v0.1.0 started" },
  { time: "15:42:01", level: "info", message: "Loading configuration..." },
  { time: "15:42:02", level: "info", message: "Core: Mihomo (Clash Meta)" },
  { time: "15:42:02", level: "info", message: "Mixed proxy listening on :7890" },
  { time: "15:42:02", level: "info", message: "RESTful API listening on 127.0.0.1:9090" },
  { time: "15:42:03", level: "info", message: "DNS server started" },
  { time: "15:42:03", level: "debug", message: "Loading GeoIP database..." },
  { time: "15:42:03", level: "debug", message: "Loading GeoSite database..." },
  { time: "15:42:04", level: "info", message: "Proxy groups initialized: 3 groups, 8 nodes" },
  { time: "15:42:05", level: "warn", message: "Node 'US East 02' health check failed: timeout" },
  { time: "15:42:06", level: "info", message: "URLTest group 'Auto Select': selected 'Hong Kong 01' (42ms)" },
  { time: "15:42:10", level: "info", message: "New connection: google.com:443 -> HK 01 [GEOSITE:google]" },
  { time: "15:42:11", level: "debug", message: "DNS resolve: google.com -> 142.250.185.78" },
  { time: "15:42:15", level: "error", message: "Connection to 'Germany 02' failed: connection refused" },
  { time: "15:42:20", level: "info", message: "Traffic stats: ↓ 1.2 MB/s ↑ 256 KB/s" },
];

const levelColors: Record<string, string> = {
  info: "bg-accent/15 text-accent",
  warn: "bg-[rgb(240,180,41)]/15 text-[rgb(240,180,41)]",
  error: "bg-danger/15 text-danger",
  debug: "bg-white/[0.08] text-[--color-text-secondary]",
};

export function LogsPage() {
  const [autoScroll, setAutoScroll] = useState(true);
  const [filter, setFilter] = useState<string>("all");
  const [search, setSearch] = useState("");
  const containerRef = useRef<HTMLDivElement>(null);

  const filtered = mockLogs.filter((log) => {
    if (filter !== "all" && log.level !== filter) return false;
    if (search && !log.message.toLowerCase().includes(search.toLowerCase())) return false;
    return true;
  });

  useEffect(() => {
    if (autoScroll && containerRef.current) {
      containerRef.current.scrollTop = containerRef.current.scrollHeight;
    }
  }, [filtered.length, autoScroll]);

  return (
    <div className="flex flex-col h-[calc(100vh-7rem)]">
      <div className="flex items-center justify-between mb-4">
        <h1 className="text-2xl font-semibold text-[--color-text-primary]">Logs</h1>
        <div className="flex items-center gap-2">
          {/* Level filter */}
          <div className="glass rounded-lg flex overflow-hidden">
            {["all", "info", "warn", "error"].map((level) => (
              <button
                key={level}
                onClick={() => setFilter(level)}
                className={cn(
                  "px-3 py-1.5 text-xs font-medium transition-colors capitalize",
                  filter === level
                    ? "bg-accent/10 text-accent"
                    : "text-[--color-text-secondary] hover:text-[--color-text-primary]",
                )}
              >
                {level}
              </button>
            ))}
          </div>

          {/* Search */}
          <div className="relative">
            <Search className="absolute left-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-[--color-text-tertiary]" />
            <input
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              placeholder="Search..."
              className="bg-white/[0.05] border border-white/[0.10] rounded-lg pl-8 pr-3 py-1.5 text-xs text-[--color-text-primary] placeholder:text-[--color-text-tertiary] focus:border-accent/50 focus:outline-none w-40"
            />
          </div>

          {/* Auto-scroll */}
          <button
            onClick={() => setAutoScroll(!autoScroll)}
            className={cn(
              "glass rounded-lg p-1.5 transition-colors",
              autoScroll ? "text-accent" : "text-[--color-text-tertiary]",
            )}
            title={autoScroll ? "Auto-scroll on" : "Auto-scroll off"}
          >
            {autoScroll ? (
              <ArrowDownToLine className="w-4 h-4" />
            ) : (
              <Pause className="w-4 h-4" />
            )}
          </button>
        </div>
      </div>

      <div
        ref={containerRef}
        className="glass rounded-xl p-4 flex-1 overflow-y-auto font-mono text-xs space-y-0.5"
      >
        {filtered.map((log, i) => (
          <div key={i} className="flex items-start gap-3 py-0.5 hover:bg-white/[0.02] rounded px-1">
            <span className="text-[--color-text-tertiary] w-16 flex-shrink-0">{log.time}</span>
            <span
              className={cn(
                "px-1.5 py-0.5 rounded text-xs w-12 text-center flex-shrink-0",
                levelColors[log.level],
              )}
            >
              {log.level}
            </span>
            <span className="text-[--color-text-primary]">{log.message}</span>
          </div>
        ))}
      </div>
    </div>
  );
}
