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

// 1. Movendo e Atualizando Dados em Structs
// Crie uma `struct` chamada `ContaBancaria` com os campos `nome_titular`, `saldo`, e
// `numero_conta`. Instancie duas contas bancárias, uma para 'Alice' e outra para 'Bob'. Simule uma
// transferência  de  saldo  movendo  a  posse  dos  valores  entre  as  contas.  Utilize  a  sintaxe  de
// atualização de structs para transferir o saldo de uma conta para outra.
fn exercicio1() {
    struct ContaBancaria {
        nome_titular: String,
        saldo: f64,
        numero_conta: u32,
    }

    let alice = ContaBancaria {
        nome_titular: String::from("Alice"),
        saldo: 100.0,
        numero_conta: 1234,
    };

    let bob = ContaBancaria {
        nome_titular: String::from("Bob"),
        saldo: 50.0,
        numero_conta: 5678,
    };

    let alice = ContaBancaria {
        saldo: alice.saldo - 10.0,
        ..alice
    };

    let bob = ContaBancaria {
        saldo: bob.saldo + 10.0,
        ..bob
    };

    println!("Saldo de Alice: R$ {}", alice.saldo);
    println!("Saldo de Bob: R$ {}", bob.saldo);
}

// 2. Gerenciamento de Empréstimos Mutáveis em Struct
// Crie uma `struct` chamada `Livro` com os campos `titulo`, `autor` e `quantidade_paginas`.
// Implemente um método `adicionar_paginas` que aceita uma referência mutável ao livro e adiciona
// páginas  ao  campo  `quantidade_paginas`.  O  método  deve  garantir  que  não  se  possa  adicionar
// páginas  de  maneira  inválida  (como  números  negativos).  No  programa  principal,  faça  diversas
// alterações na struct e garanta que o valor seja atualizado corretamente.
fn exercicio2() {
    struct Livro {
        titulo: String,
        autor: String,
        quantidade_paginas: u32,
    }

    impl Livro {
        fn adicionar_paginas(&mut self, paginas: u32) {
            if paginas > 0 {
                self.quantidade_paginas += paginas;
            }
        }
    }

    let mut livro = Livro {
        titulo: String::from("O Senhor dos Anéis"),
        autor: String::from("J.R.R. Tolkien"),
        quantidade_paginas: 1000,
    };

    println!("Páginas do livro: {}", livro.quantidade_paginas);

    livro.adicionar_paginas(500);
    println!("Páginas do livro: {}", livro.quantidade_paginas);

    livro.adicionar_paginas(0);
    println!("Páginas do livro: {}", livro.quantidade_paginas);

    livro.adicionar_paginas(100);
    println!("Páginas do livro: {}", livro.quantidade_paginas);
}

// 3. Enum Complexo com Parâmetros
// Crie  um  `enum`  chamado  `EventoSistema`  que  tenha  as  variantes  `Conexao(String)`,  `Erro(u32,
// String)`,  e  `Desconexao(String,  u32)`.  No  programa  principal,  use  `match`  para  lidar  com  cada
// evento, exibindo mensagens de status diferentes com base no evento. Cada variante deve ter um
// comportamento diferenciado.
fn exercicio3() {
    enum EventoSistema {
        Conexao(String),
        Erro(u32, String),
        Desconexao(String, u32),
    }

    fn executeEvent(evento: EventoSistema) {
        match evento {
            EventoSistema::Conexao(ip) => {
                println!("Conexão estabelecida com o IP: {}", ip);
            }
            EventoSistema::Erro(codigo, mensagem) => {
                println!("Erro {} - {}", codigo, mensagem);
            }
            EventoSistema::Desconexao(ip, porta) => {
                println!("Desconexão do IP {} na porta {}", ip, porta);
            }
        }
    }

    let mut evento = EventoSistema::Erro(404, String::from("Página não encontrada"));
    executeEvent(evento);
    evento = EventoSistema::Conexao(String::from("0.0.0.0"));
    executeEvent(evento);
    evento = EventoSistema::Desconexao(String::from("0.0.0.0"), 8080);
    executeEvent(evento);
}

// 4. Função com Retorno de Enum `Option`
// Implemente  uma  função  chamada  `encontrar_numero`  que  recebe  um  vetor  de  inteiros  e  um
// número alvo. A função deve retornar um `Option<usize>`, indicando a posição do número no vetor.
// Utilize  `match`  para  lidar  com  o  caso  em  que  o  número  é  encontrado  ou  não,  imprimindo  uma
// mensagem apropriada.
fn exercicio4() {
    fn encontrar_numero(vetor: Vec<i32>, alvo: i32) -> Option<usize> {
        for (i, &numero) in vetor.iter().enumerate() {
            if numero == alvo {
                return Some(i);
            }
        }
        None
    }

    let vetor = vec![1, 2, 3, 4, 5];
    let alvo = 3;

    match encontrar_numero(vetor, alvo) {
        Some(posicao) => println!("Número {} encontrado na posição {}", alvo, posicao),
        None => println!("Número {} não encontrado", alvo),
    }
}

