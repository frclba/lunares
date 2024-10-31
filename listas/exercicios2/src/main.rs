fn main() {
    // NOVOS EXERCÍCIOS SOBRE OWNERSHIP
    exercicio1();
    exercicio2();
    exercicio3();
    exercicio4();
    exercicio5();
    exercicio6();
    exercicio7();
    exercicio8();
    exercicio9();
    exercicio10();
}

fn exercicio1() {
    let string1 = String::from("Olá, mundo!");
    let string2 = String::from("Rust é incrível!");

    // Movendo string1 para nova_string1
    let nova_string1 = string1;

    // Clonando string2 para nova_string2
    let nova_string2 = string2.clone();

    // Tentando imprimir string1 (isso causará um erro se descomentado)
    // println!("string1: {}", string1); // string1 foi movida

    // Imprimindo as novas strings
    println!("nova_string1: {}", nova_string1);
    println!("string2: {}", string2);
    println!("nova_string2: {}", nova_string2);
}

fn exercicio2() {
    let vetor = vec![1, 2, 3, 4, 5];
    let soma_total = soma(&vetor);
    println!("Soma total: {}", soma_total);
    println!("Vetor: {:?}", vetor);
}

fn soma(numeros: &[i32]) -> i32 {
    numeros.iter().sum()
}

fn exercicio_ownership3() {
    let s1 = String::from("String 1");
    let s2 = String::from("String 2");

    // Reatribuindo s1 para s2
    let s2 = s1;

    // Tentando usar s1 após a reatribuição (isso causará um erro se descomentado)
    // println!("s1: {}", s1); // s1 foi movida para s2

    println!("s2 após reatribuição: {}", s2);
}
fn exercicio_ownership4() {
    let mut texto = String::from("Texto original");
    adicionar_prefixo(&mut texto);
    println!("Texto após modificação: {}", texto);
}

fn adicionar_prefixo(texto: &mut String) {
    texto.insert_str(0, "Prefixo: ");
}

fn exercicio_ownership5() {
    let vetor1 = vec![1, 2, 3];
    let vetor2 = vec![4, 5, 6];

    let resultado = multiplicar_vetores(&vetor1, &vetor2);
    println!("Vetor resultante: {:?}", resultado);
}

fn multiplicar_vetores(v1: &Vec<i32>, v2: &Vec<i32>) -> Vec<i32> {
    v1.iter().zip(v2.iter()).map(|(a, b)| a * b).collect()
}

fn exercicio_ownership6() {
    let texto = String::from("Olá, mundo!");
    let num_caracteres = contar_caracteres(&texto);
    println!("Número de caracteres: {}", num_caracteres);
}

fn contar_caracteres(texto: &String) -> usize {
    texto.chars().count()
}

fn exercicio_ownership7() {
    let texto = String::from("Esta é uma frase de teste");
    let palavras = dividir_palavras(&texto);
    println!("Palavras: {:?}", palavras);
}

fn dividir_palavras(texto: &String) -> Vec<&str> {
    texto.split_whitespace().collect()
}

fn exercicio_ownership8() {
    let s1 = String::from("Olá, ");
    let s2 = String::from("mundo!");
    let resultado = concatenar(&s1, &s2);
    println!("Resultado da concatenação: {}", resultado);
}

fn concatenar(s1: &String, s2: &String) -> String {
    format!("{}{}", s1, s2)
}

fn exercicio_ownership9() {
    let mut vetor = vec![1, 2, 3];
    let numero = 4;
    adicionar_elemento(&mut vetor, &numero);
    println!("Vetor após adição: {:?}", vetor);
}

fn adicionar_elemento(vetor: &mut Vec<i32>, num: &i32) {
    vetor.push(*num);
}

fn exercicio_ownership10() {
    let mut vetor = vec![1, 2, 3, 4, 5];
    dobrar_elementos(&mut vetor);
    println!("Vetor após dobrar os elementos: {:?}", vetor);
}

fn dobrar_elementos(vetor: &mut Vec<i32>) {
    for elem in vetor.iter_mut() {
        *elem *= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn teste_exercicio_ownership1() {
        let string1 = String::from("Teste");
        let string2 = String::from("Clonagem");

        let nova_string1 = string1;
        let nova_string2 = string2.clone();

        // Verificando se nova_string1 tem o valor correto
        assert_eq!(nova_string1, "Teste");

        // string1 não pode ser usada aqui, pois foi movida

        // Verificando se string2 e nova_string2 têm o mesmo valor
        assert_eq!(string2, "Clonagem");
        assert_eq!(nova_string2, "Clonagem");
    }

    #[test]
    fn teste_exercicio_ownership2() {
        let vetor = vec![10, 20, 30];
        let resultado = soma(&vetor);
        assert_eq!(resultado, 60);
        // Verificando se o vetor original permanece o mesmo
        assert_eq!(vetor, vec![10, 20, 30]);
    }
    #[test]
    fn teste_exercicio_ownership3() {
        let s1 = String::from("Teste 1");
        let s2 = String::from("Teste 2");

        let s2 = s1;

        // s1 não é mais válido aqui

        assert_eq!(s2, "Teste 1");
    }

    #[test]
    fn teste_exercicio_ownership4() {
        let mut texto = String::from("Teste");
        adicionar_prefixo(&mut texto);
        assert_eq!(texto, "Prefixo: Teste");
    }
    #[test]
    fn teste_exercicio_ownership5() {
        let vetor1 = vec![2, 3, 4];
        let vetor2 = vec![5, 6, 7];

        let resultado = multiplicar_vetores(&vetor1, &vetor2);
        assert_eq!(resultado, vec![10, 18, 28]);
    }
    #[test]
    fn teste_exercicio_ownership6() {
        let texto = String::from("Rust 🦀");
        let num_caracteres = contar_caracteres(&texto);
        assert_eq!(num_caracteres, 6); // Contando o emoji como um caractere
    }

    #[test]
    fn teste_exercicio_ownership7() {
        let texto = String::from("Um dois três");
        let palavras = dividir_palavras(&texto);
        assert_eq!(palavras, vec!["Um", "dois", "três"]);
    }

    #[test]
    fn teste_exercicio_ownership8() {
        let s1 = String::from("Rust ");
        let s2 = String::from("lang");
        let resultado = concatenar(&s1, &s2);
        assert_eq!(resultado, "Rust lang");
    }

    #[test]
    fn teste_exercicio_ownership9() {
        let mut vetor = vec![5, 6, 7];
        let numero = 8;
        adicionar_elemento(&mut vetor, &numero);
        assert_eq!(vetor, vec![5, 6, 7, 8]);
    }

    #[test]
    fn teste_exercicio_ownership10() {
        let mut vetor = vec![2, 4, 6];
        dobrar_elementos(&mut vetor);
        assert_eq!(vetor, vec![4, 8, 12]);
    }
}
