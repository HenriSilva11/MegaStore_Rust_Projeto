use megastore_search::*;

#[test]
fn teste_busca_produto() {
    let catalogo = criar_catalogo();
    assert!(catalogo.contains_key("Notebook Dell"));
}
