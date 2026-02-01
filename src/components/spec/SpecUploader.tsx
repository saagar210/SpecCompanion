import { open } from "@tauri-apps/plugin-dialog";
import { readFileContent } from "../../lib/api";
import { useUploadSpec } from "../../hooks/useSpecs";

interface Props {
  projectId: string;
}

export function SpecUploader({ projectId }: Props) {
  const uploadSpec = useUploadSpec(projectId);

  const handleUpload = async () => {
    try {
      const selected = await open({
        multiple: false,
        filters: [{ name: "Markdown", extensions: ["md", "txt", "markdown"] }],
      });
      if (!selected || typeof selected !== "string") return;

      const filename = selected.split(/[/\\]/).pop() || selected;
      const content = await readFileContent(selected);
      uploadSpec.mutate({ filename, content });
    } catch {
      // Dialog cancelled or file read failed — mutation error state handles display
    }
  };

  return (
    <div className="flex items-center gap-2">
      {uploadSpec.isError && (
        <span className="text-xs text-danger">Upload failed</span>
      )}
      <button
        onClick={handleUpload}
        disabled={uploadSpec.isPending}
        className="px-4 py-2 bg-primary hover:bg-primary-dark text-white text-sm rounded-lg transition-colors disabled:opacity-50"
      >
        {uploadSpec.isPending ? "Uploading..." : "Upload Spec"}
      </button>
    </div>
  );
}
