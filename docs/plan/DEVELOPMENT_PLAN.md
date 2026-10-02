# Plano de Desenvolvimento — Franzk

> Fonte da verdade sobre o que já existe, o que falta e em que ordem/paralelismo
> fazer. Atualize este arquivo (e o "Estado atual" do `CLAUDE.md`) a cada
> entrega concluída — ver skill `sync-plan-and-claude-md`.

Referência: `docs/prd/prd-kafka-admin-console.md`. Os RFs e regras de negócio
citados abaixo (RF-001..RF-005, RB-1, RB-2) são os mesmos do PRD, seções 6 e 7.

## 1. Arquitetura (decidida nesta sessão de setup)

- Tauri 2 + Rust no backend (`src-tauri/`), React+TS+Vite no frontend (`src/`).
- Toda a lógica de domínio e as regras de negócio vivem no Rust
  (`src-tauri/src/domain/`), não na UI — a UI só exibe e coleta input.
- Acesso a Kafka é abstraído por um trait (`kafka::KafkaClient`) com uma
  implementação mock em memória (`kafka::mock::MockKafkaClient`). Isso é o
  que permite paralelizar: quem trabalha em front-end, aprovação ou
  monitoramento não espera a integração real com o protocolo Kafka.
- Sem persistência ainda: `AppState` é tudo em memória. Primeira prioridade
  de hardening antes de qualquer piloto com usuário real.
- Sem autenticação: `requestedBy`/`decidedBy` são texto livre digitado na
  UI. Isso **não** cumpre a regra de negócio 1/2 de forma segura (qualquer
  um pode se auto-aprovar digitando um nome diferente) — é aceitável para
  prototipagem, mas é um gap de segurança a resolver antes de produção.

### Limitação de ambiente (leia antes de tocar em `src-tauri/`)

Builds Linux do Tauri precisam de `libwebkit2gtk-4.1-dev`, `libgtk-3-dev`,
`libayatana-appindicator3-dev`, `librsvg2-dev` do sistema. Um container de
sessão cloud novo não tem isso por padrão — o hook
`.claude/hooks/session-start.sh` resolve isso automaticamente. Sessões cloud
também não têm display gráfico: `cargo tauri dev`/`build` compilam, mas abrir
a janela só é possível numa máquina com GUI. Teste visual da UI é uma
pendência que só pode ser fechada fora do ambiente cloud.

## 2. O que já foi entregue nesta sessão de setup

- Domínio completo: `ClusterProfile`/`Environment`, `TopicSpec`/`TopicSummary`,
  `KafkaMessage`/`MessageFilter`, `ApprovalRequest`/`ApprovalAction`, tipos de
  monitoramento. Regras de negócio 1 e 2 como métodos de `Environment`, com
  teste unitário.
- `MockKafkaClient`: create/delete/list topic, produce/filter message,
  cluster health fixture. Testado.
- 13 comandos Tauri cobrindo as 5 áreas do PRD (mensagens, ambiente local,
  clusters compartilhados + aprovação, aprovações, monitoramento).
- Frontend com shell de navegação e uma página por área, todas consumindo os
  comandos via `src/lib/api.ts`.
- `cargo check`/`cargo test` e `npm run build` validados nesta sessão.
  **Não validado:** execução visual do app (ver limitação de ambiente).

Isso cobre o "esqueleto" dos RF-001, RF-002, RF-004 e RF-005. RF-003 foi
deliberadamente **não implementado**: o próprio PRD marca o comportamento
esperado como `[PENDENTE]` (não especificado na entrevista) — implementar
agora seria inventar requisito. Ver trilha F.

## 3. Trilhas de trabalho e paralelização

Cada trilha tem um agente sugerido (`.claude/agents/`) e pode rodar em uma
sessão própria. "Depende de" lista o que precisa já estar na branch
principal antes de começar; o resto é paralelizável livremente.

