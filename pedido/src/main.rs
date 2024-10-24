enum Produto {
    Livro {
        nome: String,
        preco: f64,
    },
    Alimento {
        nome: String,
        peso_kg: f64,
        preco_por_kg: f64,
    },
    Roupa {
        nome: String,
        tamanho: String,
        preco: f64,
    },
}

struct ItemPedido {
    produto: Produto,
    quantidade: u32,
}

struct Pedido {
    items: Vec<ItemPedido>,
}

impl Pedido {
    fn calcular_total(&self) -> f64 {
        let mut total = 0.0;
        for item in &self.items {
            total += match &item.produto {
                Produto::Livro { preco, .. } => preco * item.quantidade as f64,
                Produto::Alimento {
                    preco_por_kg,
                    peso_kg,
                    ..
                } => preco_por_kg * peso_kg * item.quantidade as f64,
                Produto::Roupa { preco, .. } => preco * item.quantidade as f64,
            };
        }
        total
    }
}

fn main() {
    println!("Hello, world!");
    let pedido = Pedido {
        items: vec![
            ItemPedido {
                produto: Produto::Livro {
                    nome: "Rust Programming Language".to_string(),
                    preco: 50.0,
                },
                quantidade: 1,
            },
            ItemPedido {
                produto: Produto::Alimento {
                    nome: "Maçã".to_string(),
                    peso_kg: 2.0,
                    preco_por_kg: 3.0,
                },
                quantidade: 2,
            },
            ItemPedido {
                produto: Produto::Roupa {
                    nome: "Camiseta".to_string(),
                    tamanho: "M".to_string(),
                    preco: 30.0,
                },
                quantidade: 3,
            },
        ],
    };
    println!("Total: R$ {:.2}", pedido.calcular_total());
}
