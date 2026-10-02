use serde::{Deserialize, Serialize};

use super::topic::TopicSpec;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ApprovalAction {
    CreateTopic {
        cluster_id: String,
        spec: TopicSpec,
    },
    DeleteTopic {
        cluster_id: String,
        topic_name: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalDecision {
    Approve,
    Reject,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApprovalRequest {
    pub id: String,
    pub action: ApprovalAction,
    pub requested_by: String,
    pub status: ApprovalStatus,
    pub decided_by: Option<String>,
}
