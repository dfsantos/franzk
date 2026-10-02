// Espelha os tipos em src-tauri/src/domain/*.rs. Mantenha os dois em sincronia
// manualmente até a trilha "tauri-specta" (ver docs/plan/DEVELOPMENT_PLAN.md)
// gerar esses bindings automaticamente a partir do Rust.

export type Environment = "local" | "test" | "staging" | "production";

export interface ClusterProfile {
  id: string;
  name: string;
  environment: Environment;
  bootstrapServers: string;
}

export interface NewClusterProfile {
  name: string;
  environment: Environment;
  bootstrapServers: string;
}

export interface TopicSpec {
  name: string;
  partitions: number;
  replicationFactor: number;
  retentionMs: number;
  aclPrincipals: string[];
}

export interface TopicSummary {
  clusterId: string;
  name: string;
  partitions: number;
  replicationFactor: number;
  retentionMs: number;
}

export interface KafkaMessage {
  partition: number;
  offset: number;
  key: string | null;
  value: string;
  timestamp: number;
}

export interface MessageFilter {
  contentContains?: string;
  keyEquals?: string;
  timestampFrom?: number;
  timestampTo?: number;
  limit: number;
}

export type ApprovalAction =
  | { type: "createTopic"; clusterId: string; spec: TopicSpec }
  | { type: "deleteTopic"; clusterId: string; topicName: string };

export type ApprovalStatus = "pending" | "approved" | "rejected";
export type ApprovalDecision = "approve" | "reject";

export interface ApprovalRequest {
  id: string;
  action: ApprovalAction;
  requestedBy: string;
  status: ApprovalStatus;
  decidedBy: string | null;
}

export type BrokerState = "online" | "offline" | "degraded";

export interface BrokerStatus {
  brokerId: number;
  host: string;
  state: BrokerState;
}

export interface PartitionInfo {
  topic: string;
  partition: number;
  leaderBrokerId: number;
  inSyncReplicas: number;
}

export interface ConsumerGroupLag {
  groupId: string;
  topic: string;
  partition: number;
  lag: number;
}

export interface ThroughputSample {
  messagesPerSecond: number;
  bytesPerSecond: number;
}

export interface ClusterHealth {
  brokers: BrokerStatus[];
  partitions: PartitionInfo[];
  consumerLag: ConsumerGroupLag[];
  throughput: ThroughputSample;
}

/** Formato de erro serializado por src-tauri/src/error.rs. */
export interface AppErrorPayload {
  kind: "not_found" | "invalid_request" | "forbidden";
  message: string;
}
