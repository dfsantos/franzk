use super::client::KafkaClient;
use super::mock::MockKafkaClient;
use crate::domain::message::MessageFilter;
use crate::domain::topic::TopicSpec;

fn spec(name: &str) -> TopicSpec {
    TopicSpec {
        name: name.to_string(),
        partitions: 3,
        replication_factor: 1,
        retention_ms: 86_400_000,
        acl_principals: Vec::new(),
    }
}

#[test]
fn produce_e_filter_por_chave() {
    let client = MockKafkaClient::default();
    client.create_topic("local", &spec("pedidos")).unwrap();
    client
        .produce_message("local", "pedidos", Some("A".into()), "primeiro".into())
        .unwrap();
    client
        .produce_message("local", "pedidos", Some("B".into()), "segundo".into())
        .unwrap();

    let filter = MessageFilter {
        key_equals: Some("B".into()),
        ..Default::default()
    };
    let result = client.filter_messages("local", "pedidos", &filter).unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].value, "segundo");
}

#[test]
fn criar_topico_duplicado_falha() {
    let client = MockKafkaClient::default();
    client.create_topic("local", &spec("pedidos")).unwrap();
    assert!(client.create_topic("local", &spec("pedidos")).is_err());
}
