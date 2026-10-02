import { FormEvent, useEffect, useState } from "react";
import { clustersApi, messagesApi } from "../../lib/api";
import { errorMessage } from "../../lib/errors";
import type { ClusterProfile, KafkaMessage, TopicSummary } from "../../lib/types";

// RF-001: localizar mensagens de um tópico já conhecido filtrando por
// conteúdo, chave e/ou intervalo de timestamp (filtros são combináveis).
export function MessagesPage() {
  const [clusters, setClusters] = useState<ClusterProfile[]>([]);
  const [clusterId, setClusterId] = useState("");
  const [topics, setTopics] = useState<TopicSummary[]>([]);
  const [topicName, setTopicName] = useState("");

  const [contentContains, setContentContains] = useState("");
  const [keyEquals, setKeyEquals] = useState("");
  const [results, setResults] = useState<KafkaMessage[]>([]);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    clustersApi.list().then(setClusters).catch((e) => setError(errorMessage(e)));
  }, []);

  useEffect(() => {
    if (!clusterId) return;
    messagesApi
      .listTopics(clusterId)
      .then(setTopics)
      .catch((e) => setError(errorMessage(e)));
  }, [clusterId]);

  async function handleFilter(event: FormEvent) {
    event.preventDefault();
    if (!clusterId || !topicName) return;
    setError(null);
    try {
      const found = await messagesApi.filter(clusterId, topicName, {
        contentContains: contentContains || undefined,
        keyEquals: keyEquals || undefined,
        limit: 100,
      });
      setResults(found);
    } catch (e) {
      setError(errorMessage(e));
    }
  }

  return (
    <div className="page">
      <h1>Inspeção de Mensagens</h1>
      {error && <div className="error-banner">{error}</div>}

      <form className="field-group" onSubmit={handleFilter}>
        <select value={clusterId} onChange={(e) => setClusterId(e.target.value)} required>
          <option value="">cluster...</option>
          {clusters.map((cluster) => (
            <option key={cluster.id} value={cluster.id}>
              {cluster.name}
            </option>
          ))}
        </select>
        <select value={topicName} onChange={(e) => setTopicName(e.target.value)} required>
          <option value="">tópico...</option>
          {topics.map((topic) => (
            <option key={topic.name} value={topic.name}>
              {topic.name}
            </option>
          ))}
        </select>
        <input
          placeholder="conteúdo contém..."
          value={contentContains}
          onChange={(e) => setContentContains(e.target.value)}
        />
        <input placeholder="chave igual a..." value={keyEquals} onChange={(e) => setKeyEquals(e.target.value)} />
        <button type="submit">Filtrar</button>
      </form>

      <table>
        <thead>
          <tr>
            <th>Partição</th>
            <th>Offset</th>
            <th>Chave</th>
            <th>Valor</th>
            <th>Timestamp</th>
          </tr>
        </thead>
        <tbody>
          {results.map((message) => (
            <tr key={`${message.partition}-${message.offset}`}>
              <td>{message.partition}</td>
              <td>{message.offset}</td>
              <td>{message.key ?? "—"}</td>
              <td>{message.value}</td>
              <td>{new Date(message.timestamp).toLocaleString()}</td>
            </tr>
          ))}
        </tbody>
      </table>
      {results.length === 0 && <p>Nenhuma mensagem encontrada com o filtro aplicado.</p>}
    </div>
  );
}
