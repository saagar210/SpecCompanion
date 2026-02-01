import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import * as api from "../lib/api";

export function useTestResults(projectId: string | undefined) {
  return useQuery({
    queryKey: ["test-results", projectId],
    queryFn: () => api.getTestResults(projectId!),
    enabled: !!projectId,
  });
}

export function useTestResult(id: string | undefined) {
  return useQuery({
    queryKey: ["test-result", id],
    queryFn: () => api.getTestResult(id!),
    enabled: !!id,
  });
}

export function useExecuteTests(projectId: string) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (testIds: string[]) => api.executeTests(projectId, testIds),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["test-results", projectId] });
    },
  });
}
