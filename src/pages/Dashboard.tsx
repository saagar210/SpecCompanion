import { useState } from "react";
import { useProjects } from "../hooks/useProjects";
import { ProjectList } from "../components/project/ProjectList";
import { CreateProjectDialog } from "../components/project/CreateProjectDialog";

export function Dashboard() {
  const [showCreate, setShowCreate] = useState(false);
  const { data: projects, isLoading, error } = useProjects();

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h2 className="text-2xl font-bold">Dashboard</h2>
        <button
          onClick={() => setShowCreate(true)}
          className="px-4 py-2 bg-primary hover:bg-primary-dark text-white text-sm rounded-lg transition-colors"
        >
          New Project
        </button>
      </div>

      {isLoading && <p className="text-text-muted">Loading projects...</p>}
      {error && (
        <div className="rounded-lg bg-danger/10 border border-danger/30 p-3 text-sm text-danger">
          {error instanceof Error ? error.message : "Failed to load projects"}
        </div>
      )}
      {projects && <ProjectList projects={projects} />}
      {showCreate && <CreateProjectDialog onClose={() => setShowCreate(false)} />}
    </div>
  );
}
