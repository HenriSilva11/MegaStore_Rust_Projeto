use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
struct Produto {
    id: u32,
    nome: String,
    categoria: String,
    preco: f32,
}

pub fn criar_catalogo() -> HashMap<String, Produto> {
    let mut catalogo = HashMap::new();

    catalogo.insert(
        "Notebook Dell".to_string(),
        Produto { id: 1, nome: "Notebook Dell".to_string(), categoria: "Eletrônicos".to_string(), preco: 4500.00 },
    );
    catalogo.insert(
        "Smartphone Samsung".to_string(),
        Produto { id: 2, nome: "Smartphone Samsung".to_string(), categoria: "Celulares".to_string(), preco: 2500.00 },
    );
    catalogo.insert(
        "Camiseta Nike".to_string(),
        Produto { id: 3, nome: "Camiseta Nike".to_string(), categoria: "Vestuário".to_string(), preco: 150.00 },
    );

    catalogo
}

pub fn buscar_produto(catalogo: &HashMap<String, Produto>, termo: &str) {
    println!("Resultados para '{}':", termo);

    let termo_lower = termo.to_lowercase();

    for (nome, produto) in catalogo.iter() {
        if nome.to_lowercase().contains(&termo_lower) {
            println!("{:?}", produto);
        }
    }
}

fn main() {
    let catalogo = criar_catalogo();

    println!("=== Sistema de Busca MegaStore ===");
    println!("Digite o nome do produto que deseja buscar:");

    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro na leitura");
    let termo = entrada.trim();

    buscar_produto(&catalogo, termo);
}
