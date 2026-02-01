import { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { useCreateProject } from "../../hooks/useProjects";

interface Props {
  onClose: () => void;
}

export function CreateProjectDialog({ onClose }: Props) {
  const [name, setName] = useState("");
  const [codebasePath, setCodebasePath] = useState("");
  const createProject = useCreateProject();

  const handleSelectFolder = async () => {
    const selected = await open({ directory: true, multiple: false });
    if (selected && typeof selected === "string") {
      setCodebasePath(selected);
    }
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!name.trim() || !codebasePath.trim()) return;
    createProject.mutate(
      { name: name.trim(), codebase_path: codebasePath.trim() },
      { onSuccess: onClose }
    );
  };

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50" onClick={onClose}>
      <div className="bg-surface-alt border border-border rounded-xl p-6 w-full max-w-md" onClick={(e) => e.stopPropagation()}>
        <h3 className="text-lg font-semibold mb-4">New Project</h3>
        {createProject.isError && (
          <div className="rounded-lg border border-danger/30 bg-danger/5 p-3 text-sm text-danger mb-4">
            {createProject.error instanceof Error ? createProject.error.message : "Failed to create project."}
          </div>
        )}
        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label className="block text-sm text-text-muted mb-1">Project Name</label>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="My Project"
              className="w-full bg-surface border border-border rounded-lg px-3 py-2 text-sm text-text focus:outline-none focus:border-primary"
              autoFocus
            />
          </div>
          <div>
            <label className="block text-sm text-text-muted mb-1">Codebase Path</label>
            <div className="flex gap-2">
              <input
                type="text"
                value={codebasePath}
                onChange={(e) => setCodebasePath(e.target.value)}
                placeholder="/path/to/project"
                className="flex-1 bg-surface border border-border rounded-lg px-3 py-2 text-sm text-text focus:outline-none focus:border-primary"
              />
              <button
                type="button"
                onClick={handleSelectFolder}
                className="px-3 py-2 bg-surface border border-border rounded-lg text-sm text-text-muted hover:bg-surface-hover transition-colors"
              >
                Browse
              </button>
            </div>
          </div>
          <div className="flex justify-end gap-2 pt-2">
            <button
              type="button"
              onClick={onClose}
              className="px-4 py-2 text-sm text-text-muted hover:text-text transition-colors"
            >
              Cancel
            </button>
            <button
              type="submit"
              disabled={!name.trim() || !codebasePath.trim() || createProject.isPending}
              className="px-4 py-2 bg-primary hover:bg-primary-dark text-white text-sm rounded-lg transition-colors disabled:opacity-50"
            >
              {createProject.isPending ? "Creating..." : "Create"}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
