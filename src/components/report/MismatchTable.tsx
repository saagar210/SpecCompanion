import type { Mismatch } from "../../lib/types";

interface Props {
  mismatches: Mismatch[];
}

const typeBadge: Record<string, { bg: string; label: string }> = {
  no_test_generated: { bg: "bg-warning/20 text-warning", label: "No Test" },
  test_failing: { bg: "bg-danger/20 text-danger", label: "Failing" },
  not_implemented: { bg: "bg-primary/20 text-primary-light", label: "Not Run" },
  partial_coverage: { bg: "bg-warning/20 text-warning", label: "Partial" },
};

export function MismatchTable({ mismatches }: Props) {
  if (mismatches.length === 0) {
    return (
      <div className="rounded-lg border border-success/30 bg-success/5 p-4 text-sm text-success">
        All requirements have passing tests.
      </div>
    );
  }

  return (
    <div className="border border-border rounded-lg overflow-hidden">
      <table className="w-full text-sm">
        <thead>
          <tr className="bg-surface-alt border-b border-border">
            <th className="text-left px-4 py-2 text-text-muted font-medium">Section</th>
            <th className="text-left px-4 py-2 text-text-muted font-medium">Type</th>
            <th className="text-left px-4 py-2 text-text-muted font-medium">Details</th>
          </tr>
        </thead>
        <tbody>
          {mismatches.map((m) => {
            const badge = typeBadge[m.mismatch_type] ?? { bg: "bg-border text-text-muted", label: m.mismatch_type };
            return (
              <tr key={m.id} className="border-b border-border">
                <td className="px-4 py-2 text-text-muted">{m.spec_section}</td>
                <td className="px-4 py-2">
                  <span className={`text-xs px-2 py-0.5 rounded ${badge.bg}`}>{badge.label}</span>
                </td>
                <td className="px-4 py-2 text-text">{m.details}</td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}
