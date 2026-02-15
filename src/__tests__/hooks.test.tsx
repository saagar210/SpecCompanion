import { renderHook, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ReactNode } from "react";
import { useProjects, useProject } from "../hooks/useProjects";
import { useSpecs, useSpec } from "../hooks/useSpecs";

// Mock the Tauri invoke function
jest.mock("@tauri-apps/api/core", () => ({
  invoke: jest.fn(),
}));

import { invoke } from "@tauri-apps/api/core";

const mockInvoke = invoke as jest.MockedFunction<typeof invoke>;

// Create a test QueryClient wrapper
const createWrapper = () => {
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: { retry: false },
      mutations: { retry: false },
    },
  });
  return ({ children }: { children: ReactNode }) => (
    <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
  );
};

describe("useProjects", () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  test("fetches projects successfully", async () => {
    const mockProjects = [
      {
        id: "p1",
        name: "Project 1",
        codebase_path: "/path/to/code",
        created_at: "2026-02-01T00:00:00Z",
        updated_at: "2026-02-01T00:00:00Z",
        spec_count: 2,
        test_count: 10,
      },
    ];

    mockInvoke.mockResolvedValue(mockProjects);

    const { result } = renderHook(() => useProjects(), {
      wrapper: createWrapper(),
    });

    await waitFor(() => {
      expect(result.current.isSuccess).toBe(true);
    });

    expect(result.current.data).toEqual(mockProjects);
    expect(mockInvoke).toHaveBeenCalledWith("list_projects");
  });

  test("handles error when fetching fails", async () => {
    mockInvoke.mockRejectedValue(new Error("Network error"));

    const { result } = renderHook(() => useProjects(), {
      wrapper: createWrapper(),
    });

    await waitFor(() => {
      expect(result.current.isError).toBe(true);
    });

    expect(result.current.error).toBeTruthy();
  });

  test("returns empty array when no projects", async () => {
    mockInvoke.mockResolvedValue([]);

    const { result } = renderHook(() => useProjects(), {
      wrapper: createWrapper(),
    });

    await waitFor(() => {
      expect(result.current.isSuccess).toBe(true);
    });

    expect(result.current.data).toEqual([]);
  });

  test("starts in loading state", () => {
    mockInvoke.mockImplementation(() => new Promise(() => {})); // Never resolves

    const { result } = renderHook(() => useProjects(), {
      wrapper: createWrapper(),
    });

    expect(result.current.isLoading).toBe(true);
    expect(result.current.data).toBeUndefined();
  });
});

describe("useProject", () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  test("fetches single project by id", async () => {
    const mockProject = {
      id: "p1",
      name: "Project 1",
      codebase_path: "/path/to/code",
      created_at: "2026-02-01T00:00:00Z",
      updated_at: "2026-02-01T00:00:00Z",
      spec_count: 2,
      test_count: 10,
    };

    mockInvoke.mockResolvedValue(mockProject);

    const { result } = renderHook(() => useProject("p1"), {
      wrapper: createWrapper(),
    });

    await waitFor(() => {
      expect(result.current.isSuccess).toBe(true);
    });

    expect(result.current.data).toEqual(mockProject);
    expect(mockInvoke).toHaveBeenCalledWith("get_project", { id: "p1" });
  });

  test("does not fetch if id is undefined", () => {
    const { result } = renderHook(() => useProject(undefined), {
      wrapper: createWrapper(),
    });

    expect(mockInvoke).not.toHaveBeenCalled();
    expect(result.current.data).toBeUndefined();
    expect(result.current.isPending).toBe(true); // Query is disabled, so it's pending
  });

  test("handles error when project not found", async () => {
    mockInvoke.mockRejectedValue(new Error("Project not found"));

    const { result } = renderHook(() => useProject("invalid-id"), {
      wrapper: createWrapper(),
    });

    await waitFor(() => {
      expect(result.current.isError).toBe(true);
    });

    expect(result.current.error).toBeTruthy();
  });
});

