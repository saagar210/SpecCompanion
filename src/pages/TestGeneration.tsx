import { useState, useEffect } from "react";
import { useParams, Link } from "react-router-dom";
import { useProject } from "../hooks/useProjects";
import { useSpecs, useSpec } from "../hooks/useSpecs";
import { TestGenerationPanel } from "../components/test/TestGenerationPanel";

export function TestGeneration() {
  const { projectId } = useParams<{ projectId: string }>();
  const { data: project } = useProject(projectId);
  const { data: specs, isError: specsError } = useSpecs(projectId);
  const [selectedSpecId, setSelectedSpecId] = useState<string | undefined>();
  const { data: parsed } = useSpec(selectedSpecId);

  // Auto-select first spec when specs load
  useEffect(() => {
    if (!selectedSpecId && specs && specs.length > 0) {
      setSelectedSpecId(specs[0].id);
    }
  }, [specs, selectedSpecId]);

  return (
    <div>
      <div className="flex items-center gap-2 text-sm text-text-muted mb-1">
        <Link to="/" className="hover:text-text transition-colors">Dashboard</Link>
        <span>/</span>
        <Link to={`/project/${projectId}`} className="hover:text-text transition-colors">
          {project?.name ?? "Project"}
        </Link>
        <span>/</span>
        <span className="text-text">Generate Tests</span>
      </div>
      <h2 className="text-2xl font-bold mb-6">Test Generation</h2>

      {specs && specs.length > 1 && (
        <div className="mb-4">
          <label className="text-sm text-text-muted mr-2">Spec:</label>
          <select
            value={selectedSpecId ?? ""}
            onChange={(e) => setSelectedSpecId(e.target.value)}
            className="bg-surface border border-border rounded-lg px-3 py-1.5 text-sm text-text"
          >
            {specs.map((s) => (
              <option key={s.id} value={s.id}>{s.filename}</option>
            ))}
          </select>
        </div>
      )}

      {specsError && (
        <div className="rounded-lg border border-danger/30 bg-danger/5 p-4 text-sm text-danger mb-4">
          Failed to load specifications.
        </div>
      )}

      {!specs || specs.length === 0 ? (
        <div className="rounded-xl border border-border bg-surface-alt p-8 text-center">
          <p className="text-text-muted">Upload a spec first to generate tests.</p>
          <Link
            to={`/project/${projectId}`}
            className="text-primary-light text-sm hover:underline mt-2 inline-block"
          >
            Go to project
          </Link>
        </div>
      ) : parsed ? (
        <TestGenerationPanel
          projectId={projectId!}
          requirements={parsed.requirements}
        />
      ) : (
        <p className="text-text-muted">Loading requirements...</p>
      )}
    </div>
  );
}
