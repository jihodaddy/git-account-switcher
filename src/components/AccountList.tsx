import { useState } from "react";
import type { Account } from "../lib/types";
import { switchAccount, deleteAccount } from "../lib/commands";
import { Plus, Pencil, Trash2, ArrowRightLeft, Check, AlertTriangle } from "lucide-react";

interface Props {
  accounts: Account[];
  onAdd: () => void;
  onEdit: (account: Account) => void;
  onRefresh: () => void;
}

interface ToastState {
  message: string;
  type: "success" | "error" | "warning";
}

export function AccountList({ accounts, onAdd, onEdit, onRefresh }: Props) {
  const [switching, setSwitching] = useState<string | null>(null);
  const [toast, setToast] = useState<ToastState | null>(null);
  const [confirmDelete, setConfirmDelete] = useState<string | null>(null);

  const showToast = (message: string, type: ToastState["type"]) => {
    setToast({ message, type });
    setTimeout(() => setToast(null), 3000);
  };

  const handleSwitch = async (id: string) => {
    setSwitching(id);
    try {
      const result = await switchAccount(id);
      if (result.validation_result === "invalid") {
        showToast("Switched, but token may be invalid", "warning");
      } else {
        showToast("Account switched successfully", "success");
      }
      onRefresh();
    } catch (e) {
      showToast(`Switch failed: ${e}`, "error");
    } finally {
      setSwitching(null);
    }
  };

  const handleDelete = async (id: string) => {
    try {
      await deleteAccount(id);
      showToast("Account deleted", "success");
      setConfirmDelete(null);
      onRefresh();
    } catch (e) {
      showToast(`Delete failed: ${e}`, "error");
    }
  };

  return (
    <div>
      <div className="page-header">
        <h2 className="page-title">Accounts</h2>
        <button className="btn btn-primary" onClick={onAdd}>
          <Plus size={16} />
          Add Account
        </button>
      </div>

      {accounts.length === 0 ? (
        <div className="empty-state">
          <div className="empty-state-icon">
            <ArrowRightLeft size={48} />
          </div>
          <h3>No accounts yet</h3>
          <p>Add your Git accounts to start switching between them.</p>
          <button className="btn btn-primary" onClick={onAdd}>
            <Plus size={16} />
            Add First Account
          </button>
        </div>
      ) : (
        <div className="account-grid">
          {accounts.map((account) => (
            <div
              key={account.id}
              className={`account-card ${account.is_active ? "active" : ""}`}
            >
              <div className="account-card-header">
                <span className={`account-host-badge ${account.host_type}`}>
                  {account.host_type}
                </span>
                {account.is_active && (
                  <span className="account-active-badge">
                    <Check size={12} />
                    Active
                  </span>
                )}
              </div>
              <div className="account-name">{account.display_name}</div>
              <div className="account-detail">{account.git_username}</div>
              <div className="account-detail">{account.git_email}</div>
              <div className="account-detail" style={{ color: "var(--text-muted)", fontSize: "12px" }}>
                {account.host}
              </div>
              <div className="account-actions">
                {!account.is_active && (
                  <button
                    className="btn btn-primary btn-sm"
                    onClick={() => handleSwitch(account.id)}
                    disabled={switching !== null}
                  >
                    {switching === account.id ? (
                      <span className="spinner" />
                    ) : (
                      <>
                        <ArrowRightLeft size={14} />
                        Switch
                      </>
                    )}
                  </button>
                )}
                <button
                  className="btn btn-secondary btn-sm"
                  onClick={() => onEdit(account)}
                >
                  <Pencil size={14} />
                  Edit
                </button>
                <button
                  className="btn btn-ghost btn-icon btn-sm"
                  onClick={() => setConfirmDelete(account.id)}
                >
                  <Trash2 size={14} />
                </button>
              </div>
            </div>
          ))}
        </div>
      )}

      {confirmDelete && (
        <div className="modal-overlay" onClick={() => setConfirmDelete(null)}>
          <div className="modal" onClick={(e) => e.stopPropagation()} style={{ width: 380 }}>
            <div className="confirm-dialog">
              <AlertTriangle size={40} color="var(--warning)" />
              <p>
                Are you sure you want to delete this account?
                <br />
                This will also remove its credentials from Windows Credential Manager.
              </p>
              <div className="confirm-actions">
                <button className="btn btn-secondary" onClick={() => setConfirmDelete(null)}>
                  Cancel
                </button>
                <button className="btn btn-danger" onClick={() => handleDelete(confirmDelete)}>
                  <Trash2 size={14} />
                  Delete
                </button>
              </div>
            </div>
          </div>
        </div>
      )}

      {toast && (
        <div className={`toast ${toast.type}`}>
          {toast.type === "success" && <Check size={16} />}
          {toast.type === "error" && <AlertTriangle size={16} />}
          {toast.type === "warning" && <AlertTriangle size={16} />}
          {toast.message}
        </div>
      )}
    </div>
  );
}
