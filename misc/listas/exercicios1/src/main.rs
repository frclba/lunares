use std::f64::consts;

fn main() {
    exercicio1();

    // EXERCICIO 2
    exercicio2(false);
    exercicio2(true);

    // EXERCICIO 3
    exercicio3();

    // EXERCICIO 4
    exercicio4();

    // EXERCICIO 5
    exercicio5();

    // EXERCICIO 6
    exercicio6();

    // EXERCICIO 7
    exercicio7();

    // EXERCICIO 8
    exercicio8();

    // EXERCICIO 9
    exercicio9();

    // EXERCICIO 10
    exercicio10();
}

// EXERCICIO 1 - Variáveis e Tipos
fn exercicio1() {
    let (mut idade, peso, ativo) = obter_variaveis();

    println!("Idade: {}", idade);
    println!("Peso: {}", peso);
    println!("Ativo: {}", ativo);

    // Atribuir um novo valor a `idade`
    modificar_idade(&mut idade);
    println!("Nova idade: {}", idade);
}

fn obter_variaveis() -> (i32, f64, bool) {
    let idade: i32 = 25;
    let peso: f64 = 70.5;
    let ativo: bool = true;

    (idade, peso, ativo)
}

fn modificar_idade(idade: &mut i32) {
    *idade = 30;
}

// EXERCICIO 2 -  Escopo de Variáveis
fn exercicio2(flag: bool) -> i32 {
    let x = 10;
    if flag {
        let x = 20;
        // Dentro do bloco, x vale 20
        println!("Dentro do bloco, x = {}", x);
        return x;
    }
    println!("Fora do bloco, x = {}", x);
    x
}

// EXERCICIO 3 - Shadows de Variáveis
fn exercicio3() -> i32 {
    let num = 5;
    let num = num + 3; // Sombreamento: num agora é 8
    let num = num * 2; // Sombreamento: num agora é 16

    println!("O valor final de num é: {}", num);
    str_to_int(&num.to_string())
}
fn str_to_int(s: &str) -> i32 {
    let valor = s; // `valor` é uma &str
    let valor = valor.parse::<i32>().unwrap(); // `valor` agora é um i32
    println!("Valor: {}", valor);
    valor
}

// EXERCICIO 4 - Constantes e Escopo de Módulo
fn exercicio4() {
    let raio = 10.0;
    let area = consts::PI * raio * raio;
    println!("A área do círculo é: {}", area);

    // Tentativa de alterar o valor de uma constante (isso causará um erro)
    // PI = 3.14; // Descomente esta linha para ver o erro
}

// EXERCICIO 5 - Segurança de Memória com Variáveis Imutáveis
fn exercicio5() {
    let dados = String::from("Segurança de Memória");
    imprimir_dados(&dados);
    println!("Dados novamente: {}", dados);
}

fn imprimir_dados(dados: &String) {
    println!("Dados: {}", dados);
}

// EXERCICIO 6 - Funções Simples
fn exercicio6() {
    let resultado = soma(10, 20);
    println!("Resultado da soma: {}", resultado);

    // Usando a função que retorna i64
    let resultado_i64 = soma_i64(10, 20);
    println!("Resultado da soma (i64): {}", resultado_i64);
}

fn soma(a: i32, b: i32) -> i32 {
    a + b
}

// Alterando o tipo de retorno para i64
fn soma_i64(a: i32, b: i32) -> i64 {
    (a + b) as i64
}

// EXERCICIO 7 - Funções com Retorno Antecipado
fn exercicio7() {
    match divisao_segura(10, 2) {
        Some(resultado) => println!("Divisão bem-sucedida: {}", resultado),
        None => println!("Divisão por zero!"),
    }

    match divisao_segura(10, 0) {
        Some(resultado) => println!("Divisão bem-sucedida: {}", resultado),
        None => println!("Divisão por zero!"),
    }
}

fn divisao_segura(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

// EXERCICIO 8 - Referências e Empréstimo
fn exercicio8() {
    let numero = 7;
    let resultado = calcular_dobro(&numero);
    println!("O dobro de {} é {}", numero, resultado);
}

fn calcular_dobro(num: &i32) -> i32 {
    *num * 2
}

// EXERCICIO 9 - Tipos Opcionais e Tratamento de Erros
fn exercicio9() {
    let vetor = vec![1, 2, 3, 4, 5];
    let numero = 3;
    match busca_numero(&vetor, numero) {
        Some(indice) => println!("Número {} encontrado na posição {}", numero, indice),
        None => println!("Número {} não encontrado", numero),
    }
}

fn busca_numero(vetor: &Vec<i32>, num: i32) -> Option<usize> {
    for (i, &valor) in vetor.iter().enumerate() {
        if valor == num {
            return Some(i);
        }
    }
    None
}

// EXERCICIO 10 - Mutabilidade e Segurança de Memória
fn exercicio10() {
    let mut lista = vec![1, 2, 3];
    println!("Lista antes: {:?}", lista);
    adicionar_numero(&mut lista, 4);
    println!("Lista depois: {:?}", lista);

    // Tentativa de passar referência não mutável (causará erro)
    // adicionar_numero(&lista, 5); // Descomente para ver o erro
}

fn adicionar_numero(lista: &mut Vec<i32>, num: i32) {
    lista.push(num);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn teste_obter_variaveis() {
        let (idade, peso, ativo) = obter_variaveis();

        assert_eq!(idade, 25);
        assert_eq!(peso, 70.5);
        assert!(ativo);
    }

    #[test]
    fn teste_modificar_idade() {
        let mut idade: i32 = 25;
        modificar_idade(&mut idade);
        assert_eq!(idade, 30);
    }

    #[test]
    fn teste_exercicio2() {
        assert_eq!(exercicio2(true), 20);
        assert_eq!(exercicio2(false), 10);
    }

    #[test]
    fn teste_exercicio3() {
        assert_eq!(exercicio3(), 16);
        assert_eq!(exercicio3(), str_to_int("16"));
    }
    #[test]
    fn teste_exercicio4() {
        exercicio4();
    }

    #[test]
    fn teste_exercicio5() {
        exercicio5();
    }

    #[test]
    fn teste_exercicio6() {
        exercicio6();
        let resultado = soma_i64(10, 20);
        assert_eq!(resultado, 30i64);
    }

    #[test]
    fn teste_exercicio7() {
        assert_eq!(divisao_segura(10, 2), Some(5));
        assert_eq!(divisao_segura(10, 0), None);
    }

    #[test]
    fn teste_exercicio8() {
        let numero = 7;
        let resultado = calcular_dobro(&numero);
        assert_eq!(resultado, 14);
    }

    #[test]
    fn teste_exercicio9() {
        let vetor = vec![1, 2, 3, 4, 5];
        assert_eq!(busca_numero(&vetor, 3), Some(2));
        assert_eq!(busca_numero(&vetor, 6), None);
    }

    #[test]
    fn teste_exercicio10() {
        let mut lista = vec![1, 2, 3];
        adicionar_numero(&mut lista, 4);
        assert_eq!(lista, vec![1, 2, 3, 4]);
    }
}
