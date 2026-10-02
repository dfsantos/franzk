# Franzk — Console de Administração de Clusters Kafka

App desktop (Tauri + React/TypeScript) para desenvolvedores, engenheiros de
plataforma e testadores administrarem clusters Kafka: inspeção de mensagens,
ambiente local de desenvolvimento, criação/exclusão de tópicos com
aprovação por pares em clusters compartilhados, e monitoramento de saúde do
cluster.

- **Para começar a desenvolver:** leia `CLAUDE.md` primeiro.
- **PRD:** `docs/prd/prd-kafka-admin-console.md`
- **Plano de desenvolvimento / backlog:** `docs/plan/DEVELOPMENT_PLAN.md`
- **Decisões de arquitetura:** `docs/adr/`

## Comandos

```bash
npm install

# Backend
cd src-tauri && cargo check
cd src-tauri && cargo test

# Frontend
npm run build
npm run dev          # servidor Vite isolado — invoke() do Tauri não funciona aqui

# App completo (precisa de display gráfico; não funciona em sessão cloud)
npm run tauri dev
```

No Linux, compilar `src-tauri/` exige as bibliotecas de sistema
`libwebkit2gtk-4.1-dev`, `libgtk-3-dev`, `libayatana-appindicator3-dev` e
`librsvg2-dev`. Em sessões do Claude Code, o hook
`.claude/hooks/session-start.sh` instala isso automaticamente.

## Kafka local para teste manual

```bash
docker compose up -d   # três clusters Kafka (test:9092, staging:9093, production:9094)
docker compose down -v # derruba e limpa os dados
```

Ver `docs/dev-kafka-compose.md`. O app ainda fala com um `MockKafkaClient`
em memória, não com esses brokers — ver `docs/plan/DEVELOPMENT_PLAN.md`,
trilha B.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
