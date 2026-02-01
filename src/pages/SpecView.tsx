import { useState } from "react";
import { useParams, Link } from "react-router-dom";
import { useSpec, useReparseSpec } from "../hooks/useSpecs";
import { useProject } from "../hooks/useProjects";
import { SpecViewer } from "../components/spec/SpecViewer";
import { RequirementsList } from "../components/spec/RequirementsList";

export function SpecView() {
  const { projectId, specId } = useParams<{ projectId: string; specId: string }>();
  const { data: project } = useProject(projectId);
  const { data: parsed, isLoading } = useSpec(specId);
  const reparse = useReparseSpec();
  const [tab, setTab] = useState<"requirements" | "source">("requirements");

  if (isLoading) return <p className="text-text-muted">Loading...</p>;
  if (!parsed) return <p className="text-text-muted">Spec not found.</p>;

  return (
    <div>
      {/* Breadcrumb */}
      <div className="flex items-center gap-2 text-sm text-text-muted mb-1">
        <Link to="/" className="hover:text-text transition-colors">Dashboard</Link>
        <span>/</span>
        <Link to={`/project/${projectId}`} className="hover:text-text transition-colors">
          {project?.name ?? "Project"}
        </Link>
        <span>/</span>
        <span className="text-text">{parsed.spec.filename}</span>
      </div>

      <div className="flex items-center justify-between mb-6">
        <h2 className="text-2xl font-bold">{parsed.spec.filename}</h2>
        <button
          onClick={() => reparse.mutate(parsed.spec.id)}
          disabled={reparse.isPending}
          className="px-4 py-2 bg-surface border border-border text-text text-sm rounded-lg hover:bg-surface-hover transition-colors disabled:opacity-50"
        >
          {reparse.isPending ? "Re-parsing..." : "Re-parse"}
        </button>
      </div>

      {/* Stats */}
      <div className="flex gap-4 mb-4 text-sm text-text-muted">
        <span>{parsed.requirements.length} requirements extracted</span>
        {parsed.spec.parsed_at && (
          <span>Parsed: {new Date(parsed.spec.parsed_at).toLocaleString()}</span>
        )}
      </div>

      {/* Tabs */}
      <div className="flex gap-1 mb-4 border-b border-border">
        <button
          onClick={() => setTab("requirements")}
          className={`px-4 py-2 text-sm transition-colors border-b-2 ${
            tab === "requirements"
              ? "border-primary text-primary-light"
              : "border-transparent text-text-muted hover:text-text"
          }`}
        >
          Requirements ({parsed.requirements.length})
        </button>
        <button
          onClick={() => setTab("source")}
          className={`px-4 py-2 text-sm transition-colors border-b-2 ${
            tab === "source"
              ? "border-primary text-primary-light"
              : "border-transparent text-text-muted hover:text-text"
          }`}
        >
          Source
        </button>
      </div>

      {tab === "requirements" ? (
        <RequirementsList requirements={parsed.requirements} />
      ) : (
        <div className="rounded-xl border border-border bg-surface-alt p-6">
          <SpecViewer content={parsed.spec.content} />
        </div>
      )}
    </div>
  );
}
