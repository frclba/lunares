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

// 1. Mapeamento de Parentesco
// Crie uma estrutura que modele um sistema de árvore genealógica. Utilize um
// `HashMap<String, Vec<String>>` para representar a relação entre pais e filhos,
// onde cada chave é o nome de uma pessoa e o valor é uma lista dos nomes de
// seus filhos. Implemente uma função que, dada uma pessoa, retorne todos os
// seus descendentes diretos e indiretos.
fn exercicio1() {
    let mut arvore_genealogica = HashMap::new();

    // Adicionando relações de parentesco
    arvore_genealogica.insert(
        "João".to_string(),
        vec!["Maria".to_string(), "José".to_string()],
    );
    arvore_genealogica.insert("Maria".to_string(), vec!["Ana".to_string()]);
    arvore_genealogica.insert("José".to_string(), vec!["Carlos".to_string()]);
    arvore_genealogica.insert("Ana".to_string(), vec!["Pedro".to_string()]);

    let pessoa = "João";
    let descendentes = obter_descendentes(&arvore_genealogica, pessoa);
    println!("Descendentes de {}: {:?}", pessoa, descendentes);
}

fn obter_descendentes(arvore: &HashMap<String, Vec<String>>, pessoa: &str) -> HashSet<String> {
    let mut descendentes = HashSet::new();
    let mut pilha = vec![pessoa.to_string()];

    while let Some(atual) = pilha.pop() {
        if let Some(filhos) = arvore.get(&atual) {
            for filho in filhos {
                if descendentes.insert(filho.clone()) {
                    pilha.push(filho.clone());
                }
            }
        }
    }

    descendentes
}

// 2. Contagem de Caracteres Repetidos com Unicode
// Modifique o exercício de contagem de ocorrências de palavras para contar a
// frequência de caracteres em uma string que contém caracteres Unicode. Utilize
// um `HashMap<char, u32>` para armazenar as frequências.

fn exercicio2() {
    let texto = "Olá, mundo! 🌍 Olá, Rust! 🦀";

    let frequencias = contar_frequencia_caracteres(texto);

    println!("Frequências de caracteres:");
    for (caractere, frequencia) in &frequencias {
        println!("'{}': {}", caractere, frequencia);
    }
}

fn contar_frequencia_caracteres(texto: &str) -> HashMap<char, u32> {
    let mut mapa = HashMap::new();

    for caractere in texto.chars() {
        *mapa.entry(caractere).or_insert(0) += 1;
    }

    mapa
}

// 3. Colapso de HashMap com Chaves Compostas
// Dado um `HashMap<(String, String), u32>`, onde a primeira parte da chave é
// uma categoria e a segunda é um item, crie uma função que retorne um novo
// `HashMap<String, u32>` colapsando todas as categorias e somando os valores
// de seus itens correspondentes.

fn exercicio3() {
    let mut mapa_original = HashMap::new();
    mapa_original.insert(("Frutas".to_string(), "Maçã".to_string()), 10);
    mapa_original.insert(("Frutas".to_string(), "Banana".to_string()), 5);
    mapa_original.insert(("Legumes".to_string(), "Cenoura".to_string()), 7);
    mapa_original.insert(("Frutas".to_string(), "Laranja".to_string()), 8);
    mapa_original.insert(("Legumes".to_string(), "Batata".to_string()), 12);

    let mapa_colapsado = colapsar_categorias(&mapa_original);

    println!("Mapa colapsado: {:?}", mapa_colapsado);
}

fn colapsar_categorias(mapa: &HashMap<(String, String), u32>) -> HashMap<String, u32> {
    let mut resultado = HashMap::new();

    for ((categoria, _item), valor) in mapa {
        *resultado.entry(categoria.clone()).or_insert(0) += valor;
    }

    resultado
}

// 4. Diferença entre HashMaps
// Implemente uma função que receba dois `HashMap<String, u32>` e retorne um
// novo `HashMap<String, u32>` contendo apenas os pares chave-valor
// presentes no primeiro `HashMap`, mas não no segundo.

fn exercicio4() {
    let mut mapa1 = HashMap::new();
    mapa1.insert("A".to_string(), 1);
    mapa1.insert("B".to_string(), 2);
    mapa1.insert("C".to_string(), 3);

    let mut mapa2 = HashMap::new();
    mapa2.insert("B".to_string(), 2);
    mapa2.insert("D".to_string(), 4);

    let diferenca = diferenca_hashmaps(&mapa1, &mapa2);

    println!("Diferença: {:?}", diferenca);
}

