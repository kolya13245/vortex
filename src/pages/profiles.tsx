import { Plus, RefreshCw, Trash2, Copy, X } from "lucide-react";
import { useState } from "react";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { getSubscriptions, addSubscription, removeSubscription, updateSubscription } from "@/lib/tauri";
import { formatBytes } from "@/lib/utils";
import type { Subscription } from "@/types/core";

export function ProfilesPage() {
  const [showAdd, setShowAdd] = useState(false);
  const queryClient = useQueryClient();

  const { data: subs = [] } = useQuery({
    queryKey: ["subscriptions"],
    queryFn: getSubscriptions,
  });

  const removeMut = useMutation({
    mutationFn: (id: string) => removeSubscription(id),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["subscriptions"] }),
  });

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-semibold text-[--color-text-primary]">Profiles</h1>
        <button
          onClick={() => setShowAdd(true)}
          className="bg-accent hover:bg-accent-hover text-[--color-text-inverse] px-4 py-2 rounded-lg text-sm font-medium transition-colors flex items-center gap-2"
        >
          <Plus className="w-4 h-4" />
          Add Subscription
        </button>
      </div>

      {subs.length === 0 ? (
        <div className="flex flex-col items-center justify-center py-20">
          <div className="w-16 h-16 rounded-full bg-white/[0.05] flex items-center justify-center mb-4">
            <Plus className="w-6 h-6 text-[--color-text-tertiary]" />
          </div>
          <p className="text-[--color-text-secondary] text-sm mb-1">No subscriptions yet</p>
          <p className="text-[--color-text-tertiary] text-xs">
            Add a subscription to get started
          </p>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
          {subs.map((sub) => (
            <SubscriptionCard
              key={sub.id}
              sub={sub}
              onRemove={() => removeMut.mutate(sub.id)}
              onRefresh={() => {
                updateSubscription(sub.id).then(() =>
                  queryClient.invalidateQueries({ queryKey: ["subscriptions"] })
                );
              }}
            />
          ))}
        </div>
      )}

      {showAdd && (
        <AddSubscriptionModal
          onClose={() => setShowAdd(false)}
          onAdded={() => {
            queryClient.invalidateQueries({ queryKey: ["subscriptions"] });
            setShowAdd(false);
          }}
        />
      )}
    </div>
  );
}

function SubscriptionCard({ sub, onRemove, onRefresh }: { sub: Subscription; onRemove: () => void; onRefresh: () => void }) {
  return (
    <div className="glass rounded-xl p-4">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          <h3 className="text-sm font-medium text-[--color-text-primary]">{sub.name}</h3>
          <span className="px-1.5 py-0.5 rounded text-xs bg-accent/10 text-accent font-mono">
            {sub.core_type === "mihomo" ? "Mihomo" : "Xray"}
          </span>
        </div>
        <div className="flex items-center gap-1">
          <button onClick={onRefresh} className="p-1.5 rounded-md text-[--color-text-tertiary] hover:text-[--color-text-primary] hover:bg-white/[0.06] transition-colors">
            <RefreshCw className="w-3.5 h-3.5" />
          </button>
          <button className="p-1.5 rounded-md text-[--color-text-tertiary] hover:text-[--color-text-primary] hover:bg-white/[0.06] transition-colors">
            <Copy className="w-3.5 h-3.5" />
          </button>
          <button
            onClick={onRemove}
            className="p-1.5 rounded-md text-[--color-text-tertiary] hover:text-danger hover:bg-danger/10 transition-colors"
          >
            <Trash2 className="w-3.5 h-3.5" />
          </button>
        </div>
      </div>

      <p className="mt-1 text-xs font-mono text-[--color-text-tertiary] truncate">{sub.url}</p>

      <div className="flex items-center gap-4 mt-3 text-xs text-[--color-text-secondary]">
        <span>{sub.node_count} nodes</span>
        {sub.last_updated && <span>Updated {sub.last_updated}</span>}
      </div>

      {sub.traffic_total != null && sub.traffic_used != null && (
        <div className="mt-3">
          <div className="h-1.5 rounded-full bg-white/[0.06] overflow-hidden">
            <div
              className="h-full rounded-full bg-accent transition-all"
              style={{
                width: `${Math.min(100, (sub.traffic_used / sub.traffic_total) * 100)}%`,
              }}
            />
          </div>
          <p className="mt-1 text-xs font-mono text-[--color-text-secondary]">
            {formatBytes(sub.traffic_used)} / {formatBytes(sub.traffic_total)}
          </p>
        </div>
      )}

      {sub.expire && (
        <p className="mt-1 text-xs text-[--color-text-tertiary]">Expires: {sub.expire}</p>
      )}
    </div>
  );
}

function AddSubscriptionModal({
  onClose,
  onAdded,
}: {
  onClose: () => void;
  onAdded: () => void;
}) {
  const [name, setName] = useState("");
  const [url, setUrl] = useState("");

  const addMut = useMutation({
    mutationFn: () => addSubscription(name, url),
    onSuccess: onAdded,
  });

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/40 backdrop-blur-sm">
      <div className="glass-elevated rounded-2xl p-6 w-full max-w-md shadow-[0_16px_64px_rgba(0,0,0,0.40)] animate-modal-in">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-lg font-semibold text-[--color-text-primary]">Add Subscription</h2>
          <button
            onClick={onClose}
            className="p-1 rounded-md text-[--color-text-tertiary] hover:text-[--color-text-primary] hover:bg-white/[0.06]"
          >
            <X className="w-4 h-4" />
          </button>
        </div>

        <div className="space-y-4">
          <div>
            <label className="text-xs text-[--color-text-secondary] mb-1 block">Name</label>
            <input
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="My Subscription"
              className="w-full bg-white/[0.05] border border-white/[0.10] rounded-lg px-3 py-2 text-sm text-[--color-text-primary] placeholder:text-[--color-text-tertiary] focus:border-accent/50 focus:ring-1 focus:ring-accent/20 focus:outline-none transition-colors"
            />
          </div>
          <div>
            <label className="text-xs text-[--color-text-secondary] mb-1 block">URL</label>
            <input
              value={url}
              onChange={(e) => setUrl(e.target.value)}
              placeholder="https://example.com/sub"
              className="w-full bg-white/[0.05] border border-white/[0.10] rounded-lg px-3 py-2 text-sm text-[--color-text-primary] placeholder:text-[--color-text-tertiary] focus:border-accent/50 focus:ring-1 focus:ring-accent/20 focus:outline-none transition-colors"
            />
          </div>
        </div>

        <div className="flex justify-end gap-2 mt-6">
          <button
            onClick={onClose}
            className="glass rounded-lg px-4 py-2 text-sm text-[--color-text-secondary] hover:text-[--color-text-primary] transition-colors"
          >
            Cancel
          </button>
          <button
            onClick={() => addMut.mutate()}
            disabled={!name || !url || addMut.isPending}
            className="bg-accent hover:bg-accent-hover text-[--color-text-inverse] px-4 py-2 rounded-lg text-sm font-medium transition-colors disabled:opacity-40 disabled:cursor-not-allowed"
          >
            {addMut.isPending ? "Fetching..." : "Add"}
          </button>
          {addMut.isError && (
            <p className="text-xs text-danger mt-2">{String(addMut.error)}</p>
          )}
        </div>
      </div>
    </div>
  );
}
