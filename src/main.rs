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
}

fn soma(a: i32, b: i32) -> i32 {
    a + b
}

fn divisao_segura(a: i32, b: i32) -> Option<f32> {
    if b == 0 {
        None
    } else {
        Some(a as f32 / b as f32)
    }
}
