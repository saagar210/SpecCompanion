import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import type { TestProgress } from "../../lib/types";

export function ExecutionProgress() {
  const [progress, setProgress] = useState<TestProgress | null>(null);

  useEffect(() => {
    const unlisten = listen<TestProgress>("test-progress", (event) => {
      setProgress(event.payload);
    });
    return () => {
      unlisten.then((fn) => fn()).catch(() => {});
    };
  }, []);

  if (!progress || progress.status === "completed") return null;

  const percent = progress.total > 0 ? (progress.completed / progress.total) * 100 : 0;

  return (
    <div className="rounded-lg border border-border bg-surface-alt p-4">
      <div className="flex items-center justify-between mb-2">
        <span className="text-sm text-text">
          Running tests... ({progress.completed}/{progress.total})
        </span>
        <span className="text-xs text-text-muted">{percent.toFixed(0)}%</span>
      </div>
      <div className="w-full bg-surface rounded-full h-2">
        <div
          className="bg-primary h-2 rounded-full transition-all duration-300"
          style={{ width: `${percent}%` }}
        />
      </div>
    </div>
  );
}
