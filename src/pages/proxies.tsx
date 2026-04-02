import { Globe, Wifi, ChevronDown, ChevronRight, ToggleLeft, ToggleRight } from "lucide-react";
import { useAtomValue } from "jotai";
import { useState } from "react";
import { coreTypeAtom, activePageAtom } from "@/stores/atoms";
import { useSetAtom } from "jotai";
import { cn } from "@/lib/utils";

interface MockNode {
  name: string;
  latency: number | null;
}

interface MockGroup {
  name: string;
  type: string;
  current: string;
  nodes: MockNode[];
}

const mockGroups: MockGroup[] = [
  {
    name: "Proxy",
    type: "Selector",
    current: "Auto Select",
    nodes: [
      { name: "Auto Select", latency: null },
      { name: "Hong Kong 01", latency: 42 },
      { name: "Hong Kong 02", latency: 78 },
      { name: "Singapore 01", latency: 120 },
      { name: "Japan 01", latency: 95 },
      { name: "US West 01", latency: 180 },
      { name: "US East 01", latency: 220 },
      { name: "Germany 01", latency: 165 },
    ],
  },
  {
    name: "Auto Select",
    type: "URLTest",
    current: "Hong Kong 01",
    nodes: [
      { name: "Hong Kong 01", latency: 42 },
      { name: "Hong Kong 02", latency: 78 },
      { name: "Japan 01", latency: 95 },
    ],
  },
  {
    name: "Streaming",
    type: "Selector",
    current: "Singapore 01",
    nodes: [
      { name: "Singapore 01", latency: 120 },
      { name: "US West 01", latency: 180 },
      { name: "Japan 01", latency: 95 },
    ],
  },
];

export function ProxiesPage() {
  const coreType = useAtomValue(coreTypeAtom);
  const setActivePage = useSetAtom(activePageAtom);
  const [mode, setMode] = useState<"rule" | "global">("rule");
  const [expandedGroups, setExpandedGroups] = useState<Set<string>>(new Set(["Proxy"]));

  if (coreType === "xray") {
    return (
      <div className="flex flex-col items-center justify-center py-20">
        <Globe className="w-12 h-12 text-[--color-text-tertiary] mb-4" />
        <p className="text-[--color-text-secondary] text-sm mb-4">
          Proxy groups are available with Mihomo core
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

  const toggleGroup = (name: string) => {
    setExpandedGroups((prev) => {
      const next = new Set(prev);
      if (next.has(name)) next.delete(name);
      else next.add(name);
      return next;
    });
  };

  return (
    <div>
      {/* Header */}
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-semibold text-[--color-text-primary]">Proxies</h1>
        <button
          onClick={() => setMode(mode === "rule" ? "global" : "rule")}
          className="glass rounded-lg px-3 py-1.5 flex items-center gap-2 text-sm text-[--color-text-secondary] hover:text-[--color-text-primary] transition-colors"
        >
          {mode === "rule" ? (
            <ToggleLeft className="w-4 h-4" />
          ) : (
            <ToggleRight className="w-4 h-4 text-accent" />
          )}
          {mode === "rule" ? "Rule" : "Global"}
        </button>
      </div>

      {/* Groups */}
      <div className="space-y-3">
        {mockGroups.map((group) => {
          const isExpanded = expandedGroups.has(group.name);
          return (
            <div key={group.name} className="glass rounded-xl overflow-hidden">
              <button
                onClick={() => toggleGroup(group.name)}
                className="w-full flex items-center justify-between px-4 py-3 hover:bg-white/[0.03] transition-colors"
              >
                <div className="flex items-center gap-3">
                  <Wifi className="w-4 h-4 text-accent" />
                  <span className="text-sm font-medium text-[--color-text-primary]">
                    {group.name}
                  </span>
                  <span className="px-2 py-0.5 rounded-md text-xs bg-white/[0.06] text-[--color-text-secondary]">
                    {group.type}
                  </span>
                </div>
                <div className="flex items-center gap-3">
                  <span className="text-xs text-[--color-text-secondary]">{group.current}</span>
                  {isExpanded ? (
                    <ChevronDown className="w-4 h-4 text-[--color-text-tertiary]" />
                  ) : (
                    <ChevronRight className="w-4 h-4 text-[--color-text-tertiary]" />
                  )}
                </div>
              </button>

              {isExpanded && (
                <div className="px-4 pb-3 grid grid-cols-2 gap-2">
                  {group.nodes.map((node) => (
                    <button
                      key={node.name}
                      className={cn(
                        "flex items-center justify-between px-3 py-2 rounded-lg text-sm transition-colors",
                        node.name === group.current
                          ? "bg-accent/10 border border-accent/20 text-accent"
                          : "bg-white/[0.03] border border-white/[0.04] text-[--color-text-primary] hover:bg-white/[0.06]",
                      )}
                    >
                      <span className="truncate">{node.name}</span>
                      {node.latency !== null && <LatencyBadge ms={node.latency} />}
                    </button>
                  ))}
                </div>
              )}
            </div>
          );
        })}
      </div>
    </div>
  );
}

function LatencyBadge({ ms }: { ms: number }) {
  const color =
    ms < 100
      ? "bg-[rgb(0,212,170)]/15 text-[rgb(0,212,170)]"
      : ms < 300
        ? "bg-[rgb(240,180,41)]/15 text-[rgb(240,180,41)]"
        : "bg-[rgb(239,68,68)]/15 text-[rgb(239,68,68)]";
  return (
    <span className={cn("px-1.5 py-0.5 rounded text-xs font-mono ml-2 flex-shrink-0", color)}>
      {ms}ms
    </span>
  );
}
