import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import * as api from "../lib/api";
import type { GenerateTestsRequest, AppSettings } from "../lib/types";

export function useGeneratedTests(requirementId: string | undefined) {
  return useQuery({
    queryKey: ["generated-tests", requirementId],
    queryFn: () => api.getGeneratedTests(requirementId!),
    enabled: !!requirementId,
  });
}

export function useGenerateTests() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (req: GenerateTestsRequest) => api.generateTests(req),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["generated-tests"] });
    },
  });
}

export function useSaveTestToDisk() {
  return useMutation({
    mutationFn: ({ testId, path }: { testId: string; path: string }) =>
      api.saveTestToDisk(testId, path),
  });
}

export function useSettings() {
  return useQuery({
    queryKey: ["settings"],
    queryFn: api.loadSettings,
  });
}

export function useSaveSettings() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (settings: AppSettings) => api.saveSettings(settings),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["settings"] });
    },
  });
}
