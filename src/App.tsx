import { useState, useEffect } from "react";
import { AccountList } from "./components/AccountList";
import { AccountForm } from "./components/AccountForm";
import { StatusBar } from "./components/StatusBar";
import { Settings } from "./components/Settings";
import { RepoBindings } from "./components/RepoBindings";
import { getAccounts, getCurrentGitUser } from "./lib/commands";
import type { Account, GitUser } from "./lib/types";
import { Users, FolderGit2, SettingsIcon } from "lucide-react";

type Page = "accounts" | "repos" | "settings";

function App() {
  const [page, setPage] = useState<Page>("accounts");
  const [accounts, setAccounts] = useState<Account[]>([]);
  const [gitUser, setGitUser] = useState<GitUser | null>(null);
  const [showForm, setShowForm] = useState(false);
  const [editingAccount, setEditingAccount] = useState<Account | null>(null);
  const [theme, setTheme] = useState<"dark" | "light">("dark");

  const refreshData = async () => {
    try {
      const [accts, user] = await Promise.all([
        getAccounts(),
        getCurrentGitUser(),
      ]);
      setAccounts(accts);
      setGitUser(user);
    } catch (e) {
      console.error("Failed to load data:", e);
    }
  };

  useEffect(() => {
    refreshData();
  }, []);

  useEffect(() => {
    document.documentElement.setAttribute("data-theme", theme);
  }, [theme]);

  const handleEdit = (account: Account) => {
    setEditingAccount(account);
    setShowForm(true);
  };

  const handleFormClose = () => {
    setShowForm(false);
    setEditingAccount(null);
    refreshData();
  };

  const navItems = [
    { id: "accounts" as Page, icon: Users, label: "Accounts" },
    { id: "repos" as Page, icon: FolderGit2, label: "Repos" },
    { id: "settings" as Page, icon: SettingsIcon, label: "Settings" },
  ];

  return (
    <div className="app">
      <nav className="sidebar">
        <div className="sidebar-header">
          <h1 className="app-title">GAS</h1>
          <span className="app-subtitle">Git Account Switcher</span>
        </div>
        <ul className="nav-list">
          {navItems.map((item) => (
            <li key={item.id}>
              <button
                className={`nav-item ${page === item.id ? "active" : ""}`}
                onClick={() => setPage(item.id)}
              >
                <item.icon size={20} />
                <span>{item.label}</span>
              </button>
            </li>
          ))}
        </ul>
      </nav>

      <main className="main-content">
        <StatusBar gitUser={gitUser} accounts={accounts} />

        {page === "accounts" && (
          <AccountList
            accounts={accounts}
            onAdd={() => setShowForm(true)}
            onEdit={handleEdit}
            onRefresh={refreshData}
          />
        )}
        {page === "repos" && <RepoBindings accounts={accounts} />}
        {page === "settings" && (
          <Settings theme={theme} onThemeChange={setTheme} />
        )}

        {showForm && (
          <AccountForm
            account={editingAccount}
            onClose={handleFormClose}
          />
        )}
      </main>
    </div>
  );
}

export default App;
