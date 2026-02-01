import { useState, useEffect, useRef } from "react";
import { useSettings, useSaveSettings } from "../hooks/useTestGeneration";
import type { AppSettings } from "../lib/types";

export function Settings() {
  const { data: settings, isLoading } = useSettings();
  const saveSettings = useSaveSettings();
  const [form, setForm] = useState<AppSettings>({
    api_key: "",
    default_framework: "jest",
    default_mode: "template",
    scan_exclusions: [],
  });
  const [exclusionInput, setExclusionInput] = useState("");
  const [showSaved, setShowSaved] = useState(false);
  const savedTimerRef = useRef<ReturnType<typeof setTimeout>>(undefined);

  useEffect(() => {
    if (settings) {
      setForm(settings);
      setExclusionInput(settings.scan_exclusions.join(", "));
    }
  }, [settings]);

  // Clean up timer on unmount
  useEffect(() => {
    return () => {
      if (savedTimerRef.current) clearTimeout(savedTimerRef.current);
    };
  }, []);

  const handleSave = () => {
    const exclusions = exclusionInput
      .split(",")
      .map((s) => s.trim())
      .filter(Boolean);
    saveSettings.mutate({ ...form, scan_exclusions: exclusions }, {
      onSuccess: () => {
        setShowSaved(true);
        if (savedTimerRef.current) clearTimeout(savedTimerRef.current);
        savedTimerRef.current = setTimeout(() => setShowSaved(false), 3000);
      },
    });
  };

  if (isLoading) return <p className="text-text-muted">Loading settings...</p>;

  return (
    <div>
      <h2 className="text-2xl font-bold mb-6">Settings</h2>
      <div className="max-w-lg space-y-6">
        {/* API Key */}
        <div>
          <label className="block text-sm text-text-muted mb-1">Claude API Key</label>
          <input
            type="password"
            value={form.api_key}
            onChange={(e) => setForm({ ...form, api_key: e.target.value })}
            placeholder="sk-ant-..."
            className="w-full bg-surface border border-border rounded-lg px-3 py-2 text-sm text-text focus:outline-none focus:border-primary"
          />
          <p className="text-xs text-text-muted mt-1">Required for LLM-powered test generation.</p>
        </div>

        {/* Default Framework */}
        <div>
          <label className="block text-sm text-text-muted mb-1">Default Framework</label>
          <select
            value={form.default_framework}
            onChange={(e) => setForm({ ...form, default_framework: e.target.value as "jest" | "pytest" })}
            className="bg-surface border border-border rounded-lg px-3 py-2 text-sm text-text"
          >
            <option value="jest">Jest</option>
            <option value="pytest">PyTest</option>
          </select>
        </div>

        {/* Default Mode */}
        <div>
          <label className="block text-sm text-text-muted mb-1">Default Generation Mode</label>
          <select
            value={form.default_mode}
            onChange={(e) => setForm({ ...form, default_mode: e.target.value as "template" | "llm" })}
            className="bg-surface border border-border rounded-lg px-3 py-2 text-sm text-text"
          >
            <option value="template">Template (offline)</option>
            <option value="llm">LLM (Claude API)</option>
          </select>
        </div>

        {/* Scan Exclusions */}
        <div>
          <label className="block text-sm text-text-muted mb-1">Scan Exclusion Patterns</label>
          <input
            type="text"
            value={exclusionInput}
            onChange={(e) => setExclusionInput(e.target.value)}
            placeholder="dist, build, .cache"
            className="w-full bg-surface border border-border rounded-lg px-3 py-2 text-sm text-text focus:outline-none focus:border-primary"
          />
          <p className="text-xs text-text-muted mt-1">Comma-separated directory names to skip during codebase scanning.</p>
        </div>

        <button
          onClick={handleSave}
          disabled={saveSettings.isPending}
          className="px-6 py-2 bg-primary hover:bg-primary-dark text-white text-sm rounded-lg transition-colors disabled:opacity-50"
        >
          {saveSettings.isPending ? "Saving..." : "Save Settings"}
        </button>

        {showSaved && (
          <p className="text-sm text-success">Settings saved.</p>
        )}
        {saveSettings.isError && (
          <p className="text-sm text-danger">Failed to save settings.</p>
        )}
      </div>
    </div>
  );
}
