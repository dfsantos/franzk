# PRD — Console de Administração de Clusters Kafka

**Status:** Rascunho v1
**Versão:** v1.0
**Data:** 2026-09-26
**Autor / Responsável:** Diego

> **Legenda:** [INFERIDO] = dedução do agente, não confirmada pelo stakeholder · [PENDENTE] = informação não obtida na entrevista.

## 1. Visão Geral e Proposta de Valor (Product Overview)

Console de administração para clusters Kafka que unifica, em uma única ferramenta, o suporte a três perfis de uso: o desenvolvedor que opera Kafka em ambiente local durante a implementação de sistemas, o engenheiro de plataforma que opera clusters compartilhados (teste, homologação e produção), e o testador que precisa inspecionar tópicos e mensagens. Hoje, na ausência de uma ferramenta consolidada, cada pessoa resolve essas necessidades do seu próprio jeito (CLI, scripts avulsos), gerando retrabalho e perda de produtividade — especialmente na tarefa mais custosa: localizar mensagens específicas para debugar um tópico.

## 2. Declaração do Problema e Contexto (Problem Statement)

### O Problema

Não existe uma ferramenta consolidada para operar Kafka nos diferentes contextos de uso (local e ambientes compartilhados). Cada pessoa resolve à sua maneira, com uma mistura de CLIs e scripts próprios. A tarefa que mais consome tempo hoje é a inspeção de mensagens de um tópico para fins de debug: mesmo sabendo qual tópico investigar, não há forma direta de filtrar por conteúdo, chave ou timestamp, o que torna a busca manual e improvisada — tanto em ambiente local quanto em ambientes compartilhados.

### Cenários de Uso Principais (Key Use Cases)

- **Inspeção de mensagens para debug:** a pessoa (desenvolvedor ou testador) já sabe em qual tópico está o problema, mas precisa localizar a mensagem específica filtrando por conteúdo, chave ou timestamp, sem recorrer a scripts improvisados.
- **Criação de tópico em ambiente compartilhado:** o engenheiro de plataforma cria um tópico definindo nome, partições, fator de replicação, retenção e permissões de acesso; a criação passa por aprovação de outro engenheiro de plataforma antes de ser efetivada. [INFERIDO]
- **Desenvolvimento local:** o desenvolvedor cria e consome tópicos em um Kafka local com configuração mínima (nome, partições, retenção), sem a fricção de configurar um ambiente compartilhado. [INFERIDO]

## 3. Personas e Públicos Impactados

- **Desenvolvedor:** opera Kafka em ambiente local enquanto implementa seu sistema. Precisa criar e consumir tópicos locais com pouca fricção de configuração, inspecionar mensagens no próprio ambiente, e simular cenários de erro ou reprocessamento antes de levar sua implementação a outros ambientes — as três necessidades com peso equivalente.
- **Engenheiro de plataforma:** responsável por operar clusters compartilhados (teste, homologação, produção). Precisa monitorar a saúde do cluster (lag de consumers, estado dos brokers, partições, throughput) e é também responsável por criar/configurar tópicos e gerenciar permissões de acesso nesses ambientes.
- **Testador:** precisa inspecionar detalhes de tópicos e o conteúdo de mensagens, tipicamente para validar comportamento ou investigar problemas relatados.

## 4. Objetivos e Métricas de Sucesso (Goals & Success Metrics)

### Objetivos Principais

1. Substituir a mistura atual de scripts e ferramentas improvisadas por um console único, adotado como ferramenta principal pelos desenvolvedores e engenheiros de plataforma.
2. Reduzir o esforço para localizar mensagens específicas em um tópico durante atividades de debug.

### Métricas e Metas

| Objetivo | Métrica / KPI | Meta Esperada |
|---|---|---|
| Substituir ferramentas improvisadas pelo console | % de desenvolvedores e engenheiros de plataforma usando o console como ferramenta principal | ≥ 70% em 3 meses após o lançamento |

## 5. Escopo e Fronteiras (Scope & Non-Goals)

### Incluso (In-Scope — O que será construído)

- Inspeção de mensagens de um tópico com filtro por conteúdo, chave e timestamp
- Criação e configuração de tópicos e gerenciamento de permissões de acesso em ambientes compartilhados, sujeitas a aprovação por pares
- Monitoramento de saúde do cluster: lag de consumers, estado dos brokers, partições e throughput
- Gerenciamento de Kafka local para desenvolvimento: criação/consumo de tópicos com configuração mínima e simulação de cenários de erro/reprocessamento

### Fora de Escopo (Non-Goals — O que NÃO será construído nesta versão)

- Gerenciamento de schemas (Schema Registry / Avro / Protobuf)

## 6. Regras de Negócio Críticas e Invariantes

1. **Aprovação na criação de tópico:** em ambientes compartilhados (teste, homologação, produção), a criação de um tópico só é efetivada após aprovação de outro engenheiro de plataforma (revisão por pares).
2. **Aprovação na exclusão de tópico em produção:** a exclusão de um tópico em ambiente de produção só é efetivada após aprovação de outro engenheiro de plataforma.

## 7. Requisitos Funcionais (RFs) e Histórias de Usuário

### Módulo: Inspeção de Mensagens

#### RF-001: Filtro de mensagens por conteúdo, chave ou timestamp (Prioridade: P0)

**História de Usuário:** Como testador ou desenvolvedor, quero filtrar mensagens de um tópico por conteúdo, chave ou timestamp, para localizar rapidamente a mensagem que preciso debugar sem varrer o tópico manualmente.

