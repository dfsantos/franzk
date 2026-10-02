---
name: add-tauri-command
description: Checklist para adicionar um comando Tauri de ponta a ponta no Franzk (Rust → registro → binding TypeScript) sem esquecer nenhuma das pontas. Use sempre que for expor uma nova operação de backend para a UI (um novo #[tauri::command]).
---

# Adicionar um comando Tauri de ponta a ponta

Este projeto (Franzk) tem um padrão fixo para expor uma operação de backend
à UI. Pular uma destas etapas é a causa mais comum de "o front chama e dá
erro de comando não encontrado" ou "o tipo chega errado no TS".

## Passos

1. **Domínio** (se precisar de um tipo novo): adicione em
   `src-tauri/src/domain/<area>.rs`. Toda struct/enum exposta ao frontend
   leva `#[derive(Serialize)]` (ou `Deserialize` se for input) e
   `#[serde(rename_all = "camelCase")]` — confira um arquivo existente em
   `domain/` para seguir o mesmo padrão, não invente uma convenção nova.

2. **Comando**: escreva a função em `src-tauri/src/commands/<area>.rs`:
   ```rust
   #[tauri::command]
   pub fn minha_operacao(state: State<AppState>, algum_arg: String) -> AppResult<MeuTipo> {
       // valida input cedo, devolve AppError::InvalidRequest se fizer sentido
       state.kafka.alguma_coisa(&algum_arg)
   }
   ```
   Erros sempre como `AppResult<T>` (= `Result<T, AppError>`), nunca
   `.unwrap()`/`.expect()` em código alcançável por input do usuário.

3. **Registro**: adicione a função em `invoke_handler![...]` dentro de
   `src-tauri/src/lib.rs`. É uma lista — adicione sua linha no final do
   grupo da área correspondente, não reordene as outras (reduz conflito de
   merge com outros agentes trabalhando em paralelo).

4. **Tipo TypeScript**: espelhe o tipo do passo 1 em `src/lib/types.ts`,
   com os mesmos nomes de campo em `camelCase` que o `serde` já produz.

5. **Wrapper de API**: adicione uma função em `src/lib/api.ts`, dentro do
   objeto do domínio correspondente (`clustersApi`, `messagesApi`, etc.):
   ```ts
   minhaOperacao: (algumArg: string) =>
     invoke<MeuTipo>("minha_operacao", { algumArg }),
   ```
   O nome da chave do segundo argumento de `invoke` precisa bater
   exatamente com o nome do parâmetro Rust convertido para `camelCase`
   (Tauri faz essa conversão automaticamente — `algum_arg` em Rust ↔
   `algumArg` no JS).

6. **Use na UI** só através do wrapper do passo 5, nunca `invoke()` direto
   num componente.

7. **Valide**:
   ```bash
   cd src-tauri && cargo check && cargo test
   npm run build
   ```

8. Se o comando cobre uma regra de negócio do PRD (§6) ou um critério de
   aceite (§10), adicione um teste em `src-tauri/src/**/tests.rs` e marque
   o item correspondente em `docs/plan/DEVELOPMENT_PLAN.md`.
