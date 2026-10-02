import { FormEvent, useEffect, useState } from "react";
import { clustersApi } from "../../lib/api";
import { errorMessage } from "../../lib/errors";
import type { ClusterProfile } from "../../lib/types";

// RF-004 + Regras de negócio 1 e 2 (PRD §6): criação/exclusão de tópico em
// cluster compartilhado passa por aprovação de outro engenheiro de
// plataforma. "requestedBy" é um placeholder até a autenticação existir
// (ver docs/plan/DEVELOPMENT_PLAN.md).
export function ClustersPage() {
  const [clusters, setClusters] = useState<ClusterProfile[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);

  const [clusterId, setClusterId] = useState("");
  const [topicName, setTopicName] = useState("");
  const [partitions, setPartitions] = useState(3);
  const [replicationFactor, setReplicationFactor] = useState(3);
  const [retentionMs, setRetentionMs] = useState(604_800_000);
  const [requestedBy, setRequestedBy] = useState("");

  async function refresh() {
    try {
      const all = await clustersApi.list();
      setClusters(all.filter((c) => c.environment !== "local"));
    } catch (e) {
      setError(errorMessage(e));
    }
  }

  useEffect(() => {
    refresh();
  }, []);

  async function handleRequestCreation(event: FormEvent) {
    event.preventDefault();
    setError(null);
    setNotice(null);
    try {
      const request = await clustersApi.requestTopicCreation(
        clusterId,
        {
          name: topicName,
          partitions,
          replicationFactor,
          retentionMs,
          aclPrincipals: [],
        },
        requestedBy,
      );
      setNotice(
        request.status === "pending"
          ? `Solicitação ${request.id} criada e aguardando aprovação de outro engenheiro de plataforma.`
          : `Tópico criado (${request.status}).`,
      );
      setTopicName("");
    } catch (e) {
      setError(errorMessage(e));
    }
  }

  return (
    <div className="page">
      <h1>Clusters Compartilhados</h1>
      {error && <div className="error-banner">{error}</div>}
      {notice && <div className="field-group">{notice}</div>}

      <h2>Clusters</h2>
      <table>
        <thead>
          <tr>
            <th>Nome</th>
            <th>Ambiente</th>
            <th>Bootstrap servers</th>
          </tr>
        </thead>
        <tbody>
          {clusters.map((cluster) => (
            <tr key={cluster.id}>
              <td>{cluster.name}</td>
              <td>{cluster.environment}</td>
              <td>{cluster.bootstrapServers}</td>
            </tr>
          ))}
        </tbody>
      </table>
      {clusters.length === 0 && (
        <p>
          Nenhum cluster compartilhado cadastrado ainda. O cadastro de clusters (nome,
          bootstrap servers, ambiente) é uma entrega própria — ver
          docs/plan/DEVELOPMENT_PLAN.md.
        </p>
      )}

      <h2>Solicitar criação de tópico</h2>
      <form className="field-group" onSubmit={handleRequestCreation}>
        <select value={clusterId} onChange={(e) => setClusterId(e.target.value)} required>
          <option value="">cluster...</option>
          {clusters.map((cluster) => (
            <option key={cluster.id} value={cluster.id}>
              {cluster.name}
            </option>
          ))}
        </select>
        <input
          placeholder="nome do tópico"
          value={topicName}
          onChange={(e) => setTopicName(e.target.value)}
          required
        />
        <input
          type="number"
          min={1}
          value={partitions}
          onChange={(e) => setPartitions(Number(e.target.value))}
          title="partições"
        />
        <input
          type="number"
          min={1}
          value={replicationFactor}
          onChange={(e) => setReplicationFactor(Number(e.target.value))}
          title="fator de replicação"
        />
        <input
          type="number"
          min={0}
          value={retentionMs}
          onChange={(e) => setRetentionMs(Number(e.target.value))}
          title="retenção (ms)"
        />
        <input
          placeholder="solicitado por"
          value={requestedBy}
          onChange={(e) => setRequestedBy(e.target.value)}
          required
        />
        <button type="submit">Solicitar criação</button>
      </form>
    </div>
  );
}
