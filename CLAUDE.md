# Franzk — Console de Administração de Clusters Kafka

> Este arquivo é o ponto de entrada para qualquer sessão do Claude Code neste
> repositório. Mantenha-o coerente a cada entrega: se uma decisão aqui
> descrita mudar, atualize este arquivo no mesmo commit/PR que a mudança.

## O que é este projeto

App desktop (Tauri) que unifica, para desenvolvedores, engenheiros de
plataforma e testadores, a administração de clusters Kafka: inspeção de
mensagens, ambiente local de desenvolvimento, criação/exclusão de tópicos em
clusters compartilhados (com aprovação por pares) e monitoramento de saúde
do cluster.

- **PRD completo:** `docs/prd/prd-kafka-admin-console.md`
- **Plano de desenvolvimento (fonte da verdade sobre o que falta):**
  `docs/plan/DEVELOPMENT_PLAN.md`
- **Decisões de arquitetura:** `docs/adr/`
- **Kafka local para teste manual:** `docker-compose.yml` +
  `docs/dev-kafka-compose.md`

Leia o PRD antes de implementar qualquer funcionalidade nova — ele define
regras de negócio (§6) e critérios de aceite (§10) que o código precisa
respeitar, incluindo o que está **fora de escopo** (Schema Registry) e o que
está marcado como `[PENDENTE]` (não especificado ainda, não deve ser
inventado — ver RF-003 no PRD).

## Stack e arquitetura

- **Backend:** Rust + Tauri 2, em `src-tauri/`.
- **Frontend:** React + TypeScript + Vite, em `src/`.
- **Comunicação:** comandos Tauri (`#[tauri::command]` ↔ `invoke(...)`),
  nunca HTTP/REST entre as duas camadas.
- **Persistência:** nenhuma ainda — todo estado (clusters, tópicos locais,
  aprovações) vive em memória (`AppState`) e se perde ao reiniciar o app.
  Isso é uma lacuna conhecida, não uma decisão definitiva (ver plano).

### Por que uma camada `KafkaClient` (trait) em vez de falar com o Kafka direto

`src-tauri/src/kafka/client.rs` define o trait `KafkaClient`. Hoje a única
implementação é `kafka::mock::MockKafkaClient`, um Kafka "de brinquedo" em
memória. Isso é proposital: permite que front-end, workflow de aprovação e
monitoramento sejam desenvolvidos e testados **sem depender de um broker
Kafka real ou de uma biblioteca de protocolo**, enquanto a integração real
(RF-001/002/004/005 contra um broker de fato) é feita em paralelo atrás do
mesmo contrato. Ver trilha "Integração Kafka real" no plano antes de trocar
o mock por uma implementação real — e **não** adicione `rdkafka` sem ler a
limitação de ambiente abaixo.

### Mapa de módulos (backend)

```
src-tauri/src/
  domain/       tipos de domínio puros (Cluster, Topic, Message, Approval, Monitoring)
  kafka/        trait KafkaClient + implementação mock
  commands/     #[tauri::command] por área funcional, um arquivo por RF
  state.rs      AppState gerenciado pelo Tauri (clusters, aprovações, kafka client)
  error.rs      AppError único, serializado como { kind, message } para o front
  lib.rs        registro de plugins/comandos — toda função nova em commands/
                precisa ser adicionada no invoke_handler! aqui
```

### Mapa de módulos (frontend)

```
src/
  lib/types.ts    espelha src-tauri/src/domain/*.rs (mantenha os dois em sincronia manualmente)
  lib/api.ts      um wrapper de invoke() por comando Tauri, agrupado por área
  lib/errors.ts   extrai a mensagem de AppErrorPayload
  app/            layout (sidebar + rotas)
  features/       uma pasta por área funcional (messages, local-env, clusters, approvals, monitoring)
```

Regra de nomenclatura: comandos Rust usam `snake_case`; todo struct exposto
ao front tem `#[serde(rename_all = "camelCase")]` para o JSON chegar em
`camelCase` no TypeScript. Novo tipo exposto a um comando → já nasce com esse
atributo, para não misturar convenções na mesma API.

## Regras de negócio críticas (não flexibilizar sem atualizar o PRD)

1. **Criação de tópico em ambiente compartilhado** (test/staging/produção)
   exige aprovação de outro engenheiro de plataforma — nunca do próprio
   solicitante. Implementado em `Environment::requires_creation_approval` +
   `commands::approvals::decide_approval`.
2. **Exclusão de tópico em produção** exige a mesma aprovação por pares;
   em outros ambientes compartilhados a exclusão é direta. Implementado em
   `Environment::requires_deletion_approval`.
3. **Ambiente local nunca passa por aprovação** — é a máquina do próprio
   desenvolvedor.

