use std::collections::HashMap;

// Estrutura de produto
#[derive(Debug, Clone)]
struct Product {
    id: u32,
    name: String,
    category: String,
}

impl Product {
    fn new(id: u32, name: &str, category: &str) -> Self {
        Product {
            id,
            name: name.to_string(),
            category: category.to_string(),
        }
    }
}

// Mecanismo de busca com índice baseado em HashMap
struct SearchEngine {
    index: HashMap<String, Vec<Product>>,
}

impl SearchEngine {
    fn new() -> Self {
        SearchEngine {
            index: HashMap::new(),
        }
    }

    // Adiciona produto ao índice
    fn add_product(&mut self, product: Product) {
        for keyword in product
            .name
            .split_whitespace()
            .chain(product.category.split_whitespace())
        {
            let keyword = keyword.to_lowercase();
            self.index
                .entry(keyword)
                .or_insert_with(Vec::new)
                .push(product.clone());
        }
    }

    // Realiza busca por termo
    fn search(&self, term: &str) -> Vec<Product> {
        let term = term.to_lowercase();
        self.index.get(&term).cloned().unwrap_or_default()
    }
}

fn main() {
    let mut engine = SearchEngine::new();

    // Adicionando produtos
    engine.add_product(Product::new(1, "Smartphone Galaxy S21", "Eletrônicos"));
    engine.add_product(Product::new(2, "Notebook Dell XPS", "Computadores"));
    engine.add_product(Product::new(3, "Fone Bluetooth JBL", "Áudio"));
    engine.add_product(Product::new(4, "Camiseta Adidas", "Vestuário"));

    // Busca por termo
    let results = engine.search("notebook");

    println!("Resultados da busca por 'notebook':");
    for product in results {
        println!("- {} ({})", product.name, product.category);
    }
}
