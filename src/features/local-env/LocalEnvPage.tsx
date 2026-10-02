import { FormEvent, useEffect, useState } from "react";
import { localEnvApi } from "../../lib/api";
import { errorMessage } from "../../lib/errors";
import type { KafkaMessage, TopicSummary } from "../../lib/types";

// RF-002: criação/consumo de tópicos locais com configuração mínima, sem o
// fluxo de aprovação dos clusters compartilhados (ver features/clusters).
export function LocalEnvPage() {
  const [topics, setTopics] = useState<TopicSummary[]>([]);
  const [error, setError] = useState<string | null>(null);

  const [name, setName] = useState("");
  const [partitions, setPartitions] = useState(1);
  const [retentionMs, setRetentionMs] = useState(86_400_000);

  const [selectedTopic, setSelectedTopic] = useState<string | null>(null);
  const [key, setKey] = useState("");
  const [value, setValue] = useState("");
  const [messages, setMessages] = useState<KafkaMessage[]>([]);

  async function refreshTopics() {
    try {
      setTopics(await localEnvApi.listTopics());
    } catch (e) {
      setError(errorMessage(e));
    }
  }

  useEffect(() => {
    refreshTopics();
  }, []);

  async function handleCreateTopic(event: FormEvent) {
    event.preventDefault();
    setError(null);
    try {
      await localEnvApi.createTopic({
        name,
        partitions,
        replicationFactor: 1,
        retentionMs,
        aclPrincipals: [],
      });
      setName("");
      await refreshTopics();
    } catch (e) {
      setError(errorMessage(e));
    }
  }

  async function handleProduce(event: FormEvent) {
    event.preventDefault();
    if (!selectedTopic) return;
    setError(null);
    try {
      await localEnvApi.produceMessage(selectedTopic, key || null, value);
      setKey("");
      setValue("");
      await handleConsume();
    } catch (e) {
      setError(errorMessage(e));
    }
  }

  async function handleConsume() {
    if (!selectedTopic) return;
    try {
      setMessages(await localEnvApi.consumeTopic(selectedTopic, 50));
    } catch (e) {
      setError(errorMessage(e));
    }
  }

  return (
    <div className="page">
      <h1>Ambiente Local</h1>
      {error && <div className="error-banner">{error}</div>}

      <h2>Novo tópico</h2>
      <form className="field-group" onSubmit={handleCreateTopic}>
        <input
          placeholder="nome do tópico"
          value={name}
          onChange={(e) => setName(e.target.value)}
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
          min={0}
          value={retentionMs}
          onChange={(e) => setRetentionMs(Number(e.target.value))}
          title="retenção (ms)"
        />
        <button type="submit">Criar</button>
      </form>

      <h2>Tópicos</h2>
      <table>
        <thead>
          <tr>
            <th>Nome</th>
            <th>Partições</th>
            <th>Retenção (ms)</th>
            <th></th>
          </tr>
        </thead>
        <tbody>
          {topics.map((topic) => (
            <tr key={topic.name}>
              <td>{topic.name}</td>
              <td>{topic.partitions}</td>
              <td>{topic.retentionMs}</td>
              <td>
                <button
                  onClick={() => {
                    setSelectedTopic(topic.name);
                    setMessages([]);
                  }}
                >
                  Selecionar
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>

      {selectedTopic && (
        <>
          <h2>Produzir / consumir — {selectedTopic}</h2>
          <form className="field-group" onSubmit={handleProduce}>
            <input placeholder="chave (opcional)" value={key} onChange={(e) => setKey(e.target.value)} />
            <input placeholder="valor" value={value} onChange={(e) => setValue(e.target.value)} required />
            <button type="submit">Produzir</button>
            <button type="button" onClick={handleConsume}>
              Atualizar consumo
            </button>
          </form>
          <table>
            <thead>
              <tr>
                <th>Partição</th>
                <th>Offset</th>
                <th>Chave</th>
                <th>Valor</th>
              </tr>
            </thead>
            <tbody>
              {messages.map((message) => (
                <tr key={`${message.partition}-${message.offset}`}>
                  <td>{message.partition}</td>
                  <td>{message.offset}</td>
                  <td>{message.key ?? "—"}</td>
                  <td>{message.value}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </>
      )}
    </div>
  );
}
