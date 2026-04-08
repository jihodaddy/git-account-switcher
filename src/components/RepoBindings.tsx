import { useState, useEffect } from "react";
import type { Account, RepositoryBinding } from "../lib/types";
import { getBindings, bindRepository, unbindRepository } from "../lib/commands";
import { open } from "@tauri-apps/plugin-dialog";
import { FolderGit2, Plus, Trash2, Link } from "lucide-react";

interface Props {
  accounts: Account[];
}

export function RepoBindings({ accounts }: Props) {
  const [bindings, setBindings] = useState<RepositoryBinding[]>([]);
  const [showAdd, setShowAdd] = useState(false);
  const [selectedAccount, setSelectedAccount] = useState("");
  const [repoPath, setRepoPath] = useState("");

  const refresh = async () => {
    try {
      const b = await getBindings();
      setBindings(b);
    } catch (e) {
      console.error(e);
    }
  };

  useEffect(() => {
    refresh();
  }, []);

  const handleBrowse = async () => {
    const selected = await open({ directory: true, multiple: false });
    if (selected) {
      setRepoPath(selected as string);
    }
  };

  const handleAdd = async () => {
    if (!repoPath || !selectedAccount) return;
    try {
      await bindRepository(repoPath, selectedAccount);
      setShowAdd(false);
      setRepoPath("");
      setSelectedAccount("");
      refresh();
    } catch (e) {
      console.error(e);
    }
  };

  const handleRemove = async (id: string) => {
    try {
      await unbindRepository(id);
      refresh();
    } catch (e) {
      console.error(e);
    }
  };

  const getAccountName = (id: string) =>
    accounts.find((a) => a.id === id)?.display_name ?? "Unknown";

  return (
    <div>
      <div className="page-header">
        <h2 className="page-title">Repository Bindings</h2>
        <button className="btn btn-primary" onClick={() => setShowAdd(true)}>
          <Plus size={16} />
          Bind Repository
        </button>
      </div>

      {bindings.length === 0 ? (
        <div className="empty-state">
          <div className="empty-state-icon">
            <FolderGit2 size={48} />
          </div>
          <h3>No repository bindings</h3>
          <p>Bind a repository to use a specific account for that directory.</p>
        </div>
      ) : (
        <div className="binding-list">
          {bindings.map((b) => (
            <div key={b.id} className="binding-item">
              <div>
                <div className="binding-path">{b.repo_path}</div>
                <div className="binding-account">
                  <Link size={12} style={{ display: "inline", marginRight: 4 }} />
                  {b.account_name || getAccountName(b.account_id)}
                </div>
              </div>
              <button
                className="btn btn-ghost btn-icon"
                onClick={() => handleRemove(b.id)}
              >
                <Trash2 size={16} />
              </button>
            </div>
          ))}
        </div>
      )}

      {showAdd && (
        <div className="modal-overlay" onClick={() => setShowAdd(false)}>
          <div className="modal" onClick={(e) => e.stopPropagation()} style={{ width: 420 }}>
            <div className="modal-header">
              <h3 className="modal-title">Bind Repository</h3>
              <button className="btn btn-ghost btn-icon" onClick={() => setShowAdd(false)}>
                <X size={18} />
              </button>
            </div>
            <div className="modal-body">
              <div className="form-group">
                <label className="form-label">Repository Path</label>
                <div style={{ display: "flex", gap: 8 }}>
                  <input
                    className="form-input"
                    value={repoPath}
                    onChange={(e) => setRepoPath(e.target.value)}
                    placeholder="C:\projects\my-repo"
                  />
                  <button className="btn btn-secondary" onClick={handleBrowse} type="button">
                    Browse
                  </button>
                </div>
              </div>
              <div className="form-group">
                <label className="form-label">Account</label>
                <select
                  className="form-select"
                  value={selectedAccount}
                  onChange={(e) => setSelectedAccount(e.target.value)}
                >
                  <option value="">Select account...</option>
                  {accounts.map((a) => (
                    <option key={a.id} value={a.id}>
                      {a.display_name} ({a.git_email})
                    </option>
                  ))}
                </select>
              </div>
            </div>
            <div className="modal-footer">
              <button className="btn btn-secondary" onClick={() => setShowAdd(false)}>
                Cancel
              </button>
              <button
                className="btn btn-primary"
                onClick={handleAdd}
                disabled={!repoPath || !selectedAccount}
              >
                Bind
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

function X({ size }: { size: number }) {
  return (
    <svg width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
      <line x1="18" y1="6" x2="6" y2="18" /><line x1="6" y1="6" x2="18" y2="18" />
    </svg>
  );
}