| Trilha | Agente | Depende de | Pode rodar em paralelo com |
|---|---|---|---|
| A — Core backend / persistência | `rust-backend` | — (base já existe) | B, C, D, E, F |
| B — Integração Kafka real | `kafka-integration` | — (trabalha atrás do trait, não precisa esperar nada) | A, C, D, E, F |
| C — Frontend / UX | `frontend-react` | — (trabalha contra o mock) | A, B, D, E, F |
| D — Workflow de aprovação (hardening) | `rust-backend` + `frontend-react` | A1 (persistência) para D2 | B, C, E, F |
| E — Qualidade / testes | `qa-test-engineer` | superfície de A/B/C existir para o que for testar | A, B, C, D |
| F — Documentação / PRD | `docs-plan-keeper` | — | todas |

**Pontos de conflito a vigiar** (arquivos tocados por várias trilhas, prefira
diffs pequenos e aditivos): `src-tauri/src/lib.rs` (registro de comandos),
`src-tauri/Cargo.toml` e `package.json` (dependências), `src/lib/api.ts` e
`src/lib/types.ts` (bindings).

## 4. Backlog em entregas pequenas

Cada item deve ser uma entrega fechável em até ~1 dia de trabalho de agente.

### Trilha A — Core backend / persistência

- [ ] **A1.** Persistir `clusters` e `approvals` em disco (arquivo JSON no
      diretório de dados do app via `tauri::Manager::path()`), substituindo
      o `HashMap` em memória. Cobre a perda de estado a cada reinício.
- [ ] **A2.** Comando `remove_cluster_profile` + proteção contra remover
      cluster com tópicos pendentes de aprovação.
- [ ] **A3.** Validação de `TopicSpec` no backend (nome não vazio,
      partições ≥ 1, fator de replicação ≥ 1) retornando `AppError::InvalidRequest`
      com mensagem específica por campo.
- [ ] **A4.** Paginação em `filter_messages`/`consume_local_topic` (hoje é
      só `limit`; falta um cursor/offset para "próxima página").

### Trilha B — Integração Kafka real

- [ ] **B1 (ADR).** Decidir `rskafka` (pure Rust, sem dependência de
      `librdkafka`/cmake) vs `rdkafka` (bindings C, mais maduro mas exige
      toolchain C disponível no ambiente de build). Registrar em
      `docs/adr/0001-cliente-kafka.md`. Dado que o ambiente cloud não tem
      garantia de `librdkafka`/cmake instalados, a recomendação de partida é
      `rskafka` — valide antes de divergir.
- [x] **B2a.** `docker-compose.yml` com três clusters Kafka de nó único
      (KRaft), um por ambiente compartilhado (`test`:9092, `staging`:9093,
      `production`:9094) para testar manualmente contra um broker real
      enquanto a trilha B não fecha — ver `docs/dev-kafka-compose.md`.
      **Atenção:** subir isso não faz o Franzk falar com Kafka de verdade;
      o app ainda usa `MockKafkaClient` até B2b/B3/B4 serem feitos.
- [ ] **B2b.** Implementar `RealKafkaClient: KafkaClient` (`list_topics`,
      `create_topic`, `delete_topic`) contra os brokers de
      `docker-compose.yml` (trilha B2a).
- [ ] **B3.** Implementar `filter_messages`/`produce_message` reais
      (consumer com seek por timestamp quando `timestampFrom` é informado).
- [ ] **B4.** Implementar `cluster_health` real: estado de brokers via
      `AdminClient::describe_cluster`, lag via `DescribeConsumerGroups` +
      offsets, throughput via amostragem de bytes/mensagens por partição
      em uma janela de tempo.
- [ ] **B5.** Trocar `MockKafkaClient` por `RealKafkaClient` em `AppState`
      por trás de uma config (cluster local continua podendo usar mock para
      testes, ou passa a apontar para um Kafka local real — decidir em B1).

### Trilha C — Frontend / UX

- [ ] **C1.** Formulário de cadastro de `ClusterProfile` (nome, ambiente,
      bootstrap servers) na página de Clusters — hoje só lista, não tem
      como adicionar pela UI (`add_cluster_profile` já existe no backend).
