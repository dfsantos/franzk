use crate::domain::message::{KafkaMessage, MessageFilter};
use crate::domain::monitoring::ClusterHealth;
use crate::domain::topic::{TopicSpec, TopicSummary};
use crate::error::AppResult;

/// Fronteira entre a lógica de domínio/UI e o protocolo Kafka real.
///
/// Implementações concretas (rdkafka/rskafka, apontando para um broker real)
/// e a implementação mock (`mock::MockKafkaClient`, em memória) satisfazem o
/// mesmo contrato. Isso permite que front-end, aprovação e monitoramento
/// sejam desenvolvidos em paralelo à integração real com o protocolo Kafka —
/// ver docs/plan/DEVELOPMENT_PLAN.md, trilha "Integração Kafka".
pub trait KafkaClient: Send + Sync {
    fn list_topics(&self, cluster_id: &str) -> AppResult<Vec<TopicSummary>>;
    fn create_topic(&self, cluster_id: &str, spec: &TopicSpec) -> AppResult<TopicSummary>;
    fn delete_topic(&self, cluster_id: &str, topic_name: &str) -> AppResult<()>;
    fn filter_messages(
        &self,
        cluster_id: &str,
        topic_name: &str,
        filter: &MessageFilter,
    ) -> AppResult<Vec<KafkaMessage>>;
    fn produce_message(
        &self,
        cluster_id: &str,
        topic_name: &str,
        key: Option<String>,
        value: String,
    ) -> AppResult<KafkaMessage>;
    fn cluster_health(&self, cluster_id: &str) -> AppResult<ClusterHealth>;
}
