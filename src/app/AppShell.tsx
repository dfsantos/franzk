import { NavLink, Outlet } from "react-router-dom";
import "./AppShell.css";

const NAV_ITEMS = [
  { to: "/messages", label: "Inspeção de Mensagens" },
  { to: "/local-env", label: "Ambiente Local" },
  { to: "/clusters", label: "Clusters Compartilhados" },
  { to: "/approvals", label: "Aprovações" },
  { to: "/monitoring", label: "Monitoramento" },
];

export function AppShell() {
  return (
    <div className="app-shell">
      <nav className="app-shell__sidebar">
        <div className="app-shell__brand">Franzk</div>
        <ul>
          {NAV_ITEMS.map((item) => (
            <li key={item.to}>
              <NavLink
                to={item.to}
                className={({ isActive }) => (isActive ? "active" : undefined)}
              >
                {item.label}
              </NavLink>
            </li>
          ))}
        </ul>
      </nav>
      <main className="app-shell__content">
        <Outlet />
      </main>
    </div>
  );
}