fn diferenca_hashmaps(
    mapa1: &HashMap<String, u32>,
    mapa2: &HashMap<String, u32>,
) -> HashMap<String, u32> {
    let mut resultado = HashMap::new();

    for (chave, valor) in mapa1 {
        if !mapa2.contains_key(chave) {
            resultado.insert(chave.clone(), *valor);
        }
    }

    resultado
}

// 5. HashMap com Tuplas como Valores
// Crie um sistema de inventário para um jogo, onde cada item é mapeado para
// uma tupla `(quantidade, valor)`. O programa deve permitir que o jogador
// adicione itens, remova itens e calcule o valor total do inventário.

fn exercicio5() {
    let mut inventario = HashMap::new();

    adicionar_item(&mut inventario, "Espada".to_string(), 1, 150);
    adicionar_item(&mut inventario, "Poção".to_string(), 5, 10);
    adicionar_item(&mut inventario, "Escudo".to_string(), 1, 100);

    remover_item(&mut inventario, "Poção", 2);

    let valor_total = calcular_valor_total(&inventario);

    println!("Inventário: {:?}", inventario);
    println!("Valor total do inventário: {}", valor_total);
}

fn adicionar_item(
    inventario: &mut HashMap<String, (u32, u32)>,
    item: String,
    quantidade: u32,
    valor: u32,
) {
    let entrada = inventario.entry(item).or_insert((0, valor));
    entrada.0 += quantidade;
}

fn remover_item(inventario: &mut HashMap<String, (u32, u32)>, item: &str, quantidade: u32) {
    if let Some(entrada) = inventario.get_mut(item) {
        if entrada.0 > quantidade {
            entrada.0 -= quantidade;
        } else {
            inventario.remove(item);
        }
    }
}

fn calcular_valor_total(inventario: &HashMap<String, (u32, u32)>) -> u32 {
    inventario.values().map(|(qtd, val)| qtd * val).sum()
}

// 6. Cálculo de Média com Erros em Entradas
// Implemente uma função que receba um vetor de strings representando
// números e calcule a média apenas dos números válidos. A função deve
// retornar um `Result<f64, String>` que contenha a média ou uma mensagem de
// erro se nenhum número for válido.
fn exercicio6() {
    let entradas = vec![
        "10".to_string(),
        "20".to_string(),
        "abc".to_string(),
        "30".to_string(),
        "def".to_string(),
    ];

    match calcular_media(&entradas) {
        Ok(media) => println!("A média é: {}", media),
        Err(erro) => println!("Erro: {}", erro),
    }
}

fn calcular_media(numeros: &Vec<String>) -> Result<f64, String> {
    let mut soma = 0.0;
    let mut contador = 0;

    for s in numeros {
        if let Ok(n) = s.parse::<f64>() {
            soma += n;
            contador += 1;
        }
    }

    if contador > 0 {
        Ok(soma / contador as f64)
    } else {
        Err("Nenhum número válido foi fornecido.".to_string())
    }
}

// 7. Mapeamento de Resultados de Funções
// Dado um vetor de funções que retornam um `Result<i32, String>`, implemente
// uma função que execute todas as funções em sequência e colete todos os
// resultados bem-sucedidos em um vetor. Se todas as funções falharem, retorne
// um `Err` com uma mensagem de erro agregada.

fn exercicio7() {
    let funcoes: Vec<Box<dyn Fn() -> Result<i32, String>>> = vec![
        Box::new(|| Ok(10)),
        Box::new(|| Err("Erro 1".to_string())),
        Box::new(|| Ok(20)),
        Box::new(|| Err("Erro 2".to_string())),
    ];

    match executar_funcoes(funcoes) {
        Ok(resultados) => println!("Resultados: {:?}", resultados),
        Err(erro) => println!("Erro: {}", erro),
    }
}

fn executar_funcoes(
    funcoes: Vec<Box<dyn Fn() -> Result<i32, String>>>,
) -> Result<Vec<i32>, String> {
    let mut resultados = Vec::new();
    let mut erros = Vec::new();

    for func in funcoes {
        match func() {
            Ok(valor) => resultados.push(valor),
            Err(erro) => erros.push(erro),
        }
    }

    if !resultados.is_empty() {
        Ok(resultados)
    } else {
        Err(format!("Todas as funções falharam: {:?}", erros))
    }
}

// 8. Processamento de Arquivos em Lote com Erro Detalhado
// Escreva uma função que recebe uma lista de caminhos para arquivos, lê o
// conteúdo de cada um e retorna um `Result<HashMap<String, String>,
// Vec<String>>`. A chave do `HashMap` deve ser o nome do arquivo e o valor
// seu conteúdo. Em caso de falha, o retorno deve conter uma lista dos arquivos
// que não puderam ser lidos.

