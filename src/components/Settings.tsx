import { Moon, Sun } from "lucide-react";

interface Props {
  theme: "dark" | "light";
  onThemeChange: (theme: "dark" | "light") => void;
}

export function Settings({ theme, onThemeChange }: Props) {
  return (
    <div>
      <div className="page-header">
        <h2 className="page-title">Settings</h2>
      </div>

      <div className="settings-section">
        <h3 className="settings-title">Appearance</h3>
        <div className="setting-row">
          <div>
            <div className="setting-label">Dark Mode</div>
            <div className="setting-description">
              Toggle between dark and light theme
            </div>
          </div>
          <button
            className={`toggle ${theme === "dark" ? "on" : ""}`}
            onClick={() => onThemeChange(theme === "dark" ? "light" : "dark")}
          >
            <div className="toggle-knob" />
          </button>
        </div>
      </div>

      <div className="settings-section">
        <h3 className="settings-title">About</h3>
        <div className="setting-row">
          <div>
            <div className="setting-label">Git Account Switcher</div>
            <div className="setting-description">Version 0.1.0</div>
          </div>
          <div style={{ display: "flex", gap: 4, color: "var(--text-muted)" }}>
            {theme === "dark" ? <Moon size={16} /> : <Sun size={16} />}
          </div>
        </div>
      </div>
    </div>
  );
}
