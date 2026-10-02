---
name: sync-plan-and-claude-md
description: Checklist para atualizar docs/plan/DEVELOPMENT_PLAN.md e CLAUDE.md depois de concluir uma entrega no Franzk, para que os dois arquivos continuem sendo a fonte da verdade sobre o estado real do projeto. Use ao final de qualquer tarefa que mude código em src-tauri/ ou src/.
---

# Manter o plano e o CLAUDE.md coerentes

No Franzk, `CLAUDE.md` e `docs/plan/DEVELOPMENT_PLAN.md` só são úteis se
refletirem o código de verdade. Rode este checklist ao final de uma
entrega, antes de considerá-la pronta.

## Checklist

1. **Confirme, não assuma.** Antes de marcar algo como feito, confira no
   código (`Grep`/`Read`) que existe mesmo, e que `cargo check`/`cargo
   test`/`npm run build` passam. Não marque uma entrega como concluída só
   porque o commit message diz isso.

2. **`docs/plan/DEVELOPMENT_PLAN.md`:**
   - Marque a(s) checkbox(es) da entrega concluída na trilha correspondente
     (seção 4, "Backlog em entregas pequenas").
   - Se a entrega fecha ou altera uma linha da tabela de critérios de
     aceite (seção 5), atualize o status ali também.
   - Se surgiu uma entrega nova não prevista (ex. um gap descoberto durante
     o trabalho), adicione-a na trilha certa em vez de deixar só numa nota
     solta no PR.

3. **`CLAUDE.md`:**
   - Se a entrega mudou algo estrutural (novo módulo, nova dependência,
     nova regra de negócio, novo comando, nova limitação de ambiente),
     atualize a seção relevante (arquitetura, mapa de módulos, regras de
     negócio, ou comandos).
   - Atualize a seção "Estado atual" (lista de checkboxes no final) para
     refletir o que mudou.

4. **`docs/adr/`:** se a entrega envolveu escolher entre alternativas
   técnicas (ex. biblioteca, abordagem de persistência, protocolo), registre
   a decisão num ADR novo em vez de deixar só no histórico do PR.

5. **Requisito `[PENDENTE]` ou `[INFERIDO]` do PRD:** se a entrega tocou em
   um requisito assim marcado, não resolva a ambiguidade implementando por
   suposição — registre a pendência na trilha F do plano e proponha nova
   entrevista com o stakeholder (skill `anthropic-skills:prd-interviewer`).

6. Faça a atualização de documentação **no mesmo commit/PR** da mudança de
   código sempre que possível — documentação que fica "para depois" tende a
   nunca ser feita.
