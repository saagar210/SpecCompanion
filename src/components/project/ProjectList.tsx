import type { ProjectWithStats } from "../../lib/types";
import { ProjectCard } from "./ProjectCard";

interface Props {
  projects: ProjectWithStats[];
}

export function ProjectList({ projects }: Props) {
  if (projects.length === 0) {
    return (
      <div className="rounded-xl border border-border bg-surface-alt p-8 text-center">
        <p className="text-text-muted">
          No projects yet. Create one to get started.
        </p>
      </div>
    );
  }

  return (
    <div className="grid gap-4 grid-cols-1 md:grid-cols-2 lg:grid-cols-3">
      {projects.map((project) => (
        <ProjectCard key={project.id} project={project} />
      ))}
    </div>
  );
}