- [ ] **C2.** Paginação/scroll incremental na tabela de mensagens de
      `MessagesPage` (depende de A4 para ter cursor real).
- [ ] **C3.** Página de histórico de aprovações (approved/rejected, não só
      pendentes) — novo comando `list_approval_history` + tela.
- [ ] **C4.** Gráficos de throughput/lag em `MonitoringPage` (biblioteca de
      charts leve, ex. `recharts` — avaliar tamanho de bundle antes de
      adicionar).
- [ ] **C5.** Testes de acessibilidade básicos (labels em todo input,
      navegação por teclado no menu lateral).

### Trilha D — Workflow de aprovação (hardening)

- [ ] **D1.** Identidade real de usuário (sessão local autenticada ou, no
      mínimo, um "usuário atual" configurado uma vez e persistido, em vez de
      campo de texto livre a cada ação) — pré-requisito para a regra de
      negócio 1/2 valerem de verdade.
- [ ] **D2.** Persistir decisão de aprovação com timestamp e motivo opcional
      de rejeição (depende de A1).
- [ ] **D3.** Notificação (toast/badge no menu) quando uma nova aprovação
      fica pendente.

### Trilha E — Qualidade / testes

- [ ] **E1.** Testes de integração dos comandos Tauri (via `tauri::test`
      ou chamando as funções de `commands::*` diretamente com um `AppState`
      de teste) cobrindo os casos de borda do PRD §7 (nenhuma mensagem
      encontrada, tópico duplicado, exclusão sem aprovação em prod).
- [ ] **E2.** Testes de componente frontend (Vitest + Testing Library) para
      `ApprovalsPage` (não deixar aprovar quando `decidedBy == requestedBy`)
      e `MessagesPage` (filtro combinando conteúdo + chave).
- [ ] **E3.** Checklist de aceite do PRD §10 mapeado 1:1 para testes
      automatizados ou, onde não for possível (ex. teste visual do app),
      para um roteiro manual documentado.

### Trilha F — Documentação / esclarecimento de PRD

- [ ] **F1.** ADR de cada decisão técnica relevante em `docs/adr/`
      (numeradas, formato curto: contexto/decisão/consequências).
- [ ] **F2.** Levar RF-003 (simulação de erro/reprocessamento, hoje
      `[PENDENTE]`) para uma nova rodada de entrevista com o stakeholder
      antes de qualquer implementação — usar a skill
      `anthropic-skills:prd-interviewer` para atualizar o PRD, não
      adivinhar o comportamento.
    - a implementação (comando backend + tela) só começa depois do PRD ser
      atualizado.
- [ ] **F3.** Avaliar o risco registrado no PRD §9 (dados sensíveis visíveis
      na inspeção de mensagens) quando a trilha D1 (identidade real) estiver
      pronta — decidir se precisa de mascaramento antes do primeiro piloto
      com dados reais, mesmo estando fora do escopo da v1.

## 5. Critérios de aceite (ligados ao PRD §10)

| Critério do PRD | Status | Onde |
|---|---|---|
| Filtrar mensagens por conteúdo/chave/timestamp (RF-001) | Esqueleto pronto (mock) | `commands::messages::filter_messages` |
| Criar/consumir tópico local (RF-002) | Esqueleto pronto (mock) | `commands::local_env` |
| Criar tópico compartilhado com aprovação por pares (RF-004, RB-1) | Esqueleto pronto (mock) | `commands::clusters::request_topic_creation` + `commands::approvals::decide_approval` |
| Exclusão em produção exige aprovação (RB-2) | Esqueleto pronto (mock) | `commands::clusters::request_topic_deletion` |
| Lag/brokers/partições/throughput (RF-005) | Fixture estática (não real) | `commands::monitoring::get_cluster_health` |

"Esqueleto pronto (mock)" = a regra de negócio e o fluxo estão implementados
e testados, mas sobre dados em memória, não um cluster Kafka real — fechar
de verdade depende da trilha B.
