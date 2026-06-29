import { useMemo, useState } from "react";

type RouteResult = "tally.list_ledgers" | "unknown";

export function App() {
  const [prompt, setPrompt] = useState("list ledgers");

  const route: RouteResult = useMemo(() => {
    const normalized = prompt.toLowerCase();
    return normalized.includes("list") && normalized.includes("ledger")
      ? "tally.list_ledgers"
      : "unknown";
  }, [prompt]);

  return (
    <main className="shell">
      <section className="sidebar" aria-label="Status">
        <div>
          <p className="label">Runtime</p>
          <h1>Tally AI Companion</h1>
        </div>
        <div className="status-list">
          <Status label="Mode" value="Offline" />
          <Status label="License" value="Unchecked" />
          <Status label="Tally" value="Not connected" />
        </div>
      </section>

      <section className="workspace" aria-label="Assistant">
        <div className="toolbar">
          <p className="label">First demo target</p>
          <span>Detect Tally, route a read-only tool, summarize locally.</span>
        </div>

        <label className="composer">
          <span>Ask</span>
          <textarea
            value={prompt}
            onChange={(event) => setPrompt(event.target.value)}
            rows={4}
          />
        </label>

        <div className="result">
          <p className="label">Planned tool</p>
          <strong>{route}</strong>
        </div>
      </section>
    </main>
  );
}

function Status({ label, value }: { label: string; value: string }) {
  return (
    <div className="status-row">
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

