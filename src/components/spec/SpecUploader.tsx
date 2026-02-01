import { open } from "@tauri-apps/plugin-dialog";
import { readFileContent } from "../../lib/api";
import { useUploadSpec } from "../../hooks/useSpecs";

interface Props {
  projectId: string;
}

export function SpecUploader({ projectId }: Props) {
  const uploadSpec = useUploadSpec(projectId);

  const handleUpload = async () => {
    const selected = await open({
      multiple: false,
      filters: [{ name: "Markdown", extensions: ["md", "txt", "markdown"] }],
    });
    if (!selected) return;

    const path = selected as string;
    const filename = path.split("/").pop() || path;
    const content = await readFileContent(path);
    uploadSpec.mutate({ filename, content });
  };

  return (
    <button
      onClick={handleUpload}
      disabled={uploadSpec.isPending}
      className="px-4 py-2 bg-primary hover:bg-primary-dark text-white text-sm rounded-lg transition-colors disabled:opacity-50"
    >
      {uploadSpec.isPending ? "Uploading..." : "Upload Spec"}
    </button>
  );
}
