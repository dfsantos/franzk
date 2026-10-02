# Guia de Início — Franzk

Este guia é para quem vai **usar** o Franzk (desenvolvedor, engenheiro de
plataforma ou testador), não para quem desenvolve o app. Se você procura a
documentação técnica, veja `CLAUDE.md` e `docs/plan/DEVELOPMENT_PLAN.md`.

> O Franzk ainda está em desenvolvimento ativo. Este guia descreve o que
> funciona **hoje** e é honesto sobre o que ainda não funciona, para você
> não perder tempo tentando algo que não foi implementado.

## Abrindo o app

Em desenvolvimento, o Franzk roda com `npm run tauri dev` (exige uma
máquina com display gráfico — não funciona dentro de uma sessão cloud do
Claude Code). Ainda não existe um instalador/versão empacotada — ver
`README.md` para os comandos.

## Os dois tipos de ambiente

O menu lateral do Franzk organiza o trabalho em cinco telas, mas para
entender **como o Franzk enxerga um cluster** o que importa primeiro é a
diferença entre dois tipos de ambiente:

- **Ambiente Local** — já vem pronto, você não cadastra nada.
- **Clusters Compartilhados** (`test`, `staging`, `production`) — você
  precisa cadastrar cada um antes de usá-lo.

### Ambiente Local

Aparece automaticamente na tela "Ambiente Local", sem nenhum cadastro.

**Atenção:** hoje esse ambiente é uma simulação em memória dentro do
próprio Franzk — não é uma conexão com um Kafka real rodando na sua
máquina. Os tópicos e mensagens que você criar ali existem só enquanto o
app está aberto e somem quando você fecha. Isso é um estado atual do
projeto, não o comportamento final pretendido (ver `docs/plan/DEVELOPMENT_PLAN.md`,
trilha B).

### Clusters Compartilhados

É aqui que mora a resposta para "o que é necessário para o Franzk
enxergar um cluster": **nenhum cluster compartilhado aparece em lugar
nenhum do app até você cadastrá-lo**, na tela **"Clusters Compartilhados"**
do menu lateral.

#### O que você precisa ter em mãos antes de cadastrar

1. **Um nome** para identificar o cluster na lista (ex.: "Homologação —
   time de pagamentos").
2. **O ambiente correto**: `test`, `staging` ou `production`. Isso não é
   só organização — define qual regra de aprovação vale para esse cluster:
   - `test` e `staging`: criar um tópico sempre passa por aprovação de
     outro engenheiro de plataforma; excluir é direto.
   - `production`: tanto criar quanto excluir um tópico exigem aprovação
     de outro engenheiro de plataforma.

   Cadastrar um cluster de produção como `test` (ou vice-versa) faz o
   Franzk aplicar a regra errada — confira o ambiente com atenção.
3. **O endereço de bootstrap servers** do cluster (ex.:
   `meu-kafka.empresa.com:9092`, ou `localhost:9093` se for um Kafka local
   de teste).

#### Passo a passo

1. Abra **"Clusters Compartilhados"** no menu lateral.
2. Na seção **"Cadastrar cluster"**, preencha nome, ambiente e bootstrap
   servers.
3. Clique em **"Cadastrar"**.
4. O cluster aparece imediatamente na tabela acima do formulário, e passa
   a estar disponível nos seletores de cluster das telas **Inspeção de
   Mensagens**, **Monitoramento** e **Solicitar criação de tópico**.

Não existe (ainda) um jeito de editar ou remover um cluster cadastrado
pela interface — se cadastrar com um dado errado, o caminho hoje é
cadastrar de novo com o nome/dado corrigido.

## O que "enxergar um cluster" ainda não quer dizer, hoje

Cadastrar um cluster faz o Franzk **listá-lo** e liberar as telas que
dependem de um cluster selecionado. Ainda **não** significa que o Franzk
está de fato conversando com o broker Kafka daquele endereço: a leitura de
tópicos, mensagens e saúde do cluster (lag, brokers, partições,
throughput) ainda roda sobre dados simulados internamente, não sobre o
cluster real — a integração com o protocolo Kafka de verdade é um
trabalho em andamento (ver trilha B em `docs/plan/DEVELOPMENT_PLAN.md`).
Isso significa, por exemplo, que o endereço de bootstrap servers não é
validado no cadastro: você pode digitar qualquer coisa que o Franzk aceita
do mesmo jeito.

Se quiser ter um Kafka real no ar para já ir testando endereços de
bootstrap servers verdadeiros (mesmo sem o Franzk ainda operar contra
eles), veja `docs/dev-kafka-compose.md` — ele sobe três clusters Kafka
locais via Docker, um para cada ambiente compartilhado.

## Próximos passos depois de cadastrar um cluster

- **Solicitar criação de tópico**: na própria tela de Clusters
  Compartilhados, escolha o cluster, preencha o tópico e quem está
  solicitando. Em `test`/`staging`/`production` isso sempre gera uma
  solicitação de aprovação.
- **Aprovações**: na tela "Aprovações", outra pessoa (nunca quem
  solicitou) aprova ou rejeita. Hoje "quem solicitou" e "quem aprova" são
  só campos de texto digitados na tela — ainda não há login (ver gap de
  autenticação em `CLAUDE.md`), então o controle depende de cada um
  digitar seu próprio nome corretamente.
- **Inspeção de Mensagens** e **Monitoramento**: selecione o cluster
  cadastrado no seletor no topo da tela.
