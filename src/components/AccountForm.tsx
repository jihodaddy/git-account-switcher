import { useState } from "react";
import type { Account, HostType } from "../lib/types";
import { createAccount, updateAccount, validateToken } from "../lib/commands";
import { X, Eye, EyeOff, CheckCircle, XCircle, Loader } from "lucide-react";

interface Props {
  account: Account | null;
  onClose: () => void;
}

const HOST_DEFAULTS: Record<HostType, string> = {
  github: "github.com",
  gitlab: "gitlab.com",
  bitbucket: "bitbucket.org",
  custom: "",
};

export function AccountForm({ account, onClose }: Props) {
  const isEdit = account !== null;

  const [displayName, setDisplayName] = useState(account?.display_name ?? "");
  const [gitUsername, setGitUsername] = useState(account?.git_username ?? "");
  const [gitEmail, setGitEmail] = useState(account?.git_email ?? "");
  const [hostType, setHostType] = useState<HostType>(account?.host_type ?? "github");
  const [host, setHost] = useState(account?.host ?? "github.com");
  const [authToken, setAuthToken] = useState("");
  const [showToken, setShowToken] = useState(false);
  const [saving, setSaving] = useState(false);
  const [validating, setValidating] = useState(false);
  const [validationResult, setValidationResult] = useState<boolean | null>(null);
  const [error, setError] = useState("");

  const handleHostTypeChange = (type: HostType) => {
    setHostType(type);
    if (HOST_DEFAULTS[type]) {
      setHost(HOST_DEFAULTS[type]);
    }
  };

  const handleValidate = async () => {
    if (!authToken && !isEdit) return;
    setValidating(true);
    try {
      if (isEdit && account) {
        const result = await validateToken(account.id);
        setValidationResult(result.valid);
      } else {
        setValidationResult(null);
        setError("Save the account first to validate the token");
      }
    } catch {
      setValidationResult(false);
    } finally {
      setValidating(false);
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");
    setSaving(true);

    try {
      if (isEdit && account) {
        await updateAccount({
          id: account.id,
          display_name: displayName,
          git_username: gitUsername,
          git_email: gitEmail,
          host,
          host_type: hostType,
          auth_token: authToken || undefined,
        });
      } else {
        if (!authToken) {
          setError("Token is required for new accounts");
          setSaving(false);
          return;
        }
        await createAccount({
          display_name: displayName,
          git_username: gitUsername,
          git_email: gitEmail,
          host,
          host_type: hostType,
          auth_token: authToken,
        });
      }
      onClose();
    } catch (e) {
      setError(String(e));
    } finally {
      setSaving(false);
    }
  };

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal" onClick={(e) => e.stopPropagation()}>
        <div className="modal-header">
          <h3 className="modal-title">
            {isEdit ? "Edit Account" : "Add Account"}
          </h3>
          <button className="btn btn-ghost btn-icon" onClick={onClose}>
            <X size={18} />
          </button>
        </div>

        <form onSubmit={handleSubmit}>
          <div className="modal-body">
            <div className="form-group">
              <label className="form-label">Host Type</label>
              <select
                className="form-select"
                value={hostType}
                onChange={(e) => handleHostTypeChange(e.target.value as HostType)}
              >
                <option value="github">GitHub</option>
                <option value="gitlab">GitLab</option>
                <option value="bitbucket">Bitbucket</option>
                <option value="custom">Custom</option>
              </select>
            </div>

            <div className="form-group">
              <label className="form-label">Host URL</label>
              <input
                className="form-input"
                value={host}
                onChange={(e) => setHost(e.target.value)}
                placeholder="github.com"
                required
              />
            </div>

            <div className="form-group">
              <label className="form-label">Display Name</label>
              <input
                className="form-input"
                value={displayName}
                onChange={(e) => setDisplayName(e.target.value)}
                placeholder="e.g. Personal, Work"
                required
                maxLength={50}
              />
            </div>

            <div className="form-row">
              <div className="form-group">
                <label className="form-label">Git Username</label>
                <input
                  className="form-input"
                  value={gitUsername}
                  onChange={(e) => setGitUsername(e.target.value)}
                  placeholder="user.name"
                  required
                />
              </div>
              <div className="form-group">
                <label className="form-label">Git Email</label>
                <input
                  className="form-input"
                  type="email"
                  value={gitEmail}
                  onChange={(e) => setGitEmail(e.target.value)}
                  placeholder="user@example.com"
                  required
                />
              </div>
            </div>

            <div className="form-group">
              <label className="form-label">
                {isEdit ? "Token (leave empty to keep current)" : "Personal Access Token"}
              </label>
              <div className="token-input-wrapper">
                <input
                  className="form-input"
                  type={showToken ? "text" : "password"}
                  value={authToken}
                  onChange={(e) => {
                    setAuthToken(e.target.value);
                    setValidationResult(null);
                  }}
                  placeholder={isEdit ? "••••••••" : "ghp_xxxxxxxxxxxx"}
                  required={!isEdit}
                  style={{ paddingRight: "70px" }}
                />
                <button
                  type="button"
                  className="token-toggle"
                  onClick={() => setShowToken(!showToken)}
                >
                  {showToken ? <EyeOff size={16} /> : <Eye size={16} />}
                </button>
              </div>
              {isEdit && (
                <button
                  type="button"
                  className="btn btn-secondary btn-sm"
                  onClick={handleValidate}
                  disabled={validating}
                  style={{ marginTop: "8px" }}
                >
                  {validating ? (
                    <Loader size={14} className="spinner" />
                  ) : validationResult === true ? (
                    <CheckCircle size={14} color="var(--success)" />
                  ) : validationResult === false ? (
                    <XCircle size={14} color="var(--danger)" />
                  ) : null}
                  Test Connection
                </button>
              )}
            </div>

            {error && <div className="form-error">{error}</div>}
          </div>

          <div className="modal-footer">
            <button type="button" className="btn btn-secondary" onClick={onClose}>
              Cancel
            </button>
            <button type="submit" className="btn btn-primary" disabled={saving}>
              {saving ? <span className="spinner" /> : isEdit ? "Save Changes" : "Add Account"}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