fn exercicio8() {
    let arquivos = vec!["arquivo1.txt", "arquivo2.txt", "arquivo_inexistente.txt"];
    match ler_arquivos(arquivos) {
        Ok(conteudos) => {
            println!("Conteúdos dos arquivos: {:?}", conteudos);
        }
        Err(erros) => {
            println!("Falha ao ler os arquivos: {:?}", erros);
        }
    }
}

fn ler_arquivos(caminhos: Vec<&str>) -> Result<HashMap<String, String>, Vec<String>> {
    let mut conteudos = HashMap::new();
    let mut erros = Vec::new();

    for caminho in caminhos {
        match fs::read_to_string(caminho) {
            Ok(conteudo) => {
                conteudos.insert(caminho.to_string(), conteudo);
            }
            Err(_) => {
                erros.push(caminho.to_string());
            }
        }
    }

    if erros.is_empty() {
        Ok(conteudos)
    } else {
        Err(erros)
    }
}

// 9. Conversão Segura com Erros Detalhados
// Implemente uma função que receba um vetor de strings e tente convertê-las
// para números inteiros. Para cada conversão falha, adicione um detalhe do erro
// (como o índice da string falha) em um vetor de erros. A função deve retornar
// um `Result<Vec<i32>, Vec<String>>`.

fn exercicio9() {
    let entradas = vec![
        "10".to_string(),
        "abc".to_string(),
        "20".to_string(),
        "def".to_string(),
    ];

    match converter_strings(&entradas) {
        Ok(numeros) => println!("Números convertidos: {:?}", numeros),
        Err(erros) => println!("Erros: {:?}", erros),
    }
}

fn converter_strings(entradas: &Vec<String>) -> Result<Vec<i32>, Vec<String>> {
    let mut numeros = Vec::new();
    let mut erros = Vec::new();

    for (indice, entrada) in entradas.iter().enumerate() {
        match entrada.parse::<i32>() {
            Ok(numero) => numeros.push(numero),
            Err(_) => erros.push(format!("Índice {}: '{}'", indice, entrada)),
        }
    }

    if erros.is_empty() {
        Ok(numeros)
    } else {
        Err(erros)
    }
}

// 10. Erro ao Aplicar Funções a HashMaps
// Escreva uma função que aplique uma operação binária entre os valores de
// dois `HashMap<String, i32>`. Se qualquer chave estiver presente em apenas
// um dos mapas, a função deve retornar um erro detalhado informando quais
// chaves estavam ausentes. Retorne um `Result<HashMap<String, i32>,
// Vec<String>>`.

fn exercicio10() {
    let mut mapa1 = HashMap::new();
    mapa1.insert("A".to_string(), 5);
    mapa1.insert("B".to_string(), 10);

    let mut mapa2 = HashMap::new();
    mapa2.insert("B".to_string(), 3);
    mapa2.insert("C".to_string(), 8);

    match aplicar_operacao(&mapa1, &mapa2, |a, b| a + b) {
        Ok(resultado) => println!("Resultado: {:?}", resultado),
        Err(erros) => println!("Chaves ausentes: {:?}", erros),
    }
}

