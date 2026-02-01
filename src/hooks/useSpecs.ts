import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import * as api from "../lib/api";

export function useSpecs(projectId: string | undefined) {
  return useQuery({
    queryKey: ["specs", projectId],
    queryFn: () => api.listSpecs(projectId!),
    enabled: !!projectId,
  });
}

export function useSpec(id: string | undefined) {
  return useQuery({
    queryKey: ["spec", id],
    queryFn: () => api.getSpec(id!),
    enabled: !!id,
  });
}

export function useUploadSpec(projectId: string) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ filename, content }: { filename: string; content: string }) =>
      api.uploadSpec(projectId, filename, content),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["specs", projectId] });
      queryClient.invalidateQueries({ queryKey: ["projects"] });
    },
  });
}

export function useDeleteSpec(projectId: string) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => api.deleteSpec(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["specs", projectId] });
      queryClient.invalidateQueries({ queryKey: ["projects"] });
    },
  });
}

export function useReparseSpec() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => api.reparseSpec(id),
    onSuccess: (_data, id) => {
      queryClient.invalidateQueries({ queryKey: ["spec", id] });
    },
  });
}
