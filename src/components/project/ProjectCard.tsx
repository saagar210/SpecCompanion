import { Link } from "react-router-dom";
import type { ProjectWithStats } from "../../lib/types";

interface Props {
  project: ProjectWithStats;
}

export function ProjectCard({ project }: Props) {
  const coverage = project.coverage_percent;

  return (
    <Link
      to={`/project/${project.id}`}
      className="block rounded-xl border border-border bg-surface-alt p-5 hover:bg-surface-hover transition-colors"
    >
      <div className="flex items-start justify-between">
        <div>
          <h3 className="font-semibold text-text">{project.name}</h3>
          <p className="text-xs text-text-muted mt-1 truncate max-w-xs">
            {project.codebase_path}
          </p>
        </div>
        {coverage != null && (
          <div
            className={`text-sm font-medium px-2 py-0.5 rounded ${
              coverage >= 80
                ? "bg-success/20 text-success"
                : coverage >= 50
                ? "bg-warning/20 text-warning"
                : "bg-danger/20 text-danger"
            }`}
          >
            {coverage.toFixed(0)}%
          </div>
        )}
      </div>
      <div className="flex gap-4 mt-3 text-xs text-text-muted">
        <span>{project.spec_count} spec{project.spec_count !== 1 ? "s" : ""}</span>
        {project.last_run_at && (
          <span>Last run: {new Date(project.last_run_at).toLocaleDateString()}</span>
        )}
      </div>
    </Link>
  );
}