fn aplicar_operacao<F>(
    mapa1: &HashMap<String, i32>,
    mapa2: &HashMap<String, i32>,
    operacao: F,
) -> Result<HashMap<String, i32>, Vec<String>>
where
    F: Fn(i32, i32) -> i32,
{
    let mut resultado = HashMap::new();
    let mut chaves_ausentes = Vec::new();

    let chaves: HashSet<_> = mapa1.keys().chain(mapa2.keys()).collect();

    for chave in chaves {
        match (mapa1.get(chave), mapa2.get(chave)) {
            (Some(valor1), Some(valor2)) => {
                resultado.insert(chave.clone(), operacao(*valor1, *valor2));
            }
            _ => {
                chaves_ausentes.push(chave.clone());
            }
        }
    }

    if chaves_ausentes.is_empty() {
        Ok(resultado)
    } else {
        Err(chaves_ausentes)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn teste_exercicio1() {
        let mut arvore_genealogica = HashMap::new();

        arvore_genealogica.insert(
            "João".to_string(),
            vec!["Maria".to_string(), "José".to_string()],
        );
        arvore_genealogica.insert("Maria".to_string(), vec!["Ana".to_string()]);
        arvore_genealogica.insert("José".to_string(), vec!["Carlos".to_string()]);
        arvore_genealogica.insert("Ana".to_string(), vec!["Pedro".to_string()]);

        let pessoa = "João";
        let descendentes = obter_descendentes(&arvore_genealogica, pessoa);

        let esperado: HashSet<String> = ["Maria", "José", "Ana", "Carlos", "Pedro"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        assert_eq!(descendentes, esperado);
    }

    #[test]
    fn teste_exercicio2() {
        let texto = "aáâäãåāabcç";
        let frequencias = contar_frequencia_caracteres(texto);

        let mut esperado = HashMap::new();
        esperado.insert('a', 2);
        esperado.insert('á', 1);
        esperado.insert('â', 1);
        esperado.insert('ä', 1);
        esperado.insert('ã', 1);
        esperado.insert('å', 1);
        esperado.insert('ā', 1);
        esperado.insert('b', 1);
        esperado.insert('c', 2);
        esperado.insert('ç', 1);

        assert_eq!(frequencias, esperado);
    }

    #[test]
    fn teste_exercicio3() {
        let mut mapa_original = HashMap::new();
        mapa_original.insert(("Cat1".to_string(), "Item1".to_string()), 3);
        mapa_original.insert(("Cat1".to_string(), "Item2".to_string()), 7);
        mapa_original.insert(("Cat2".to_string(), "Item3".to_string()), 5);

        let mapa_colapsado = colapsar_categorias(&mapa_original);

        let mut esperado = HashMap::new();
        esperado.insert("Cat1".to_string(), 10);
        esperado.insert("Cat2".to_string(), 5);

        assert_eq!(mapa_colapsado, esperado);
    }
    #[test]
    fn teste_exercicio4() {
        let mut mapa1 = HashMap::new();
        mapa1.insert("X".to_string(), 10);
        mapa1.insert("Y".to_string(), 20);

        let mut mapa2 = HashMap::new();
        mapa2.insert("Y".to_string(), 20);
        mapa2.insert("Z".to_string(), 30);

        let diferenca = diferenca_hashmaps(&mapa1, &mapa2);

        let mut esperado = HashMap::new();
        esperado.insert("X".to_string(), 10);

        assert_eq!(diferenca, esperado);
    }

    #[test]
    fn teste_exercicio5() {
        let mut inventario = HashMap::new();

        adicionar_item(&mut inventario, "Flecha".to_string(), 10, 2);
        adicionar_item(&mut inventario, "Arco".to_string(), 1, 50);
        remover_item(&mut inventario, "Flecha", 5);

        let valor_total = calcular_valor_total(&inventario);

        assert_eq!(inventario.get("Flecha"), Some(&(5, 2)));
        assert_eq!(inventario.get("Arco"), Some(&(1, 50)));
        assert_eq!(valor_total, 5 * 2 + 1 * 50); // 10 + 50 = 60
    }

    #[test]
    fn teste_exercicio6() {
        let entradas_validas = vec!["1".to_string(), "2".to_string(), "3".to_string()];
        let resultado = calcular_media(&entradas_validas);
        assert_eq!(resultado, Ok(2.0));

        let entradas_invalidas = vec!["a".to_string(), "b".to_string()];
        let resultado = calcular_media(&entradas_invalidas);
        assert!(resultado.is_err());
    }
    #[test]
    fn teste_exercicio7() {
        let funcoes_sucesso: Vec<Box<dyn Fn() -> Result<i32, String>>> =
            vec![Box::new(|| Ok(1)), Box::new(|| Ok(2))];
        let resultado = executar_funcoes(funcoes_sucesso);
        assert_eq!(resultado, Ok(vec![1, 2]));

        let funcoes_falha: Vec<Box<dyn Fn() -> Result<i32, String>>> = vec![
            Box::new(|| Err("Erro A".to_string())),
            Box::new(|| Err("Erro B".to_string())),
        ];
        let resultado = executar_funcoes(funcoes_falha);
        assert!(resultado.is_err());
    }
    #[test]
    fn teste_exercicio9() {
        let entradas = vec!["1".to_string(), "x".to_string(), "3".to_string()];
        let resultado = converter_strings(&entradas);

        assert_eq!(resultado, Err(vec!["Índice 1: 'x'".to_string()]));
    }
    #[test]
    fn teste_exercicio10() {
        let mut mapa1 = HashMap::new();
        mapa1.insert("X".to_string(), 2);
        mapa1.insert("Y".to_string(), 3);

        let mut mapa2 = HashMap::new();
        mapa2.insert("X".to_string(), 4);
        mapa2.insert("Z".to_string(), 5);

        let resultado = aplicar_operacao(&mapa1, &mapa2, |a, b| a * b);

        assert_eq!(resultado, Err(vec!["Y".to_string(), "Z".to_string()]));
    }
}
