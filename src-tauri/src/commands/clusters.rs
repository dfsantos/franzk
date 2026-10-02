use tauri::State;
use uuid::Uuid;

use crate::domain::approval::{ApprovalAction, ApprovalRequest, ApprovalStatus};
use crate::domain::cluster::{ClusterProfile, NewClusterProfile};
use crate::domain::topic::TopicSpec;
use crate::error::{AppError, AppResult};
use crate::state::{AppState, LOCAL_CLUSTER_ID};

#[tauri::command]
pub fn list_clusters(state: State<AppState>) -> AppResult<Vec<ClusterProfile>> {
    let clusters = state.clusters.lock().expect("clusters lock");
    Ok(clusters.values().cloned().collect())
}

#[tauri::command]
pub fn add_cluster_profile(
    state: State<AppState>,
    profile: NewClusterProfile,
) -> AppResult<ClusterProfile> {
    let id = Uuid::new_v4().to_string();
    let profile = ClusterProfile {
        id: id.clone(),
        name: profile.name,
        environment: profile.environment,
        bootstrap_servers: profile.bootstrap_servers,
    };
    let mut clusters = state.clusters.lock().expect("clusters lock");
    clusters.insert(id, profile.clone());
    Ok(profile)
}

fn get_cluster(state: &AppState, cluster_id: &str) -> AppResult<ClusterProfile> {
    state
        .clusters
        .lock()
        .expect("clusters lock")
        .get(cluster_id)
        .cloned()
        .ok_or_else(|| AppError::NotFound(format!("cluster '{cluster_id}' desconhecido")))
}

/// RF-004 + Regra de negócio 1: em qualquer cluster que não seja o local, a
/// criação de tópico vira uma [`ApprovalRequest`] pendente em vez de ser
/// efetivada na hora.
#[tauri::command]
pub fn request_topic_creation(
    state: State<AppState>,
    cluster_id: String,
    spec: TopicSpec,
    requested_by: String,
) -> AppResult<ApprovalRequest> {
    if cluster_id == LOCAL_CLUSTER_ID {
        return Err(AppError::InvalidRequest(
            "o ambiente local não usa fluxo de aprovação; use create_local_topic".into(),
        ));
    }
    let cluster = get_cluster(&state, &cluster_id)?;

    if !cluster.environment.requires_creation_approval() {
        let summary = state.kafka.create_topic(&cluster_id, &spec)?;
        return Ok(ApprovalRequest {
            id: Uuid::new_v4().to_string(),
            action: ApprovalAction::CreateTopic {
                cluster_id,
                spec: TopicSpec {
                    name: summary.name,
                    partitions: summary.partitions,
                    replication_factor: summary.replication_factor,
                    retention_ms: summary.retention_ms,
                    acl_principals: Vec::new(),
                },
            },
            requested_by,
            status: ApprovalStatus::Approved,
            decided_by: None,
        });
    }

    let request = ApprovalRequest {
        id: Uuid::new_v4().to_string(),
        action: ApprovalAction::CreateTopic { cluster_id, spec },
        requested_by,
        status: ApprovalStatus::Pending,
        decided_by: None,
    };
    state
        .approvals
        .lock()
        .expect("approvals lock")
        .insert(request.id.clone(), request.clone());
    Ok(request)
}

/// RF-004 + Regra de negócio 2: exclusão de tópico só exige aprovação em
/// produção; em teste/homologação é efetivada direto.
#[tauri::command]
pub fn request_topic_deletion(
    state: State<AppState>,
    cluster_id: String,
    topic_name: String,
    requested_by: String,
) -> AppResult<ApprovalRequest> {
    let cluster = get_cluster(&state, &cluster_id)?;

    if !cluster.environment.requires_deletion_approval() {
        state.kafka.delete_topic(&cluster_id, &topic_name)?;
        return Ok(ApprovalRequest {
            id: Uuid::new_v4().to_string(),
            action: ApprovalAction::DeleteTopic {
                cluster_id,
                topic_name,
            },
            requested_by,
            status: ApprovalStatus::Approved,
            decided_by: None,
        });
    }

    let request = ApprovalRequest {
        id: Uuid::new_v4().to_string(),
        action: ApprovalAction::DeleteTopic {
            cluster_id,
            topic_name,
        },
        requested_by,
        status: ApprovalStatus::Pending,
        decided_by: None,
    };
    state
        .approvals
        .lock()
        .expect("approvals lock")
        .insert(request.id.clone(), request.clone());
    Ok(request)
}
