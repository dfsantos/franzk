import { Navigate, Route, HashRouter, Routes } from "react-router-dom";
import { AppShell } from "./app/AppShell";
import { MessagesPage } from "./features/messages/MessagesPage";
import { LocalEnvPage } from "./features/local-env/LocalEnvPage";
import { ClustersPage } from "./features/clusters/ClustersPage";
import { ApprovalsPage } from "./features/approvals/ApprovalsPage";
import { MonitoringPage } from "./features/monitoring/MonitoringPage";

// HashRouter: o build de produção do Tauri serve os assets via file://, sem
// um servidor que resolva rotas da History API.
function App() {
  return (
    <HashRouter>
      <Routes>
        <Route element={<AppShell />}>
          <Route index element={<Navigate to="/messages" replace />} />
          <Route path="/messages" element={<MessagesPage />} />
          <Route path="/local-env" element={<LocalEnvPage />} />
          <Route path="/clusters" element={<ClustersPage />} />
          <Route path="/approvals" element={<ApprovalsPage />} />
          <Route path="/monitoring" element={<MonitoringPage />} />
        </Route>
      </Routes>
    </HashRouter>
  );
}

export default App;