// 5. Referências Imutáveis e Mutáveis Simultâneas
// Crie uma `struct` chamada `Retangulo` com os campos `largura` e `altura`. Escreva um programa
// que  simultaneamente  passa  uma  referência  imutável  da  `largura`  e  uma  referência  mutável  da
// `altura` para diferentes funções. A função que recebe a referência mutável deve alterar o valor da
// altura. Mostre como o Rust previne a violação das regras de empréstimo com referências
// simultâneas.
fn exercicio5() {
    struct Retangulo {
        largura: u32,
        altura: u32,
    }

    fn alterar_altura(retangulo: &mut Retangulo) {
        retangulo.altura = 100;
    }

    let mut retangulo = Retangulo {
        largura: 50,
        altura: 50,
    };

    let largura = retangulo.largura;
    alterar_altura(&mut retangulo);

    println!("Largura: {}", largura);
    println!("Altura: {}", retangulo.altura);
}

// 6. Transferência de Ownership com Vetores
// Implemente um programa que cria dois vetores de strings representando listas de compras de duas
// pessoas.  A  primeira  pessoa  decide  mover  todos  os  seus  itens  para  a  lista  da  segunda  pessoa.
// Após o movimento, o primeiro vetor deve estar vazio. Utilize as regras de *ownership* para garantir
// que a operação seja segura e eficiente.
fn exercicio6() {
    let mut lista_compras_pessoa1 = vec![
        String::from("Arroz"),
        String::from("Feijão"),
        String::from("Carne"),
    ];

    let mut lista_compras_pessoa2 = vec![
        String::from("Leite"),
        String::from("Pão"),
        String::from("Queijo"),
    ];

    lista_compras_pessoa2.append(&mut lista_compras_pessoa1);

    println!("Lista de compras da pessoa 1: {:?}", lista_compras_pessoa1);
    println!("Lista de compras da pessoa 2: {:?}", lista_compras_pessoa2);
}

// 7. Enumeração com Métodos e Atualizações
// Crie um `enum` chamado `Jogo` que tenha as variantes `Iniciado(u32)`, `EmProgresso(u32, u32)` e
// `Finalizado(u32)`,  representando  o  status  de  um  jogo  com  pontuações.  Implemente  métodos  para
// iniciar  o  jogo,  atualizar  a  pontuação  e  finalizar  o  jogo.  Utilize  `match`  no  programa  principal  para
// gerenciar a lógica do jogo.
fn exercicio7() {
    enum Jogo {
        Iniciado(u32),
        EmProgresso(u32, u32),
        Finalizado(u32),
    }

    impl Jogo {
        fn iniciar_jogo(pontuacao: u32) -> Jogo {
            Jogo::Iniciado(pontuacao)
        }

        fn atualizar_pontuacao(&mut self, nova_pontuacao: u32) {
            match self {
                Jogo::Iniciado(_) => {
                    *self = Jogo::EmProgresso(0, nova_pontuacao);
                }
                Jogo::EmProgresso(pontuacao_atual, _) => {
                    *self = Jogo::EmProgresso(*pontuacao_atual, nova_pontuacao);
                }
                Jogo::Finalizado(_) => {
                    println!("Jogo finalizado, não é possível atualizar a pontuação");
                }
            }
        }

        fn finalizar_jogo(&mut self) {
            match self {
                Jogo::Iniciado(_) => {
                    println!("Jogo não pode ser finalizado sem pontuação");
                }
                Jogo::EmProgresso(_, pontuacao) => {
                    *self = Jogo::Finalizado(*pontuacao);
                }
                Jogo::Finalizado(_) => {
                    println!("Jogo já finalizado");
                }
            }
        }
    }

    let mut jogo = Jogo::iniciar_jogo(0);
    println!("Jogo iniciado");

    jogo.atualizar_pontuacao(100);
    println!("Pontuação atualizada");

    jogo.atualizar_pontuacao(200);
    println!("Pontuação atualizada");

    jogo.finalizar_jogo();
    println!("Jogo finalizado");

    jogo.atualizar_pontuacao(300);
    println!("Pontuação atualizada");
}

