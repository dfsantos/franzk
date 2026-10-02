import { useEffect, useState } from "react";
import { clustersApi, monitoringApi } from "../../lib/api";
import { errorMessage } from "../../lib/errors";
import type { ClusterHealth, ClusterProfile } from "../../lib/types";

// RF-005: lag de consumers, estado dos brokers, partições e throughput do
// cluster selecionado.
export function MonitoringPage() {
  const [clusters, setClusters] = useState<ClusterProfile[]>([]);
  const [clusterId, setClusterId] = useState("");
  const [health, setHealth] = useState<ClusterHealth | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    clustersApi.list().then(setClusters).catch((e) => setError(errorMessage(e)));
  }, []);

  useEffect(() => {
    if (!clusterId) return;
    monitoringApi
      .getClusterHealth(clusterId)
      .then(setHealth)
      .catch((e) => setError(errorMessage(e)));
  }, [clusterId]);

  return (
    <div className="page">
      <h1>Monitoramento de Cluster</h1>
      {error && <div className="error-banner">{error}</div>}

      <select value={clusterId} onChange={(e) => setClusterId(e.target.value)}>
        <option value="">cluster...</option>
        {clusters.map((cluster) => (
          <option key={cluster.id} value={cluster.id}>
            {cluster.name}
          </option>
        ))}
      </select>

      {health && (
        <>
          <h2>Brokers</h2>
          <table>
            <thead>
              <tr>
                <th>ID</th>
                <th>Host</th>
                <th>Estado</th>
              </tr>
            </thead>
            <tbody>
              {health.brokers.map((broker) => (
                <tr key={broker.brokerId}>
                  <td>{broker.brokerId}</td>
                  <td>{broker.host}</td>
                  <td>{broker.state}</td>
                </tr>
              ))}
            </tbody>
          </table>

          <h2>Lag de consumers</h2>
          <table>
            <thead>
              <tr>
                <th>Grupo</th>
                <th>Tópico</th>
                <th>Partição</th>
                <th>Lag</th>
              </tr>
            </thead>
            <tbody>
              {health.consumerLag.map((lag) => (
                <tr key={`${lag.groupId}-${lag.topic}-${lag.partition}`}>
                  <td>{lag.groupId}</td>
                  <td>{lag.topic}</td>
                  <td>{lag.partition}</td>
                  <td>{lag.lag}</td>
                </tr>
              ))}
            </tbody>
          </table>

          <h2>Throughput</h2>
          <p>
            {health.throughput.messagesPerSecond.toFixed(1)} msg/s ·{" "}
            {health.throughput.bytesPerSecond.toFixed(0)} bytes/s
          </p>
        </>
      )}
    </div>
  );
}
