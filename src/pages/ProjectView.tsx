import { useState } from "react";
import { useParams, Link, useNavigate } from "react-router-dom";
import { useProject, useDeleteProject } from "../hooks/useProjects";
import { useSpecs, useDeleteSpec } from "../hooks/useSpecs";
import { SpecUploader } from "../components/spec/SpecUploader";

export function ProjectView() {
  const { projectId } = useParams<{ projectId: string }>();
  const navigate = useNavigate();
  const { data: project, isLoading } = useProject(projectId);
  const { data: specs, isError: specsError } = useSpecs(projectId);
  const deleteProject = useDeleteProject();
  const deleteSpec = useDeleteSpec(projectId ?? "");
  const [confirmDelete, setConfirmDelete] = useState(false);
  const [confirmDeleteSpecId, setConfirmDeleteSpecId] = useState<string | null>(null);

  if (isLoading) return <p className="text-text-muted">Loading...</p>;
  if (!project) return <p className="text-text-muted">Project not found.</p>;

  const handleDeleteProject = () => {
    deleteProject.mutate(project.id, {
      onSuccess: () => navigate("/"),
    });
  };

  return (
    <div>
      {/* Header */}
      <div className="flex items-start justify-between mb-6">
        <div>
          <div className="flex items-center gap-2 text-sm text-text-muted mb-1">
            <Link to="/" className="hover:text-text transition-colors">Dashboard</Link>
            <span>/</span>
            <span className="text-text">{project.name}</span>
          </div>
          <h2 className="text-2xl font-bold">{project.name}</h2>
          <p className="text-sm text-text-muted mt-1">{project.codebase_path}</p>
        </div>
        <div className="flex gap-2">
          <Link
            to={`/project/${projectId}/generate`}
            className="px-4 py-2 bg-primary hover:bg-primary-dark text-white text-sm rounded-lg transition-colors"
          >
            Generate Tests
          </Link>
          <Link
            to={`/project/${projectId}/execute`}
            className="px-4 py-2 bg-surface border border-border text-text text-sm rounded-lg hover:bg-surface-hover transition-colors"
          >
            Run Tests
          </Link>
          <Link
            to={`/project/${projectId}/reports`}
            className="px-4 py-2 bg-surface border border-border text-text text-sm rounded-lg hover:bg-surface-hover transition-colors"
          >
            Reports
          </Link>
          {confirmDelete ? (
            <div className="flex items-center gap-2">
              <span className="text-xs text-danger">Are you sure?</span>
              <button
                onClick={handleDeleteProject}
                disabled={deleteProject.isPending}
                className="px-3 py-1.5 bg-danger text-white text-xs rounded-lg hover:bg-danger/80 transition-colors disabled:opacity-50"
              >
                {deleteProject.isPending ? "Deleting..." : "Yes, delete"}
              </button>
              <button
                onClick={() => setConfirmDelete(false)}
                className="px-3 py-1.5 text-xs text-text-muted hover:text-text transition-colors"
              >
                Cancel
              </button>
            </div>
          ) : (
            <button
              onClick={() => setConfirmDelete(true)}
              className="px-3 py-2 text-danger text-sm hover:bg-danger/10 rounded-lg transition-colors"
            >
              Delete
            </button>
          )}
        </div>
      </div>

      {/* Stats */}
      <div className="grid grid-cols-3 gap-4 mb-6">
        <div className="rounded-xl border border-border bg-surface-alt p-4">
          <p className="text-xs text-text-muted">Specs</p>
          <p className="text-2xl font-bold mt-1">{project.spec_count}</p>
        </div>
        <div className="rounded-xl border border-border bg-surface-alt p-4">
          <p className="text-xs text-text-muted">Coverage</p>
          <p className="text-2xl font-bold mt-1">
            {project.coverage_percent != null ? `${project.coverage_percent.toFixed(0)}%` : "--"}
          </p>
        </div>
        <div className="rounded-xl border border-border bg-surface-alt p-4">
          <p className="text-xs text-text-muted">Last Run</p>
          <p className="text-sm font-medium mt-2">
            {project.last_run_at ? new Date(project.last_run_at).toLocaleDateString() : "Never"}
          </p>
        </div>
      </div>

      {/* Specs */}
      {(specsError || deleteSpec.isError) && (
        <div className="rounded-lg border border-danger/30 bg-danger/5 p-4 text-sm text-danger mb-4">
          {deleteSpec.isError ? "Failed to delete spec." : "Failed to load specifications."}
        </div>
      )}

      <div className="flex items-center justify-between mb-4">
        <h3 className="text-lg font-semibold">Specifications</h3>
        <SpecUploader projectId={projectId!} />
      </div>

      {specs && specs.length > 0 ? (
        <div className="space-y-2">
          {specs.map((spec) => (
            <div
              key={spec.id}
              className="flex items-center justify-between p-4 rounded-lg border border-border bg-surface-alt"
            >
              <Link
                to={`/project/${projectId}/spec/${spec.id}`}
                className="text-sm font-medium text-text hover:text-primary-light transition-colors"
              >
                {spec.filename}
              </Link>
              <div className="flex items-center gap-3">
                <span className="text-xs text-text-muted">
                  {spec.parsed_at ? "Parsed" : "Not parsed"}
                </span>
                {confirmDeleteSpecId === spec.id ? (
                  <div className="flex items-center gap-2">
                    <span className="text-xs text-danger">Delete?</span>
                    <button
                      onClick={() => { deleteSpec.mutate(spec.id); setConfirmDeleteSpecId(null); }}
                      className="text-xs text-danger font-medium hover:text-danger/80 transition-colors"
                    >
                      Yes
                    </button>
                    <button
                      onClick={() => setConfirmDeleteSpecId(null)}
                      className="text-xs text-text-muted hover:text-text transition-colors"
                    >
                      No
                    </button>
                  </div>
                ) : (
                  <button
                    onClick={() => setConfirmDeleteSpecId(spec.id)}
                    className="text-xs text-danger hover:text-danger/80 transition-colors"
                  >
                    Delete
                  </button>
                )}
              </div>
            </div>
          ))}
        </div>
      ) : (
        <div className="rounded-xl border border-border bg-surface-alt p-8 text-center">
          <p className="text-text-muted">No specs uploaded yet.</p>
        </div>
      )}
    </div>
  );
}
