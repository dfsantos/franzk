# Kafka local para testar o Franzk (`docker-compose.yml`)

Três clusters Kafka de nó único (modo KRaft, sem Zookeeper), um para cada
ambiente compartilhado do Franzk — `test`, `staging` e `production` (ver
`Environment` em `src-tauri/src/domain/cluster.rs`). São clusters
**independentes entre si**, não um cluster de 3 brokers.

| Serviço | Porta no host | Ambiente (`Environment`) |
|---|---|---|
| `kafka-test` | `localhost:9092` | `test` |
| `kafka-staging` | `localhost:9093` | `staging` |
| `kafka-production` | `localhost:9094` | `production` |

O ambiente `local` do Franzk (RF-002/RF-003) **não** usa nada daqui: ele
roda contra o `MockKafkaClient` embutido no app, não contra um broker de
verdade.

## Subir e derrubar

```bash
docker compose up -d      # sobe os três, ~15-20s até ficarem "healthy"
docker compose ps         # confirma o status de cada um
docker compose down -v    # derruba e apaga os dados (recomeça do zero)
```

Cada serviço tem `healthcheck` via `kafka-broker-api-versions.sh`; espere
aparecer `healthy` antes de tentar conectar.

## Testar manualmente (CLI, sem o Franzk)

O bootstrap precisa ser feito como um cliente **de fora** do Docker (mesmo
ponto de vista que o Franzk vai ter rodando no host) — por isso aqui usamos
um container descartável com `--network host`, não `docker exec` num dos
serviços (dentro do container o listener anunciado aponta para a porta do
host, que não existe na rede interna dele — ver nota técnica abaixo):

```bash
docker run --rm --network host apache/kafka:3.9.0 \
  /opt/kafka/bin/kafka-topics.sh --bootstrap-server localhost:9093 \
  --create --topic smoke --partitions 1 --replication-factor 1

docker run --rm -i --network host apache/kafka:3.9.0 \
  /opt/kafka/bin/kafka-console-producer.sh --bootstrap-server localhost:9093 --topic smoke
# digite uma linha, Ctrl+D

docker run --rm --network host apache/kafka:3.9.0 \
  /opt/kafka/bin/kafka-console-consumer.sh --bootstrap-server localhost:9093 \
  --topic smoke --from-beginning --max-messages 1
```

Troque `9093` por `9092` (test) ou `9094` (production) conforme o cluster.

## Testar com o Franzk

Hoje isso **não está fechado de ponta a ponta** — duas lacunas conhecias:

1. **Trilha B do plano (integração Kafka real) ainda não existe.** O app
   fala com `MockKafkaClient` independente do `bootstrapServers` cadastrado
   num `ClusterProfile`. Subir estes containers não faz o Franzk conversar
   com eles até a trilha B ser implementada — isso aqui é a infraestrutura
   para quando ela existir (e para testar o protocolo manualmente via CLI
   enquanto isso).
2. **Não há UI para cadastrar um `ClusterProfile` ainda** (trilha C1). O
   comando `add_cluster_profile` já existe no backend; para registrar um
   cluster hoje sem UI, é preciso chamar o comando diretamente (ex. um
   teste de integração temporário) com algo como:
   ```json
   { "name": "Teste", "environment": "test", "bootstrapServers": "localhost:9092" }
   ```

## Nota técnica: por que `docker exec` dentro do container engana

Cada serviço anuncia (`advertised.listeners`) o endereço que um cliente
**externo** deve usar para falar com ele depois do bootstrap — aqui,
`localhost:<porta do host>`. Um cliente que já está *dentro* do container
(via `docker exec`) recebe essa mesma informação, mas `localhost:<porta do
host>` não existe na rede interna do container (que só expõe a porta
interna `9092`) — por isso o comando trava tentando reconectar. Isso não é
um bug do compose, é a troca inerente de expor nós distintos sob a mesma
porta de container através de portas de host diferentes: o endereço
anunciado só pode servir corretamente uma audiência (aqui, clientes
externos rodando no host — que é o caso do Franzk).
