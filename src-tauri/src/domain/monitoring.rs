use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BrokerState {
    Online,
    Offline,
    Degraded,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BrokerStatus {
    pub broker_id: u32,
    pub host: String,
    pub state: BrokerState,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PartitionInfo {
    pub topic: String,
    pub partition: u32,
    pub leader_broker_id: u32,
    pub in_sync_replicas: u16,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsumerGroupLag {
    pub group_id: String,
    pub topic: String,
    pub partition: u32,
    pub lag: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThroughputSample {
    pub messages_per_second: f64,
    pub bytes_per_second: f64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterHealth {
    pub brokers: Vec<BrokerStatus>,
    pub partitions: Vec<PartitionInfo>,
    pub consumer_lag: Vec<ConsumerGroupLag>,
    pub throughput: ThroughputSample,
}
