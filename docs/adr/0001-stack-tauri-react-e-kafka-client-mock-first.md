# ADR 0001 — Stack Tauri + React e abordagem "mock-first" para o cliente Kafka

**Status:** Aceita
**Data:** 2026-10-02

## Contexto

O PRD (`docs/prd/prd-kafka-admin-console.md`) pede um console de
administração Kafka multiplataforma, com paridade de uso entre
desenvolvedor (ambiente local), engenheiro de plataforma (clusters
compartilhados) e testador (inspeção de mensagens). O pedido explícito foi
usar Tauri. O desenvolvimento precisa ser paralelizável entre múltiplos
agentes/sessões do Claude Code, incluindo sessões cloud sem acesso a um
broker Kafka real nem a display gráfico.

## Decisão

1. **Tauri 2 + Rust (backend) + React/TypeScript/Vite (frontend)**, scaffold
   oficial (`create-tauri-app`, template `react-ts`).
2. **Toda lógica de domínio e regra de negócio em Rust**
   (`src-tauri/src/domain/`), nunca duplicada como fonte de verdade na UI.
3. **Acesso a Kafka abstraído por um trait** (`kafka::KafkaClient`), com uma
   implementação em memória (`kafka::mock::MockKafkaClient`) como primeira
   e única implementação nesta sessão de setup. A integração real com o
   protocolo (rdkafka/rskafka contra um broker de fato) é tratada como uma
   trilha própria e paralela (trilha B do plano), não um bloqueador para as
   demais.
4. Identificador do app: `dev.franzk.app` — neutro, sem nome de empresa ou
   cliente, por não haver menção a isso no pedido original.

## Alternativas consideradas

- **Implementar direto contra `rdkafka`** desde o início: descartado porque
  `rdkafka` depende de `librdkafka` via bindings C, exigindo toolchain C
  (cmake, libssl-dev, etc.) que não há garantia de existir em todo ambiente
  de build (incluindo sessões cloud do Claude Code). Isso bloquearia
  qualquer trabalho em paralelo até a integração real estar pronta.
- **Sem camada de abstração**, chamando a lib de Kafka direto dos
  comandos: descartado porque acopla toda a superfície de comandos/
  frontend à disponibilidade de um broker real, impedindo testar
  front-end, aprovação e monitoramento de forma isolada.

## Consequências

- Positivo: front-end, workflow de aprovação, monitoramento e testes podem
  avançar sem depender de um broker Kafka real ou de decidir a biblioteca
  de protocolo.
- Positivo: a troca do mock pela implementação real fica isolada em
  `kafka::real` (ou nome equivalente), sem tocar em `commands/` nem no
  frontend, já que ambos dependem apenas do trait.
- Negativo: até a trilha B ser concluída, nenhuma operação é "real" —
  qualquer demonstração do app hoje opera sobre dados em memória que não
  sobrevivem a um restart. Isso está documentado em `CLAUDE.md` e no plano
  como gap conhecido, não como entrega finalizada.
- Negativo: builds Linux do Tauri (mesmo só com o mock) exigem bibliotecas
  de sistema GTK/WebKit; mitigado com o hook
  `.claude/hooks/session-start.sh`.
