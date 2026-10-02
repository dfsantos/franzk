use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Environment {
    Local,
    Test,
    Staging,
    Production,
}

impl Environment {
    /// Regra de negócio 1 (PRD §6): criação de tópico em ambiente
    /// compartilhado exige aprovação por pares. Local é o único ambiente
    /// isento, pois roda na máquina do desenvolvedor sem impacto em terceiros.
    pub fn requires_creation_approval(&self) -> bool {
        !matches!(self, Environment::Local)
    }

    /// Regra de negócio 2 (PRD §6): exclusão de tópico só exige aprovação
    /// em produção.
    pub fn requires_deletion_approval(&self) -> bool {
        matches!(self, Environment::Production)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterProfile {
    pub id: String,
    pub name: String,
    pub environment: Environment,
    pub bootstrap_servers: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewClusterProfile {
    pub name: String,
    pub environment: Environment,
    pub bootstrap_servers: String,
}
