use super::cluster::Environment;

#[test]
fn apenas_local_e_isento_de_aprovacao_na_criacao() {
    assert!(!Environment::Local.requires_creation_approval());
    assert!(Environment::Test.requires_creation_approval());
    assert!(Environment::Staging.requires_creation_approval());
    assert!(Environment::Production.requires_creation_approval());
}

#[test]
fn apenas_producao_exige_aprovacao_na_exclusao() {
    assert!(!Environment::Local.requires_deletion_approval());
    assert!(!Environment::Test.requires_deletion_approval());
    assert!(!Environment::Staging.requires_deletion_approval());
    assert!(Environment::Production.requires_deletion_approval());
}
