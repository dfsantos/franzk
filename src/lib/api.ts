import { invoke } from "@tauri-apps/api/core";
import type {
  ApprovalDecision,
  ApprovalRequest,
  ClusterHealth,
  ClusterProfile,
  KafkaMessage,
  MessageFilter,
  NewClusterProfile,
  TopicSpec,
  TopicSummary,
} from "./types";

// Uma função por comando Tauri (src-tauri/src/commands/*.rs), para que cada
// feature importe só o que usa e o nome do comando apareça em um único lugar.

export const clustersApi = {
  list: () => invoke<ClusterProfile[]>("list_clusters"),
  add: (profile: NewClusterProfile) =>
    invoke<ClusterProfile>("add_cluster_profile", { profile }),
  requestTopicCreation: (clusterId: string, spec: TopicSpec, requestedBy: string) =>
    invoke<ApprovalRequest>("request_topic_creation", {
      clusterId,
      spec,
      requestedBy,
    }),
  requestTopicDeletion: (clusterId: string, topicName: string, requestedBy: string) =>
    invoke<ApprovalRequest>("request_topic_deletion", {
      clusterId,
      topicName,
      requestedBy,
    }),
};

export const approvalsApi = {
  listPending: () => invoke<ApprovalRequest[]>("list_pending_approvals"),
  decide: (requestId: string, decision: ApprovalDecision, decidedBy: string) =>
    invoke<ApprovalRequest>("decide_approval", {
      requestId,
      decision,
      decidedBy,
    }),
};

export const messagesApi = {
  listTopics: (clusterId: string) => invoke<TopicSummary[]>("list_topics", { clusterId }),
  filter: (clusterId: string, topicName: string, filter: MessageFilter) =>
    invoke<KafkaMessage[]>("filter_messages", { clusterId, topicName, filter }),
};

export const localEnvApi = {
  createTopic: (spec: TopicSpec) => invoke<TopicSummary>("create_local_topic", { spec }),
  listTopics: () => invoke<TopicSummary[]>("list_local_topics"),
  produceMessage: (topicName: string, key: string | null, value: string) =>
    invoke<KafkaMessage>("produce_local_message", { topicName, key, value }),
  consumeTopic: (topicName: string, limit: number) =>
    invoke<KafkaMessage[]>("consume_local_topic", { topicName, limit }),
};

export const monitoringApi = {
  getClusterHealth: (clusterId: string) =>
    invoke<ClusterHealth>("get_cluster_health", { clusterId }),
};
