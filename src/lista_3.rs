struct Coordenada(f64, f64);

fn distancia(p1: Coordenada, p2: Coordenada) -> f64 {
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;
    (dx * dx + dy * dy).sqrt()
}

fn struct_com_tuplas() {
    let coord1: Coordenada = Coordenada(1.0, 2.0);
    let coord2: Coordenada = Coordenada(3.0, 4.0);

    distancia(coord1, coord2);
}

pub enum Operacao {
    Soma(i32, i32),
    Subtracao(i32, i32),
    Multiplicacao(i32, i32),
    Divisao(i32, i32),
    Quadrado(i32),
}

pub fn calcular(op: Operacao) -> i32 {
    match op {
        Operacao::Soma(a, b) => a + b,
        Operacao::Subtracao(a, b) => a - b,
        Operacao::Multiplicacao(a, b) => a * b,
        Operacao::Divisao(a, b) => a / b,
        Operacao::Quadrado(a) => a * a,
    }
}

fn maior(numeros: &[i32]) -> Option<i32> {
    if numeros.is_empty() {
        None
    } else {
        Some(*numeros.iter().max().unwrap())
    }
}

pub enum Estado {
    Ligado,
    Desligado,
}

impl Estado {
    fn descricao(&self) -> &str {
        match self {
            Estado::Ligado => "Ligado",
            Estado::Desligado => "Desligado",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distancia() {
        let coord1: Coordenada = Coordenada(1.0, 2.0);
        let coord2: Coordenada = Coordenada(3.0, 4.0);
        assert_eq!(distancia(coord1, coord2), 2.8284271247461903);
    }

    #[test]
    fn test_struct_com_tuplas() {
        struct_com_tuplas();
    }
}
