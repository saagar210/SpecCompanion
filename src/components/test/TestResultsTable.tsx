import { Fragment, useState } from "react";
import type { TestResult } from "../../lib/types";

interface Props {
  results: TestResult[];
}

const statusColors: Record<string, string> = {
  passed: "text-success",
  failed: "text-danger",
  error: "text-danger",
  skipped: "text-text-muted",
};

const statusIcons: Record<string, string> = {
  passed: "\u2713",
  failed: "\u2717",
  error: "\u26A0",
  skipped: "\u2014",
};

export function TestResultsTable({ results }: Props) {
  const [expanded, setExpanded] = useState<string | null>(null);

  if (results.length === 0) {
    return <p className="text-text-muted text-sm">No test results yet.</p>;
  }

  return (
    <div className="border border-border rounded-lg overflow-hidden">
      <table className="w-full text-sm">
        <thead>
          <tr className="bg-surface-alt border-b border-border">
            <th className="text-left px-4 py-2 text-text-muted font-medium">Status</th>
            <th className="text-left px-4 py-2 text-text-muted font-medium">Test ID</th>
            <th className="text-right px-4 py-2 text-text-muted font-medium">Time</th>
            <th className="text-left px-4 py-2 text-text-muted font-medium">Executed</th>
          </tr>
        </thead>
        <tbody>
          {results.map((result) => (
            <Fragment key={result.id}>
              <tr
                className="border-b border-border hover:bg-surface-hover cursor-pointer"
                onClick={() => setExpanded(expanded === result.id ? null : result.id)}
              >
                <td className={`px-4 py-2 font-mono ${statusColors[result.status]}`}>
                  {statusIcons[result.status]} {result.status}
                </td>
                <td className="px-4 py-2 text-text-muted font-mono text-xs">
                  {result.generated_test_id.slice(0, 8)}
                </td>
                <td className="px-4 py-2 text-right text-text-muted">
                  {result.execution_time_ms}ms
                </td>
                <td className="px-4 py-2 text-text-muted">
                  {new Date(result.executed_at).toLocaleString()}
                </td>
              </tr>
              {expanded === result.id && (
                <tr key={`${result.id}-detail`}>
                  <td colSpan={4} className="px-4 py-3 bg-surface">
                    {result.stdout && (
                      <div className="mb-2">
                        <span className="text-xs text-text-muted font-medium">stdout:</span>
                        <pre className="text-xs text-text mt-1 bg-surface-alt p-2 rounded overflow-x-auto max-h-48">
                          {result.stdout}
                        </pre>
                      </div>
                    )}
                    {result.stderr && (
                      <div>
                        <span className="text-xs text-text-muted font-medium">stderr:</span>
                        <pre className="text-xs text-danger mt-1 bg-surface-alt p-2 rounded overflow-x-auto max-h-48">
                          {result.stderr}
                        </pre>
                      </div>
                    )}
                    {!result.stdout && !result.stderr && (
                      <p className="text-xs text-text-muted">No output captured.</p>
                    )}
                  </td>
                </tr>
              )}
            </Fragment>
          ))}
        </tbody>
      </table>
    </div>
  );
}
