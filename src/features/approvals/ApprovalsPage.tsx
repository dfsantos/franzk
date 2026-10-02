import { useEffect, useState } from "react";
import { approvalsApi } from "../../lib/api";
import { errorMessage } from "../../lib/errors";
import type { ApprovalRequest } from "../../lib/types";

function describeAction(request: ApprovalRequest): string {
  if (request.action.type === "createTopic") {
    return `Criar tópico "${request.action.spec.name}" em ${request.action.clusterId}`;
  }
  return `Excluir tópico "${request.action.topicName}" em ${request.action.clusterId}`;
}

// Regras de negócio 1 e 2 (PRD §6): revisão por pares — quem decide não
// pode ser quem solicitou. O backend (commands/approvals.rs) também aplica
// essa regra; a UI só evita o erro óbvio de digitar o mesmo nome.
export function ApprovalsPage() {
  const [pending, setPending] = useState<ApprovalRequest[]>([]);
  const [decidedBy, setDecidedBy] = useState("");
  const [error, setError] = useState<string | null>(null);

  async function refresh() {
    try {
      setPending(await approvalsApi.listPending());
    } catch (e) {
      setError(errorMessage(e));
    }
  }

  useEffect(() => {
    refresh();
  }, []);

  async function decide(request: ApprovalRequest, approve: boolean) {
    setError(null);
    if (approve && request.requestedBy === decidedBy) {
      setError("quem solicitou a ação não pode aprová-la (revisão por pares)");
      return;
    }
    try {
      await approvalsApi.decide(request.id, approve ? "approve" : "reject", decidedBy);
      await refresh();
    } catch (e) {
      setError(errorMessage(e));
    }
  }

  return (
    <div className="page">
      <h1>Aprovações Pendentes</h1>
      {error && <div className="error-banner">{error}</div>}

      <div className="field-group">
        <input
          placeholder="seu nome (aprovador)"
          value={decidedBy}
          onChange={(e) => setDecidedBy(e.target.value)}
          required
        />
      </div>

      <table>
        <thead>
          <tr>
            <th>Ação</th>
            <th>Solicitado por</th>
            <th></th>
          </tr>
        </thead>
        <tbody>
          {pending.map((request) => (
            <tr key={request.id}>
              <td>{describeAction(request)}</td>
              <td>{request.requestedBy}</td>
              <td>
                <button disabled={!decidedBy} onClick={() => decide(request, true)}>
                  Aprovar
                </button>
                <button disabled={!decidedBy} onClick={() => decide(request, false)}>
                  Rejeitar
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
      {pending.length === 0 && <p>Nenhuma solicitação pendente.</p>}
    </div>
  );
}
