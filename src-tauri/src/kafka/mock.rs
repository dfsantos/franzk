use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::domain::message::{KafkaMessage, MessageFilter};
use crate::domain::monitoring::{
    BrokerState, BrokerStatus, ClusterHealth, ConsumerGroupLag, PartitionInfo, ThroughputSample,
};
use crate::domain::topic::{TopicSpec, TopicSummary};
use crate::error::{AppError, AppResult};
use crate::kafka::client::KafkaClient;

struct TopicRecord {
    spec: TopicSpec,
    messages: Vec<KafkaMessage>,
}

/// Implementação em memória de [`KafkaClient`], usada hoje para o ambiente
/// Local (RF-002/RF-003) e como stand-in dos clusters compartilhados
/// enquanto a integração real (rdkafka/rskafka) não é feita. Nenhum dado
/// sobrevive ao encerramento do app.
#[derive(Default)]
pub struct MockKafkaClient {
    topics: Mutex<HashMap<String, HashMap<String, TopicRecord>>>,
}

fn now_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

impl KafkaClient for MockKafkaClient {
    fn list_topics(&self, cluster_id: &str) -> AppResult<Vec<TopicSummary>> {
        let topics = self.topics.lock().expect("mock kafka lock");
        Ok(topics
            .get(cluster_id)
            .map(|by_name| {
                by_name
                    .values()
                    .map(|record| TopicSummary {
                        cluster_id: cluster_id.to_string(),
                        name: record.spec.name.clone(),
                        partitions: record.spec.partitions,
                        replication_factor: record.spec.replication_factor,
                        retention_ms: record.spec.retention_ms,
                    })
                    .collect()
            })
            .unwrap_or_default())
    }

    fn create_topic(&self, cluster_id: &str, spec: &TopicSpec) -> AppResult<TopicSummary> {
        let mut topics = self.topics.lock().expect("mock kafka lock");
        let by_name = topics.entry(cluster_id.to_string()).or_default();
        if by_name.contains_key(&spec.name) {
            return Err(AppError::InvalidRequest(format!(
                "tópico '{}' já existe neste cluster",
                spec.name
            )));
        }
        by_name.insert(
            spec.name.clone(),
            TopicRecord {
                spec: spec.clone(),
                messages: Vec::new(),
            },
        );
        Ok(TopicSummary {
            cluster_id: cluster_id.to_string(),
            name: spec.name.clone(),
            partitions: spec.partitions,
            replication_factor: spec.replication_factor,
            retention_ms: spec.retention_ms,
        })
    }

    fn delete_topic(&self, cluster_id: &str, topic_name: &str) -> AppResult<()> {
        let mut topics = self.topics.lock().expect("mock kafka lock");
        let by_name = topics
            .get_mut(cluster_id)
            .ok_or_else(|| AppError::NotFound(format!("cluster '{cluster_id}' desconhecido")))?;
        by_name
            .remove(topic_name)
            .map(|_| ())
            .ok_or_else(|| AppError::NotFound(format!("tópico '{topic_name}' não encontrado")))
    }

    fn filter_messages(
        &self,
        cluster_id: &str,
        topic_name: &str,
        filter: &MessageFilter,
    ) -> AppResult<Vec<KafkaMessage>> {
        let topics = self.topics.lock().expect("mock kafka lock");
        let record = topics
            .get(cluster_id)
            .and_then(|by_name| by_name.get(topic_name))
            .ok_or_else(|| AppError::NotFound(format!("tópico '{topic_name}' não encontrado")))?;
        Ok(record
            .messages
            .iter()
            .rev()
            .filter(|message| filter.matches(message))
            .take(filter.limit.max(1))
            .cloned()
            .collect())
    }

    fn produce_message(
        &self,
        cluster_id: &str,
        topic_name: &str,
        key: Option<String>,
        value: String,
    ) -> AppResult<KafkaMessage> {
        let mut topics = self.topics.lock().expect("mock kafka lock");
        let record = topics
            .get_mut(cluster_id)
            .and_then(|by_name| by_name.get_mut(topic_name))
            .ok_or_else(|| AppError::NotFound(format!("tópico '{topic_name}' não encontrado")))?;
        let message = KafkaMessage {
            partition: (record.messages.len() as u32) % record.spec.partitions.max(1),
            offset: record.messages.len() as i64,
            key,
            value,
            timestamp: now_millis(),
        };
        record.messages.push(message.clone());
        Ok(message)
    }

    fn cluster_health(&self, _cluster_id: &str) -> AppResult<ClusterHealth> {
        // Dados ilustrativos até a integração real com o protocolo Kafka
        // (JMX/AdminClient) estar disponível — ver RF-005 no plano.
        Ok(ClusterHealth {
            brokers: vec![
                BrokerStatus {
                    broker_id: 1,
                    host: "broker-1:9092".into(),
                    state: BrokerState::Online,
                },
                BrokerStatus {
                    broker_id: 2,
                    host: "broker-2:9092".into(),
                    state: BrokerState::Online,
                },
            ],
            partitions: vec![PartitionInfo {
                topic: "exemplo".into(),
                partition: 0,
                leader_broker_id: 1,
                in_sync_replicas: 2,
            }],
            consumer_lag: vec![ConsumerGroupLag {
                group_id: "grupo-exemplo".into(),
                topic: "exemplo".into(),
                partition: 0,
                lag: 0,
            }],
            throughput: ThroughputSample {
                messages_per_second: 0.0,
                bytes_per_second: 0.0,
            },
        })
    }
}