// 8. Função que Aceita e Retorna `Structs` com Empréstimo
// Implemente  uma  função  `juntar_nomes`  que  recebe  referências  imutáveis  para  duas  `structs`
// chamadas  `Pessoa`,  cada  uma  contendo  o  campo  `nome`.  A  função  deve  retornar  uma  nova
// `Pessoa`  com  o  nome  concatenado.  O  programa  principal  deve  testar  a  função  sem  mover  as
// `structs` originais.
fn exercicio8() {
    struct Pessoa {
        nome: String,
    }

    fn juntar_nomes(pessoa1: &Pessoa, pessoa2: &Pessoa) -> Pessoa {
        Pessoa {
            nome: format!("{} {}", pessoa1.nome, pessoa2.nome),
        }
    }

    let pessoa1 = Pessoa {
        nome: String::from("Alice"),
    };

    let pessoa2 = Pessoa {
        nome: String::from("Bob"),
    };

    let pessoa3 = juntar_nomes(&pessoa1, &pessoa2);

    println!("Nome da pessoa 1: {}", pessoa1.nome);
    println!("Nome da pessoa 2: {}", pessoa2.nome);
    println!("Nome da pessoa 3: {}", pessoa3.nome);
}

// 9. Ciclo de Empréstimo e Match com Enum
// Crie um `enum` chamado `OperacaoBancaria` que tenha as variantes `Deposito(f64)`, `Saque(f64)`
// e `ConsultaSaldo`. Crie uma função que recebe uma referência mutável para o saldo de uma conta
// e  um  `OperacaoBancaria`,  e  realiza  a  operação  correta,  utilizando  `match`.  Garanta  que  o  saldo
// não possa ficar negativo e imprima mensagens apropriadas para cada operação.
fn exercicio9() {
    enum OperacaoBancaria {
        Deposito(f64),
        Saque(f64),
        ConsultaSaldo,
    }

    fn operacao_bancaria(saldo: &mut f64, operacao: OperacaoBancaria) {
        match operacao {
            OperacaoBancaria::Deposito(valor) => {
                *saldo += valor;
                println!("Depósito de R$ {:.2} realizado", valor);
            }
            OperacaoBancaria::Saque(valor) => {
                if *saldo >= valor {
                    *saldo -= valor;
                    println!("Saque de R$ {:.2} realizado", valor);
                } else {
                    println!("Saldo insuficiente para saque de R$ {:.2}", valor);
                }
            }
            OperacaoBancaria::ConsultaSaldo => {
                println!("Saldo atual: R$ {:.2}", saldo);
            }
        }
    }

    let mut saldo = 100.0;
    operacao_bancaria(&mut saldo, OperacaoBancaria::ConsultaSaldo);
    operacao_bancaria(&mut saldo, OperacaoBancaria::Deposito(50.0));
    operacao_bancaria(&mut saldo, OperacaoBancaria::Saque(75.0));
    operacao_bancaria(&mut saldo, OperacaoBancaria::Saque(100.0));
    operacao_bancaria(&mut saldo, OperacaoBancaria::ConsultaSaldo);
}

// 10. Implementando um Sistema de Pedidos com Enum e Struct
// Crie uma `struct` chamada `Pedido` que tenha os campos `numero_pedido`, `cliente`, e `itens` (um
// vetor de strings). Em seguida, crie um `enum` chamado `StatusPedido` com as variantes
// `Processando`,  `Enviado(u32)`,  e  `Concluido`.  Escreva  um  programa  que  atualiza  o  status  de  um
// pedido usando `match` e imprime uma mensagem apropriada para cada estado.
fn exercicio10() {
    struct Pedido {
        numero_pedido: u32,
        cliente: String,
        itens: Vec<String>,
    }

    enum StatusPedido {
        Processando,
        Enviado(u32),
        Concluido,
    }

    impl Pedido {
        fn atualizar_status(&self, status: StatusPedido) {
            match status {
                StatusPedido::Processando => {
                    println!("Pedido {} está sendo processado", self.numero_pedido);
                }
                StatusPedido::Enviado(dias) => {
                    println!(
                        "Pedido {} enviado, previsão de entrega em {} dias",
                        self.numero_pedido, dias
                    );
                }
                StatusPedido::Concluido => {
                    println!("Pedido {} concluído", self.numero_pedido);
                }
            }
        }
    }

    let pedido = Pedido {
        numero_pedido: 1234,
        cliente: String::from("Alice"),
        itens: vec![String::from("Arroz"), String::from("Feijão")],
    };

    pedido.atualizar_status(StatusPedido::Processando);
    pedido.atualizar_status(StatusPedido::Enviado(3));
    pedido.atualizar_status(StatusPedido::Concluido);
}