Essas regras têm teste unitário em `src-tauri/src/domain/tests.rs` e em
`commands::approvals::decide_approval` (rejeita `decided_by == requested_by`).
Qualquer mudança nessas regras precisa atualizar PRD §6, os testes e este
arquivo no mesmo PR.

## Limitação importante do ambiente de build

Tauri no Linux precisa de `libwebkit2gtk-4.1-dev`, `libgtk-3-dev`,
`libayatana-appindicator3-dev` e `librsvg2-dev` do sistema para `cargo
check`/`cargo build` funcionarem — sem isso, falha com
`Package 'gdk-3.0' was not found` via pkg-config. Essas libs **não** vêm
pré-instaladas em um container de sessão cloud novo. O hook
`.claude/hooks/session-start.sh` instala tudo isso automaticamente no início
da sessão (ver `.claude/settings.json`); se você estiver fora desse fluxo,
rode o hook manualmente ou instale as libs com apt antes de tocar em
`src-tauri/`.

**Sessões cloud não têm display gráfico.** `cargo tauri dev`/`cargo tauri
build` compilam, mas abrir a janela de fato (teste visual) só é possível na
máquina de um desenvolvedor com GUI. Nesta sessão de setup, a verificação
que foi feita é: `cargo check`, `cargo test` (backend) e `npm run build`
(`tsc` + `vite build`) — **não** uma verificação visual da UI. Trate isso
como pendência ao avaliar qualquer entrega de UI feita em sessão cloud.

## Comandos

```bash
# Backend
cd src-tauri && cargo check        # mais rápido para validar que compila
cd src-tauri && cargo test         # roda os testes de domínio e do mock

# Frontend
npm run build                      # tsc --noEmit (via build) + vite build
npm run dev                        # servidor Vite (sem IPC real do Tauri;
                                    # invoke() só funciona dentro do webview
                                    # do Tauri, não em um browser solto)

# App completo (requer display gráfico — não funciona em sessão cloud)
npm run tauri dev
```

## Convenções de código

- Comentários no Rust e no TS: só quando o "porquê" não é óbvio (ex.:
  por que RF-003 não tem implementação, por que `HashRouter` em vez de
  `BrowserRouter`). Não narrar o que o código já diz pelo nome.
- Toda struct/enum nova exposta a um comando Tauri precisa de tipo TS
  correspondente em `src/lib/types.ts` no mesmo PR.
- Todo comando novo precisa: função em `commands/<area>.rs`, registro em
  `lib.rs`, wrapper em `src/lib/api.ts`. Esses três arquivos são pontos de
  concorrência entre agents/PRs em paralelo — prefira diffs pequenos e
  aditivos (uma linha nova, não reordenar o que já existe) para reduzir
  conflito de merge.

## Agentes especializados

Ver `.claude/agents/`. Use o agente cuja especialidade corresponde à área do
arquivo que você vai tocar (`rust-backend` para `src-tauri/src/{domain,commands,state,error}.rs`,
`kafka-integration` para `src-tauri/src/kafka/`, `frontend-react` para `src/`,
`qa-test-engineer` para testes/critérios de aceite, `docs-plan-keeper` para
manter este arquivo e o plano atualizados).

## Skills

Ver `.claude/skills/`:
- `add-tauri-command`: passo a passo para adicionar um comando Tauri de
  ponta a ponta (Rust → registro → binding TS) sem esquecer nenhuma ponta.
- `sync-plan-and-claude-md`: checklist para atualizar `DEVELOPMENT_PLAN.md`
  e este arquivo depois de concluir uma entrega.

## Estado atual (atualize a cada marco)

- [x] Scaffold Tauri (React+TS+Vite) criado e com identificador próprio
      (`dev.franzk.app` — nunca usar identificadores de empresas/clientes
      não mencionados explicitamente na conversa).
- [x] Domínio completo modelado (Cluster/Environment, Topic, Message+Filter,
      Approval, Monitoring) com as regras de negócio 1 e 2 implementadas.
- [x] `MockKafkaClient` cobrindo as 5 áreas funcionais do PRD com dados em
      memória.
- [x] 13 comandos Tauri expostos e testados via `cargo check`/`cargo test`.
- [x] Frontend com 5 páginas (uma por módulo do PRD) consumindo os comandos.
- [x] `docker-compose.yml` com três clusters Kafka reais (test/staging/
      production) para teste manual — validado nesta sessão (sobem
      `healthy`, produce/consume reais funcionam). O app ainda **não** fala
      com eles: continua no `MockKafkaClient` até a trilha B fechar.
- [ ] Integração real com um broker Kafka (ver plano, trilha B).
- [ ] Persistência de clusters/tópicos/aprovações entre reinícios.
- [ ] Autenticação/identidade de usuário (hoje `requestedBy`/`decidedBy` são
      campos de texto livre digitados na UI).
- [ ] RF-003 (simulação de erro/reprocessamento) — aguarda especificação,
      está `[PENDENTE]` no próprio PRD.
