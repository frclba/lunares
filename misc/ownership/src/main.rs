use std::collections::HashMap;

fn main() {
    let mut filmes = HashMap::new();
    filmes.insert("The Dark Knight".to_string(), 2008);
    filmes.insert("The Shawshank Redemption".to_string(), 1994);
    filmes.insert("The Godfather".to_string(), 1972);

    let ano = filmes.get("The Dark Knight");

    println!("Ano de lançamento de The Dark Knight: {:?}", ano);

    for (titulo, ano) in &filmes {
        println!("{} foi lançado em {}", titulo, ano);
    }

    let um_filme = String::from("Her");
    let ano = 2013;
    filmes.insert(um_filme, ano);
    println!("{:?}", ano);

    filmes.insert(String::from("Power"), 2020);
    filmes
        .entry(String::from("Rise of the Planet of the Apes"))
        .or_insert(2011);
}