describe("useSpecs", () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  test("fetches specs for a project", async () => {
    const mockSpecs = [
      {
        id: "s1",
        project_id: "p1",
        filename: "spec.md",
        content: "# Spec",
        parsed_at: "2026-02-01T00:00:00Z",
        created_at: "2026-02-01T00:00:00Z",
      },
    ];

    mockInvoke.mockResolvedValue(mockSpecs);

    const { result } = renderHook(() => useSpecs("p1"), {
      wrapper: createWrapper(),
    });

    await waitFor(() => {
      expect(result.current.isSuccess).toBe(true);
    });

    expect(result.current.data).toEqual(mockSpecs);
    expect(mockInvoke).toHaveBeenCalledWith("list_specs", { project_id: "p1" });
  });

  test("handles empty specs", async () => {
    mockInvoke.mockResolvedValue([]);

    const { result } = renderHook(() => useSpecs("p1"), {
      wrapper: createWrapper(),
    });

    await waitFor(() => {
      expect(result.current.isSuccess).toBe(true);
    });

    expect(result.current.data).toHaveLength(0);
  });

  test("does not fetch if projectId is undefined", () => {
    const { result } = renderHook(() => useSpecs(undefined), {
      wrapper: createWrapper(),
    });

    expect(mockInvoke).not.toHaveBeenCalled();
    expect(result.current.isPending).toBe(true); // Disabled query
  });

  test("handles error when fetching specs fails", async () => {
    mockInvoke.mockRejectedValue(new Error("Database error"));

    const { result } = renderHook(() => useSpecs("p1"), {
      wrapper: createWrapper(),
    });

    await waitFor(() => {
      expect(result.current.isError).toBe(true);
    });

    expect(result.current.error).toBeTruthy();
  });
});

describe("useSpec", () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  test("fetches parsed spec with requirements", async () => {
    const mockSpec = {
      spec: {
        id: "s1",
        project_id: "p1",
        filename: "spec.md",
        content: "# Features",
        parsed_at: "2026-02-01T00:00:00Z",
        created_at: "2026-02-01T00:00:00Z",
      },
      requirements: [
        {
          id: "r1",
          spec_id: "s1",
          section: "Features",
          description: "User login",
          req_type: "functional",
          priority: "high",
        },
      ],
    };

    mockInvoke.mockResolvedValue(mockSpec);

    const { result } = renderHook(() => useSpec("s1"), {
      wrapper: createWrapper(),
    });

    await waitFor(() => {
      expect(result.current.isSuccess).toBe(true);
    });

    expect(result.current.data).toEqual(mockSpec);
    expect(result.current.data?.requirements).toHaveLength(1);
    expect(mockInvoke).toHaveBeenCalledWith("get_spec", { id: "s1" });
  });

  test("does not fetch if id is undefined", () => {
    const { result } = renderHook(() => useSpec(undefined), {
      wrapper: createWrapper(),
    });

    expect(mockInvoke).not.toHaveBeenCalled();
    expect(result.current.data).toBeUndefined();
  });

  test("handles spec with no requirements", async () => {
    const mockSpec = {
      spec: {
        id: "s1",
        project_id: "p1",
        filename: "empty.md",
        content: "# Empty",
        parsed_at: "2026-02-01T00:00:00Z",
        created_at: "2026-02-01T00:00:00Z",
      },
      requirements: [],
    };

    mockInvoke.mockResolvedValue(mockSpec);

    const { result } = renderHook(() => useSpec("s1"), {
      wrapper: createWrapper(),
    });

    await waitFor(() => {
      expect(result.current.isSuccess).toBe(true);
    });

    expect(result.current.data?.requirements).toHaveLength(0);
  });

  test("handles multiple requirements", async () => {
    const mockSpec = {
      spec: {
        id: "s1",
        project_id: "p1",
        filename: "full.md",
        content: "# Full Spec",
        parsed_at: "2026-02-01T00:00:00Z",
        created_at: "2026-02-01T00:00:00Z",
      },
      requirements: [
        {
          id: "r1",
          spec_id: "s1",
          section: "Features",
          description: "User login",
          req_type: "functional",
          priority: "high",
        },
        {
          id: "r2",
          spec_id: "s1",
          section: "Features",
          description: "User logout",
          req_type: "functional",
          priority: "medium",
        },
        {
          id: "r3",
          spec_id: "s1",
          section: "Performance",
          description: "Response time < 100ms",
          req_type: "non_functional",
          priority: "high",
        },
      ],
    };

    mockInvoke.mockResolvedValue(mockSpec);

    const { result } = renderHook(() => useSpec("s1"), {
      wrapper: createWrapper(),
    });

    await waitFor(() => {
      expect(result.current.isSuccess).toBe(true);
    });

    expect(result.current.data?.requirements).toHaveLength(3);
    const reqTypes = result.current.data?.requirements.map((r) => r.req_type);
    expect(reqTypes).toContain("functional");
    expect(reqTypes).toContain("non_functional");
  });
});

// Cache behavior is managed by TanStack Query internals and tested in their library
