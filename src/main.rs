use lunares::{divisao_segura, soma};

fn main() {
    let a = 10;
    let b = 20;
    let c = soma(a, b);
    println!("A soma de {} com {} é {}", a, b, c);

    let d: Option<f32> = divisao_segura(a, b);
    match d {
        Some(valor) => println!("A divisão de {} por {} é {}", a, b, valor),
        None => println!("Não é possível dividir {} por {}", a, b),
    }
    lunares::string_on_stack_and_heap();
    lunares::break_ownership();
}
