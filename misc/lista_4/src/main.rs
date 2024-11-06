struct Retangulo {
    largura: u32,
    altura: u32,
}

fn alterar_largura(largura: &mut u32, nova_largura: u32) {
    *largura = nova_largura;
}

fn main() {
    let first: &str = "Hello, world!";
    let second: &str = "Hello, world!";

    // let soma: lista_3::Operacao = lista_3::Operacao::Soma(10, 20);

    let mut retangulo = Retangulo {
        altura: 10,
        largura: 20,
    };

    let largura_mutavel = &mut retangulo.largura;

    *largura_mutavel = 60;
    // alterar_largura(largura_mutavel, 60);

    println!("Largura: {}", retangulo.largura);
    println!("Altura: {}", retangulo.altura);
}
