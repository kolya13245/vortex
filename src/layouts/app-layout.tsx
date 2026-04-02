import { useAtomValue } from "jotai";
import { activePageAtom } from "@/stores/atoms";
import { Sidebar } from "./sidebar";
import { DashboardPage } from "@/pages/dashboard";
import { ProxiesPage } from "@/pages/proxies";
import { ProfilesPage } from "@/pages/profiles";
import { ConnectionsPage } from "@/pages/connections";
import { LogsPage } from "@/pages/logs";
import { SettingsPage } from "@/pages/settings";

const pages: Record<string, React.FC> = {
  dashboard: DashboardPage,
  proxies: ProxiesPage,
  profiles: ProfilesPage,
  connections: ConnectionsPage,
  logs: LogsPage,
  settings: SettingsPage,
};

export function AppLayout() {
  const activePage = useAtomValue(activePageAtom);
  const PageComponent = pages[activePage] ?? DashboardPage;

  return (
    <div className="app-background h-screen flex overflow-hidden" data-theme="dark">
      {/* Custom title bar */}
      <div
        className="fixed top-0 left-0 right-0 h-8 z-50 flex items-center justify-between px-3"
        data-tauri-drag-region
      >
        <span className="text-xs text-[--color-text-tertiary] font-medium select-none pointer-events-none">
          Vortex
        </span>
        <div className="flex items-center gap-1">
          <TitleBarButton>
            <svg width="10" height="1" viewBox="0 0 10 1" fill="currentColor"><rect width="10" height="1" /></svg>
          </TitleBarButton>
          <TitleBarButton>
            <svg width="9" height="9" viewBox="0 0 9 9" fill="none" stroke="currentColor" strokeWidth="1"><rect x="0.5" y="0.5" width="8" height="8" /></svg>
          </TitleBarButton>
          <TitleBarButton danger>
            <svg width="10" height="10" viewBox="0 0 10 10" fill="none" stroke="currentColor" strokeWidth="1.2"><line x1="1" y1="1" x2="9" y2="9" /><line x1="9" y1="1" x2="1" y2="9" /></svg>
          </TitleBarButton>
        </div>
      </div>

      <Sidebar />

      <main className="flex-1 pt-8 overflow-y-auto">
        <div className="p-6 animate-fade-in" key={activePage}>
          <PageComponent />
        </div>
      </main>
    </div>
  );
}

function TitleBarButton({ children, danger }: { children: React.ReactNode; danger?: boolean }) {
  return (
    <button
      className={`w-7 h-7 flex items-center justify-center rounded-md text-[--color-text-tertiary] transition-colors ${
        danger
          ? "hover:text-white hover:bg-danger"
          : "hover:text-[--color-text-secondary] hover:bg-white/[0.06]"
      }`}
    >
      {children}
    </button>
  );
}
