fn main() {
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

// 1. Criando e Usando Structs Simples
// Crie  uma  `struct`  chamada  `Pessoa`  que  tenha  três  campos:  `nome`,  `idade`  e  `cidade`.  Instancie
// duas pessoas diferentes no programa principal e imprima seus dados
fn exercicio1() {
    struct Pessoa {
        nome: String,
        idade: u32,
        cidade: String,
    }
    let joao = Pessoa {
        nome: String::from("Joao"),
        idade: 30,
        cidade: String::from("São Paulo"),
    };
    let maria = Pessoa {
        nome: String::from("Maria"),
        idade: 25,
        cidade: String::from("Rio de Janeiro"),
    };

    println!(
        "Pessoa 1: {} de {} anos, de {}",
        joao.nome, joao.idade, joao.cidade
    );
    println!(
        "Pessoa 2: {} de {} anos, de {}",
        maria.nome, maria.idade, maria.cidade
    );
}

// 2. Atualizando Campos de uma Struct
// Crie uma `struct` chamada `Carro` com os campos `marca`, `modelo` e `ano`. Instancie um carro e,
// utilizando  a  sintaxe  de  atualização  (`update  syntax`),  crie  uma  nova  instância  de  `Carro`  com
// apenas o campo `ano` alterado
fn exercicio2() {
    struct Carro {
        marca: String,
        modelo: String,
        ano: u32,
    }

    let carro1 = Carro {
        marca: String::from("Fiat"),
        modelo: String::from("Uno"),
        ano: 2024,
    };
    let carro2 = Carro {
        ano: 2025,
        ..carro1
    };
}

// 3. Struct com Tuples
// Crie uma `struct tuple` chamada `Coordenada` que tenha dois valores do tipo `f64`. Instancie duas
// coordenadas e calcule a distância entre elas.
fn exercicio3() {
    struct Coordenada {
        x: f64,
        y: f64,
    }
    impl Coordenada {
        fn distancia(&self, outra: &Coordenada) -> f64 {
            ((self.x - outra.x).powi(2) + (self.y - outra.y).powi(2)).sqrt()
        }
    }

    let coord1 = Coordenada { x: 1.0, y: 1.0 };
    let coord2 = Coordenada { x: 2.0, y: 2.0 };

    let distancia = coord1.distancia(&coord2);
    println!("Distância entre as coordenadas: {}", distancia);
}

// 4. Método em Struct
// Crie  uma  `struct`  chamada  `Retangulo`  que  tenha  os  campos  `largura`  e  `altura`.  Implemente  um
// método `area` que calcula a área do retângulo. No programa principal, crie um retângulo e imprima
// a área calculada.
fn exercicio4() {
    struct Retangulo {
        largura: f64,
        altura: f64,
    }

    impl Retangulo {
        fn area(&self) -> f64 {
            self.largura * self.altura
        }
    }
}

// 5. Enumeração Simples
// Crie  um  `enum`  chamado  `Cor`  que  tenha  três  variantes:  `Vermelho`,  `Verde`,  e  `Azul`.  No
// programa principal, use um `match` para imprimir uma mensagem diferente para cada cor.
fn exercicio5() {
    enum Cor {
        Vermelho,
        Verde,
        Azul,
    }

    let cor = Cor::Verde;

    match cor {
        Cor::Vermelho => println!("A cor é vermelha"),
        Cor::Verde => println!("A cor é verde"),
        Cor::Azul => println!("A cor é azul"),
    }
}

// 6. Enum com Parâmetros
// Crie  um  `enum`  chamado  `Operacao`  que  tenha  as  variantes  `Soma(i32,  i32)`,  `Subtracao(i32,i32)`,  `Multiplicacao(i32,  i32)`  e  `Divisao(i32,  i32)`.  Implemente  uma  função  que  recebe  uma
// `Operacao` e retorna o resultado, usando `match` para realizar a operação correta.
fn exercicio6() {
    enum Operacao {
        Soma(i32, i32),
        Subtracao(i32, i32),
        Multiplicacao(i32, i32),
        Divisao(i32, i32),
    }

    fn calcular(operacao: Operacao) -> i32 {
        match operacao {
            Operacao::Soma(a, b) => a + b,
            Operacao::Subtracao(a, b) => a - b,
            Operacao::Multiplicacao(a, b) => a * b,
            Operacao::Divisao(a, b) => a / b,
        }
    }

    let num1: i32 = 10;
    let num2: i32 = 5;
    let soma = Operacao::Soma(num1, num2);
    let subtracao = Operacao::Subtracao(num1, num2);

    println!("Soma: {}", calcular(soma));
    println!("Subtração: {}", calcular(subtracao));
}

// 7. Enum `Option`
// Crie  uma  função  que  recebe  uma  referência  para  uma  lista  de  números  inteiros  e  retorna  o  maior
// número  da  lista,  utilizando  `Option<i32>`.  Use  o  `match`  para  lidar  com  o  caso  onde  a  lista  está
// vazia.
fn exercicio7() {
    fn maior_numero(lista: &[i32]) -> Option<i32> {
        match lista.iter().max() {
            Some(&maior) => Some(maior),
            None => None,
        }
    }

    let numeros = vec![10, 20, 5, 32, 15];
    match maior_numero(&numeros) {
        Some(maior) => println!("O maior número é: {}", maior),
        None => println!("A lista está vazia."),
    }

    let lista_vazia: Vec<i32> = vec![];
    match maior_numero(&lista_vazia) {
        Some(maior) => println!("O maior número é: {}", maior),
        None => println!("A lista está vazia."),
    }
}

// 8. Funções Associadas em Enums
// Crie  um  `enum`  chamado  `Estado`  com  as  variantes  `Ligado`  e  `Desligado`.  Implemente  uma
// função  associada  ao  `enum`  que  retorna  a  descrição  textual  do  estado  atual.  Teste  a  função  no
// programa principal.
fn exercicio8() {
    enum Estado {
        Ligado,
        Desligado,
    }

    impl Estado {
        fn descricao(&self) -> &str {
            match self {
                Estado::Ligado => "O estado atual é Ligado",
                Estado::Desligado => "O estado atual é Desligado",
            }
        }
    }
    let estado_ligado = Estado::Ligado;
    let estado_desligado = Estado::Desligado;
    println!("{}", estado_ligado.descricao());
    println!("{}", estado_desligado.descricao());
}

// 9. Uso de `if let`
// Modifique  o  exercício  anterior  para  utilizar  `if  let`  em  vez  de  `match`  ao  verificar  se  o  estado  é
// `Ligado` antes de imprimir uma mensagem.
fn exercicio9() {
    enum Estado {
        Ligado,
        Desligado,
    }
    impl Estado {
        fn descricao(&self) -> &str {
            if let Estado::Ligado = self {
                "O estado atual é Ligado"
            } else {
                "O estado atual é Desligado"
            }
        }
    }
    let estado_ligado = Estado::Ligado;
    let estado_desligado = Estado::Desligado;

    println!("{}", estado_ligado.descricao());
    println!("{}", estado_desligado.descricao());
}

// 10. Match com Múltiplos Comandos
// Crie  um  `enum`  chamado  `Transporte`  com  as  variantes  `Carro`,  `Bicicleta`,  `Caminhada`.  Para
// cada  variante,  use  um  `match`  que  imprime  múltiplas  mensagens,  como  a  velocidade  média  e  o
// impacto ambiental do meio de transporte escolhido.
fn exercicio10() {
    enum Transporte {
        Carro,
        Bicicleta,
        Caminhada,
    }

    impl Transporte {
        fn info(&self) {
            match self {
                Transporte::Carro => {
                    println!("Carro: Velocidade média: 60 km/h");
                    println!("Impacto ambiental: Alto");
                }
                Transporte::Bicicleta => {
                    println!("Bicicleta: Velocidade média: 20 km/h");
                    println!("Impacto ambiental: Baixo");
                }
                Transporte::Caminhada => {
                    println!("Caminhada: Velocidade média: 5 km/h");
                    println!("Impacto ambiental: Nenhum");
                }
            }
        }
    }

    let carro = Transporte::Carro;
    let bicicleta = Transporte::Bicicleta;
    let caminhada = Transporte::Caminhada;

    carro.info();
    bicicleta.info();
    caminhada.info();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercicio1() {
        exercicio1();
    }
}
