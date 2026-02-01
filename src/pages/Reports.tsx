import { useState, useEffect } from "react";
import { useParams, Link } from "react-router-dom";
import { useProject } from "../hooks/useProjects";
import {
  useReports,
  useAlignmentReport,
  useGenerateAlignmentReport,
  useExportReport,
} from "../hooks/useReports";
import { CoverageGauge } from "../components/report/CoverageGauge";
import { AlignmentChart } from "../components/report/AlignmentChart";
import { MismatchTable } from "../components/report/MismatchTable";

export function Reports() {
  const { projectId } = useParams<{ projectId: string }>();
  const { data: project } = useProject(projectId);
  const { data: reports } = useReports(projectId);
  const generateReport = useGenerateAlignmentReport(projectId ?? "");
  const exportReport = useExportReport();
  const [selectedReportId, setSelectedReportId] = useState<string | undefined>();

  const { data: report } = useAlignmentReport(selectedReportId);

  // Auto-select latest report
  useEffect(() => {
    if (!selectedReportId && reports && reports.length > 0) {
      setSelectedReportId(reports[0].id);
    }
  }, [reports, selectedReportId]);

  const handleExport = (format: "json" | "html" | "csv") => {
    if (!selectedReportId) return;
    exportReport.mutate(
      { reportId: selectedReportId, format },
      {
        onSuccess: (content) => {
          const blob = new Blob([content], {
            type: format === "json" ? "application/json" : format === "html" ? "text/html" : "text/csv",
          });
          const url = URL.createObjectURL(blob);
          const a = document.createElement("a");
          a.href = url;
          a.download = `alignment-report.${format}`;
          a.click();
          URL.revokeObjectURL(url);
        },
      }
    );
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
        <span className="text-text">Reports</span>
      </div>

      <div className="flex items-center justify-between mb-6">
        <h2 className="text-2xl font-bold">Alignment Reports</h2>
        <div className="flex gap-2">
          <button
            onClick={() => generateReport.mutate(undefined, {
              onSuccess: (data) => setSelectedReportId(data.id),
            })}
            disabled={generateReport.isPending}
            className="px-4 py-2 bg-primary hover:bg-primary-dark text-white text-sm rounded-lg transition-colors disabled:opacity-50"
          >
            {generateReport.isPending ? "Generating..." : "Generate Report"}
          </button>
        </div>
      </div>

      {/* Report selector */}
      {reports && reports.length > 1 && (
        <div className="mb-4">
          <label className="text-sm text-text-muted mr-2">Report:</label>
          <select
            value={selectedReportId ?? ""}
            onChange={(e) => setSelectedReportId(e.target.value)}
            className="bg-surface border border-border rounded-lg px-3 py-1.5 text-sm text-text"
          >
            {reports.map((r) => (
              <option key={r.id} value={r.id}>
                {new Date(r.generated_at).toLocaleString()} — {r.coverage_percent.toFixed(0)}%
              </option>
            ))}
          </select>
        </div>
      )}

      {report ? (
        <div className="space-y-6">
          {/* Coverage and chart */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div className="rounded-xl border border-border bg-surface-alt p-6">
              <h3 className="text-sm text-text-muted mb-4">Coverage</h3>
              <CoverageGauge
                coveragePercent={report.coverage_percent}
                total={report.total_requirements}
                covered={report.covered_requirements}
              />
            </div>
            <div className="rounded-xl border border-border bg-surface-alt p-6">
              <h3 className="text-sm text-text-muted mb-4">Breakdown</h3>
              <AlignmentChart
                mismatches={report.mismatches}
                totalRequirements={report.total_requirements}
                coveredRequirements={report.covered_requirements}
              />
            </div>
          </div>

          {/* Export */}
          <div className="flex gap-2">
            <span className="text-sm text-text-muted pt-1">Export:</span>
            <button onClick={() => handleExport("json")} className="text-sm text-primary-light hover:underline">JSON</button>
            <button onClick={() => handleExport("html")} className="text-sm text-primary-light hover:underline">HTML</button>
            <button onClick={() => handleExport("csv")} className="text-sm text-primary-light hover:underline">CSV</button>
          </div>

          {/* Mismatches */}
          <div>
            <h3 className="text-lg font-semibold mb-3">Mismatches ({report.mismatches.length})</h3>
            <MismatchTable mismatches={report.mismatches} />
          </div>
        </div>
      ) : reports && reports.length === 0 ? (
        <div className="rounded-xl border border-border bg-surface-alt p-8 text-center">
          <p className="text-text-muted">No reports generated yet. Generate one to see alignment analysis.</p>
        </div>
      ) : null}
    </div>
  );
}
