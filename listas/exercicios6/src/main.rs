fn main() {
    exercicio1();
    exercicio2();
    exercicio3();
    exercicio4();
    exercicio5();
    exercicio6();
    exercicio7();
}

// 1. Lifetime em Struct
// Defina uma estrutura Point<'a> que contenha duas referências a i32. Implemente
// uma função que, dado um Point, retorne o maior valor entre os dois.
fn exercicio1() {
    let a = 10;
    let b = 20;
    let ponto = Point { x: &a, y: &b };
    let maior = maior_valor(&ponto);
    println!("O maior valor é: {}", maior);
}

struct Point<'a> {
    x: &'a i32,
    y: &'a i32,
}

fn maior_valor<'a>(ponto: &Point<'a>) -> &'a i32 {
    if ponto.x > ponto.y {
        ponto.x
    } else {
        ponto.y
    }
}

// 2. Retorno de Referências em Structs
// Crie uma estrutura Owner<'a> que contenha uma referência a uma String.
// Implemente um método que retorne uma fatia da String que a estrutura contém,
// utilizando lifetimes apropriados.
fn exercicio2() {
    let texto = String::from("Hello, Rust!");
    let owner = Owner { texto: &texto };
    let fatia = owner.obter_fatia(0, 5);
    println!("Fatia obtida: {}", fatia);
}

struct Owner<'a> {
    texto: &'a String,
}

impl<'a> Owner<'a> {
    fn obter_fatia(&self, inicio: usize, fim: usize) -> &'a str {
        &self.texto[inicio..fim]
    }
}

// 3. Função com Lifetimes Diferentes
// Implemente uma função concat_with_prefix que recebe duas referências de
// lifetime diferentes: uma &str com prefixo e uma &str para o conteúdo. Retorne
// uma nova String que concatene o prefixo e o conteúdo, garantindo que os
// lifetimes sejam respeitados.
fn exercicio3() {
    let prefixo = "Sr.";
    let nome = String::from("Anderson");
    let resultado = concat_with_prefix(prefixo, nome.as_str());
    println!("Resultado: {}", resultado);
}

fn concat_with_prefix<'a, 'b>(prefixo: &'a str, conteudo: &'b str) -> String {
    format!("{} {}", prefixo, conteudo)
}

// 4. Struct com Lifetimes Aninhados
// Implemente uma estrutura Context<'a, 'b> que contenha referências para duas
// outras estruturas, A e B. Ambas as estruturas devem conter lifetimes diferentes ('a
// e 'b). Implemente um método em Context que modifique uma das referências com
// base no valor da outra, utilizando lifetimes corretamente.
fn exercicio4() {
    let valor_a = A { dado: 10 };
    let mut valor_b = B { dado: 20 };
    {
        let contexto = Context {
            a: &valor_a,
            b: &mut valor_b,
        };
        contexto.atualizar_b_com_base_em_a();
    }
    println!("Novo valor de B: {}", valor_b.dado);
}

struct A {
    dado: i32,
}

struct B {
    dado: i32,
}

struct Context<'a, 'b> {
    a: &'a A,
    b: &'b mut B,
}

impl<'a, 'b> Context<'a, 'b> {
    fn atualizar_b_com_base_em_a(&self) {
        self.b.dado += self.a.dado;
    }
}

// 5. Função com Lifetimes e Mut
// Crie uma função swap_refs que recebe duas referências mutáveis a variáveis i32.
// A função deve trocar os valores entre as duas variáveis usando lifetimes
// explicitamente, para evitar problemas de referências múltiplas.
fn exercicio5() {
    let mut x = 100;
    let mut y = 200;
    println!("Antes da troca: x = {}, y = {}", x, y);
    swap_refs(&mut x, &mut y);
    println!("Após a troca: x = {}, y = {}", x, y);
}

fn swap_refs<'a>(a: &'a mut i32, b: &'a mut i32) {
    let temp = *a;
    *a = *b;
    *b = temp;
}

// 6. Função Recursiva com Lifetimes Complexos
// Escreva uma função nested_refs<'a, 'b> que retorna uma referência a uma função
// dentro de outra, que faz operações aritméticas. Cada função interna deve ter seu
// próprio lifetime, e a função principal deve garantir que todas as referências
// internas não ultrapassem seu escopo.
fn exercicio6() {
    let resultado = nested_refs(10, 5);
    println!("Resultado: {}", resultado);
}

fn nested_refs<'a, 'b>(x: i32, y: i32) -> i32 {
    fn operacao1<'a>(a: i32) -> i32 {
        a * 2
    }

    fn operacao2<'b>(b: i32) -> i32 {
        b + 3
    }

    operacao1(x) + operacao2(y)
}

// 7. Lifetime Anônimo com Closures e Iteradores
// Implemente uma função process_elements<'a> que recebe uma fatia de inteiros e
// uma closure. A closure deve ser capaz de modificar a fatia de dados, e a função
// process_elements deve aplicar a closure a cada elemento da fatia usando um
// iterador. Garanta que o lifetime da closure e do iterador sejam compatíveis com o
// lifetime da fatia original, sem causar problemas de mutabilidade ou uso posterior.
fn exercicio7() {
    let mut dados = vec![1, 2, 3, 4, 5];
    process_elements(&mut dados, |x| *x *= 2);
    println!("Dados após processamento: {:?}", dados);
}

fn process_elements<F>(dados: &mut [i32], mut func: F)
where
    F: FnMut(&mut i32),
{
    for item in dados.iter_mut() {
        func(item);
    }
}

mod tests {
    use super::*;

    #[test]
    fn teste_exercicio1() {
        let a = 30;
        let b = 25;
        let ponto = Point { x: &a, y: &b };
        let maior = maior_valor(&ponto);
        assert_eq!(*maior, 30);
    }

    #[test]
    fn teste_exercicio2() {
        let texto = String::from("Testing lifetimes");
        let owner = Owner { texto: &texto };
        let fatia = owner.obter_fatia(0, 7);
        assert_eq!(fatia, "Testing");
    }

    #[test]
    fn teste_exercicio3() {
        let prefixo = "Dra.";
        let nome = "Silva";
        let resultado = concat_with_prefix(prefixo, nome);
        assert_eq!(resultado, "Dra. Silva");
    }

    #[test]
    fn teste_exercicio4() {
        let valor_a = A { dado: 5 };
        let mut valor_b = B { dado: 15 };
        {
            let contexto = Context {
                a: &valor_a,
                b: &mut valor_b,
            };
            contexto.atualizar_b_com_base_em_a();
        }
        assert_eq!(valor_b.dado, 20);
    }

    #[test]
    fn teste_exercicio5() {
        let mut x = 1;
        let mut y = 2;
        swap_refs(&mut x, &mut y);
        assert_eq!(x, 2);
        assert_eq!(y, 1);
    }

    #[test]
    fn teste_exercicio6() {
        let resultado = nested_refs(2, 3);
        assert_eq!(resultado, (2 * 2) + (3 + 3)); // 4 + 6 = 10
    }

    #[test]
    fn teste_exercicio7() {
        let mut dados = vec![10, 20, 30];
        process_elements(&mut dados, |x| *x += 5);
        assert_eq!(dados, vec![15, 25, 35]);
    }
}
