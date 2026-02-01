import type { Requirement } from "../../lib/types";

interface Props {
  requirements: Requirement[];
  selectable?: boolean;
  selected?: Set<string>;
  onToggle?: (id: string) => void;
}

const typeBadgeColors: Record<string, string> = {
  functional: "bg-primary/20 text-primary-light",
  non_functional: "bg-warning/20 text-warning",
  constraint: "bg-danger/20 text-danger",
};

const priorityBadgeColors: Record<string, string> = {
  high: "text-danger",
  medium: "text-warning",
  low: "text-text-muted",
};

export function RequirementsList({ requirements, selectable, selected, onToggle }: Props) {
  // Group by section
  const grouped = requirements.reduce<Record<string, Requirement[]>>((acc, req) => {
    if (!acc[req.section]) acc[req.section] = [];
    acc[req.section].push(req);
    return acc;
  }, {});

  if (requirements.length === 0) {
    return <p className="text-text-muted text-sm">No requirements found.</p>;
  }

  return (
    <div className="space-y-4">
      {Object.entries(grouped).map(([section, reqs]) => (
        <div key={section}>
          <h4 className="text-sm font-medium text-text-muted mb-2">{section}</h4>
          <div className="space-y-1">
            {reqs.map((req) => (
              <div
                key={req.id}
                className={`flex items-start gap-3 p-3 rounded-lg border border-border bg-surface ${
                  selectable ? "cursor-pointer hover:bg-surface-hover" : ""
                } ${selected?.has(req.id) ? "border-primary bg-primary/5" : ""}`}
                onClick={() => selectable && onToggle?.(req.id)}
              >
                {selectable && (
                  <input
                    type="checkbox"
                    checked={selected?.has(req.id) ?? false}
                    onChange={() => onToggle?.(req.id)}
                    className="mt-1 accent-primary"
                  />
                )}
                <div className="flex-1 min-w-0">
                  <p className="text-sm text-text">{req.description}</p>
                  <div className="flex gap-2 mt-1">
                    <span className={`text-xs px-1.5 py-0.5 rounded ${typeBadgeColors[req.req_type] ?? "bg-border text-text-muted"}`}>
                      {req.req_type.replace("_", "-")}
                    </span>
                    <span className={`text-xs ${priorityBadgeColors[req.priority] ?? "text-text-muted"}`}>
                      {req.priority}
                    </span>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      ))}
    </div>
  );
}
