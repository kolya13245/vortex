import {
  LayoutDashboard,
  Globe,
  FolderSync,
  Cable,
  ScrollText,
  Settings,
} from "lucide-react";
import { useAtom, useAtomValue } from "jotai";
import { useState } from "react";
import { activePageAtom, coreTypeAtom } from "@/stores/atoms";
import { cn } from "@/lib/utils";

const navItems = [
  { icon: LayoutDashboard, label: "Dashboard", id: "dashboard" },
  { icon: Globe, label: "Proxies", id: "proxies", mihomoOnly: true },
  { icon: FolderSync, label: "Profiles", id: "profiles" },
  { icon: Cable, label: "Connections", id: "connections", mihomoOnly: true },
  { icon: ScrollText, label: "Logs", id: "logs" },
  { icon: Settings, label: "Settings", id: "settings" },
] as const;

export function Sidebar() {
  const [activePage, setActivePage] = useAtom(activePageAtom);
  const coreType = useAtomValue(coreTypeAtom);
  const [expanded, setExpanded] = useState(false);

  return (
    <aside
      className={cn(
        "glass border-r border-white/[0.10] h-screen pt-8 flex flex-col transition-[width] duration-200 flex-shrink-0",
        expanded ? "w-52" : "w-16",
      )}
      onMouseEnter={() => setExpanded(true)}
      onMouseLeave={() => setExpanded(false)}
    >
      {/* Logo */}
      <div className="px-3 py-4 flex items-center justify-center gap-3">
        <div className="w-8 h-8 rounded-lg bg-accent/20 flex items-center justify-center flex-shrink-0">
          <span className="text-accent font-bold text-sm">V</span>
        </div>
        {expanded && (
          <span className="text-sm font-semibold text-[--color-text-primary] whitespace-nowrap">
            Vortex
          </span>
        )}
      </div>

      {/* Navigation */}
      <nav className="flex-1 px-2 py-2 space-y-1">
        {navItems.map(({ icon: Icon, label, id, ...rest }) => {
          const mihomoOnly = "mihomoOnly" in rest && rest.mihomoOnly;
          const isActive = activePage === id;
          const isDisabled = mihomoOnly && coreType === "xray";

          return (
            <button
              key={id}
              onClick={() => !isDisabled && setActivePage(id)}
              className={cn(
                "w-full flex items-center gap-3 px-3 py-2.5 rounded-lg transition-colors duration-150",
                isActive
                  ? "bg-accent/10 text-accent"
                  : isDisabled
                    ? "text-[--color-text-tertiary] cursor-not-allowed"
                    : "text-[--color-text-secondary] hover:bg-white/[0.06] hover:text-[--color-text-primary] cursor-pointer",
              )}
              title={expanded ? undefined : label}
            >
              <Icon className="w-5 h-5 flex-shrink-0" />
              {expanded && (
                <span className={cn("text-sm whitespace-nowrap", isActive && "font-medium")}>
                  {label}
                </span>
              )}
            </button>
          );
        })}
      </nav>

      {/* Core badge */}
      <div className="px-3 py-3 border-t border-white/[0.06]">
        <div className="flex items-center gap-2 justify-center">
          <span className="inline-flex items-center gap-1.5 px-2 py-0.5 rounded-md text-xs font-mono bg-accent/10 text-accent">
            <span className="w-1.5 h-1.5 rounded-full bg-accent" />
            {expanded
              ? coreType === "mihomo"
                ? "Mihomo"
                : "Xray"
              : coreType === "mihomo"
                ? "M"
                : "X"}
          </span>
        </div>
      </div>
    </aside>
  );
}
