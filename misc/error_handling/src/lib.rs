use std::fs::File;
use std::io::{self, ErrorKind, Read};

pub fn dividir(a: i32, b: i32) -> Result<f32, String> {
    if b == 0 {
        Err("Divisão por zero!".to_string())
    } else {
        Ok(a as f32 / b as f32)
    }
}

pub fn ler_arquivo() -> Result<String, io::Error> {
    let mut file = File::open("arquivo.txt")?;
    let mut conteudo = String::new();
    file.read_to_string(&mut conteudo)?;
    Ok(conteudo)
}

pub fn abrir_arquivo() -> File {
    let file_result = File::open("arquivo.txt");

    match file_result {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("arquivo.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problema ao criar o arquivo: {:?}", e),
            },
            other_error => {
                panic!("Problema ao abrir o arquivo: {:?}", other_error);
            }
        },
    }
}
