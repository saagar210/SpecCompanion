import { BrowserRouter, Routes, Route } from "react-router-dom";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { AppLayout } from "./components/layout/AppLayout";
import { ErrorBoundary } from "./components/layout/ErrorBoundary";
import { Dashboard } from "./pages/Dashboard";
import { Settings } from "./pages/Settings";
import { ProjectView } from "./pages/ProjectView";
import { SpecView } from "./pages/SpecView";
import { TestGeneration } from "./pages/TestGeneration";
import { TestExecution } from "./pages/TestExecution";
import { Reports } from "./pages/Reports";

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: 1,
      refetchOnWindowFocus: false,
    },
  },
});

function App() {
  return (
    <ErrorBoundary>
    <QueryClientProvider client={queryClient}>
      <BrowserRouter>
        <Routes>
          <Route element={<AppLayout />}>
            <Route path="/" element={<Dashboard />} />
            <Route path="/settings" element={<Settings />} />
            <Route path="/project/:projectId" element={<ProjectView />} />
            <Route path="/project/:projectId/spec/:specId" element={<SpecView />} />
            <Route path="/project/:projectId/generate" element={<TestGeneration />} />
            <Route path="/project/:projectId/execute" element={<TestExecution />} />
            <Route path="/project/:projectId/reports" element={<Reports />} />
          </Route>
        </Routes>
      </BrowserRouter>
    </QueryClientProvider>
    </ErrorBoundary>
  );
}

export default App;
