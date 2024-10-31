fn main() {
    let resultado = match error_handling::dividir(10, 0) {
        Ok(valor) => valor,
        Err(mensagem) => {
            println!("Erro: {}", mensagem);
            0.0
        }
    };

    println!("Resultado: {}", resultado);
}
