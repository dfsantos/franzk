use tauri::State;

use crate::domain::approval::{ApprovalAction, ApprovalDecision, ApprovalRequest, ApprovalStatus};
use crate::error::{AppError, AppResult};
use crate::state::AppState;

#[tauri::command]
pub fn list_pending_approvals(state: State<AppState>) -> AppResult<Vec<ApprovalRequest>> {
    let approvals = state.approvals.lock().expect("approvals lock");
    Ok(approvals
        .values()
        .filter(|request| request.status == ApprovalStatus::Pending)
        .cloned()
        .collect())
}

/// Regras de negócio 1 e 2 (PRD §6): a aprovação é sempre por pares — quem
/// decide não pode ser quem solicitou.
#[tauri::command]
pub fn decide_approval(
    state: State<AppState>,
    request_id: String,
    decision: ApprovalDecision,
    decided_by: String,
) -> AppResult<ApprovalRequest> {
    let mut approvals = state.approvals.lock().expect("approvals lock");
    let request = approvals
        .get_mut(&request_id)
        .ok_or_else(|| AppError::NotFound(format!("solicitação '{request_id}' não encontrada")))?;

    if request.status != ApprovalStatus::Pending {
        return Err(AppError::InvalidRequest(
            "esta solicitação já foi decidida".into(),
        ));
    }
    if request.requested_by == decided_by {
        return Err(AppError::Forbidden(
            "quem solicitou a ação não pode aprová-la (revisão por pares)".into(),
        ));
    }

    match decision {
        ApprovalDecision::Reject => {
            request.status = ApprovalStatus::Rejected;
        }
        ApprovalDecision::Approve => {
            match &request.action {
                ApprovalAction::CreateTopic { cluster_id, spec } => {
                    state.kafka.create_topic(cluster_id, spec)?;
                }
                ApprovalAction::DeleteTopic {
                    cluster_id,
                    topic_name,
                } => {
                    state.kafka.delete_topic(cluster_id, topic_name)?;
                }
            }
            request.status = ApprovalStatus::Approved;
        }
    }
    request.decided_by = Some(decided_by);
    Ok(request.clone())
}
