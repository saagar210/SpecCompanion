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

  const isError = progress.status === "error";
  const percent = progress.total > 0 ? (progress.completed / progress.total) * 100 : 0;

  return (
    <div className={`rounded-lg border p-4 mb-4 ${isError ? "border-danger/30 bg-danger/5" : "border-border bg-surface-alt"}`}>
      <div className="flex items-center justify-between mb-2">
        <span className={`text-sm ${isError ? "text-danger" : "text-text"}`}>
          {isError
            ? `Test execution error at ${progress.completed}/${progress.total}`
            : `Running tests... (${progress.completed}/${progress.total})`}
        </span>
        <span className="text-xs text-text-muted">{percent.toFixed(0)}%</span>
      </div>
      <div className="w-full bg-surface rounded-full h-2">
        <div
          className={`h-2 rounded-full transition-all duration-300 ${isError ? "bg-danger" : "bg-primary"}`}
          style={{ width: `${percent}%` }}
        />
      </div>
    </div>
  );
}
