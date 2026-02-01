import { useState } from "react";
import { useParams, Link } from "react-router-dom";
import { useProject } from "../hooks/useProjects";
import { useTestResults, useExecuteTests } from "../hooks/useTestExecution";
import { TestResultsTable } from "../components/test/TestResultsTable";
import { ExecutionProgress } from "../components/test/ExecutionProgress";
import { getAllGeneratedTests } from "../lib/api";
import { useQuery } from "@tanstack/react-query";

export function TestExecution() {
  const { projectId } = useParams<{ projectId: string }>();
  const { data: project } = useProject(projectId);
  const { data: results, isError: resultsError } = useTestResults(projectId);
  const executeTests = useExecuteTests(projectId ?? "");

  const { data: allTests, isLoading: testsLoading, isError: testsError } = useQuery({
    queryKey: ["all-generated-tests", projectId],
    queryFn: () => getAllGeneratedTests(projectId!),
    enabled: !!projectId,
  });

  const [selectedTests, setSelectedTests] = useState<Set<string>>(new Set());

  const toggleTest = (id: string) => {
    setSelectedTests((prev) => {
      const next = new Set(prev);
      if (next.has(id)) next.delete(id);
      else next.add(id);
      return next;
    });
  };

  const selectAll = () => {
    if (!allTests) return;
    if (selectedTests.size === allTests.length) {
      setSelectedTests(new Set());
    } else {
      setSelectedTests(new Set(allTests.map((t) => t.id)));
    }
  };

  const handleExecute = () => {
    const ids = Array.from(selectedTests);
    executeTests.mutate(ids);
  };

  return (
    <div>
      <div className="flex items-center gap-2 text-sm text-text-muted mb-1">
        <Link to="/" className="hover:text-text transition-colors">Dashboard</Link>
        <span>/</span>
        <Link to={`/project/${projectId}`} className="hover:text-text transition-colors">
          {project?.name ?? "Project"}
        </Link>
        <span>/</span>
        <span className="text-text">Execute Tests</span>
      </div>
      <h2 className="text-2xl font-bold mb-6">Test Execution</h2>

      <ExecutionProgress />

      {(testsError || resultsError || executeTests.isError) && (
        <div className="rounded-lg border border-danger/30 bg-danger/5 p-4 text-sm text-danger mb-4">
          {executeTests.isError
            ? `Execution failed: ${String(executeTests.error)}`
            : "Failed to load test data. Please try again."}
        </div>
      )}

      {testsLoading && (
        <p className="text-text-muted mb-4">Loading generated tests...</p>
      )}

      {/* Test selection */}
      {allTests && allTests.length > 0 && (
        <div className="mb-6">
          <div className="flex items-center justify-between mb-3">
            <h3 className="text-lg font-semibold">Generated Tests ({allTests.length})</h3>
            <div className="flex gap-2">
              <button
                onClick={selectAll}
                className="text-sm text-primary-light hover:text-primary transition-colors"
              >
                {selectedTests.size === allTests.length ? "Deselect All" : "Select All"}
              </button>
              <button
                onClick={handleExecute}
                disabled={selectedTests.size === 0 || executeTests.isPending}
                className="px-4 py-2 bg-primary hover:bg-primary-dark text-white text-sm rounded-lg transition-colors disabled:opacity-50"
              >
                {executeTests.isPending ? "Running..." : `Run (${selectedTests.size})`}
              </button>
            </div>
          </div>
          <div className="space-y-1 max-h-64 overflow-y-auto">
            {allTests.map((test) => (
              <div
                key={test.id}
                className={`flex items-center gap-3 p-3 rounded-lg border cursor-pointer transition-colors ${
                  selectedTests.has(test.id)
                    ? "border-primary bg-primary/5"
                    : "border-border bg-surface-alt hover:bg-surface-hover"
                }`}
                onClick={() => toggleTest(test.id)}
              >
                <input
                  type="checkbox"
                  checked={selectedTests.has(test.id)}
                  readOnly
                  className="accent-primary"
                />
                <span className="text-sm text-text truncate flex-1">
                  {test.id.slice(0, 8)} — {test.framework} ({test.generation_mode})
                </span>
                <span className="text-xs text-text-muted">
                  {new Date(test.created_at).toLocaleDateString()}
                </span>
              </div>
            ))}
          </div>
        </div>
      )}

      {allTests && allTests.length === 0 && (
        <div className="rounded-xl border border-border bg-surface-alt p-8 text-center mb-6">
          <p className="text-text-muted">No tests generated yet.</p>
          <Link
            to={`/project/${projectId}/generate`}
            className="text-primary-light text-sm hover:underline mt-2 inline-block"
          >
            Generate tests first
          </Link>
        </div>
      )}

      {/* Results */}
      {results && results.length > 0 && (
        <div>
          <h3 className="text-lg font-semibold mb-3">Results</h3>
          <TestResultsTable results={results} />
        </div>
      )}
    </div>
  );
}
