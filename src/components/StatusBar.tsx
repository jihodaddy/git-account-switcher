import type { Account, GitUser } from "../lib/types";
import { GitBranch } from "lucide-react";

interface Props {
  gitUser: GitUser | null;
  accounts: Account[];
}

export function StatusBar({ gitUser, accounts }: Props) {
  const activeAccount = accounts.find((a) => a.is_active);

  return (
    <div className="status-bar">
      <div className="status-info">
        <div className={`status-dot ${activeAccount ? "" : "inactive"}`} />
        <div>
          <div className="status-label">Current Git Identity</div>
          <div className="status-value">
            {gitUser?.name || "Not configured"}
          </div>
          {gitUser?.email && (
            <div className="status-email">{gitUser.email}</div>
          )}
        </div>
      </div>
      {activeAccount && (
        <div className={`account-host-badge ${activeAccount.host_type}`}>
          <GitBranch size={12} />
          {activeAccount.host}
        </div>
      )}
    </div>
  );
}