**Comportamento Esperado:**
- O usuário seleciona um tópico já conhecido.
- O usuário aplica um ou mais filtros (conteúdo, chave, timestamp) sobre as mensagens do tópico.
- O sistema retorna a(s) mensagem(ns) que atendem ao filtro.

**Exceções e Casos de Borda:**
- Nenhuma mensagem encontrada com o filtro aplicado. [INFERIDO]
- Volume de mensagens muito grande para varrer no intervalo de tempo informado. [INFERIDO]

### Módulo: Ambiente Local

#### RF-002: Gerenciamento de tópicos em Kafka local (Prioridade: P0)

**História de Usuário:** Como desenvolvedor, quero criar e consumir tópicos em um ambiente Kafka local com configuração mínima (nome, partições, retenção), para trabalhar na minha implementação sem fricção.

**Comportamento Esperado:**
- O usuário cria um tópico local informando nome, número de partições e política de retenção.
- O usuário consome mensagens do tópico criado.

**Exceções e Casos de Borda:**
- Tópico com nome já existente localmente. [INFERIDO]

#### RF-003: Simulação de cenários de erro e reprocessamento (Prioridade: P1) [INFERIDO]

**História de Usuário:** Como desenvolvedor, quero simular cenários de erro ou reprocessamento de mensagens no meu ambiente local, para validar o comportamento do meu sistema antes de subir para outros ambientes.

**Comportamento Esperado:**
- [PENDENTE] — o comportamento detalhado desta simulação (quais tipos de erro, como o reprocessamento é disparado) não foi especificado na entrevista.

**Exceções e Casos de Borda:**
- [PENDENTE]

### Módulo: Administração de Clusters Compartilhados

#### RF-004: Criação e configuração de tópicos com aprovação (Prioridade: P0)

**História de Usuário:** Como engenheiro de plataforma, quero criar tópicos definindo nome, partições, fator de replicação, retenção e permissões de acesso, com aprovação de outro engenheiro de plataforma, para garantir governança em ambientes compartilhados.

**Comportamento Esperado:**
- O engenheiro de plataforma preenche nome, partições, fator de replicação, retenção e permissões de acesso do novo tópico.
- A solicitação é enviada para aprovação de outro engenheiro de plataforma.
- Após aprovação, o tópico é efetivamente criado no cluster.

**Exceções e Casos de Borda:**
- Solicitação de criação rejeitada pelo aprovador. [INFERIDO]
- Solicitação de exclusão de tópico em produção também segue o mesmo fluxo de aprovação (regra de negócio 2).

### Módulo: Monitoramento de Cluster

#### RF-005: Monitoramento de saúde do cluster (Prioridade: P0)

**História de Usuário:** Como engenheiro de plataforma, quero visualizar lag de consumers, estado dos brokers, partições e throughput, para operar o cluster com visibilidade e agir antes de incidentes.

**Comportamento Esperado:**
- O engenheiro de plataforma visualiza, para um cluster selecionado, o lag de consumers, o estado dos brokers, as partições e o throughput.

**Exceções e Casos de Borda:**
- Broker indisponível ou não respondendo no momento da consulta. [INFERIDO]

## 8. Restrições e Expectativas de Negócio

- **Períodos Críticos:** [PENDENTE] — existe um período crítico do negócio em que o console não pode falhar, mas o período específico não foi identificado na entrevista.
- **Volume e Intensidade de Uso:** [PENDENTE] — não abordado na entrevista.
- **Dados Sensíveis e Acesso:** as mensagens inspecionadas podem conter dados sensíveis ou pessoais, dependendo do tópico/sistema de origem. O mascaramento desse conteúdo não é obrigatório nesta versão (ver risco na seção 9); o controle de quem acessa cada tópico se dá pelas permissões definidas em RF-004.
- **Regulação e Compliance:** [PENDENTE] — não abordado na entrevista.
- **Continuidade da Operação:** [PENDENTE] — não abordado na entrevista.

## 9. Premissas, Riscos e Mitigações (Assumptions & Risks)

- **Premissa:** não há dependência de outra área ou decisão externa (como Schema Registry ou políticas de permissão pré-existentes) para que este plano funcione.
- **Risco Registrado:** mensagens com dados sensíveis ou pessoais podem ficar visíveis durante a inspeção, já que o mascaramento de conteúdo não está no escopo desta versão → **Mitigação:** restringir o acesso a tópicos sensíveis por meio das permissões definidas em RF-004, e reavaliar a necessidade de mascaramento em versão futura. [INFERIDO]

## 10. Critérios de Aceitação (Definition of Done)

- [ ] O usuário consegue filtrar mensagens de um tópico por conteúdo, chave ou timestamp e localizar a mensagem desejada (RF-001)
- [ ] O desenvolvedor consegue criar e consumir tópicos em ambiente local com nome, partições e retenção configuráveis (RF-002)
- [ ] O engenheiro de plataforma consegue criar um tópico em ambiente compartilhado definindo nome, partições, fator de replicação, retenção e permissões — e a criação só é efetivada após aprovação de outro engenheiro de plataforma (RF-004, regra de negócio 1)
- [ ] A exclusão de tópico em produção só é efetivada após aprovação de outro engenheiro de plataforma (regra de negócio 2)
- [ ] O engenheiro de plataforma consegue visualizar lag de consumers, estado dos brokers, partições e throughput do cluster (RF-005)
