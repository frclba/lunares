use core::str;
pub mod diffie_hellman;

pub fn string_on_stack_and_heap() {
    for i in 0..10 {
        println!("{}", i);
    }
    let string_on_stack: &str = "Hello, world!";
    let mut string_on_heap: String = String::from("Hello, world!");
    string_on_heap.push_str("with push_str!");

    println!("{}", string_on_heap);
    println!("{}", string_on_stack);
}

pub fn soma(a: i32, b: i32) -> i32 {
    a + b
}

pub fn divisao_segura(a: i32, b: i32) -> Option<f32> {
    if b == 0 {
        None
    } else {
        Some(a as f32 / b as f32)
    }
}

pub fn break_ownership() {
    let string_a = String::from("Hello, world!");
    let mut string_b = string_a;
    change_string(&mut string_b, "mutation inside move!");
    println!("{}", string_b);
}

pub fn change_string(r: &mut String, to_push: &str) {
    r.push_str(to_push);
}

pub fn testing_sum() {
    let a = 10;
    let b = 20;
    let c = soma(a, b);
    println!("A soma de {} com {} é {}", a, b, c);

    let d: Option<f32> = divisao_segura(a, b);
    match d {
        Some(valor) => println!("A divisão de {} por {} é {}", a, b, valor),
        None => println!("Não é possível dividir {} por {}", a, b),
    }
    string_on_stack_and_heap();
    break_ownership();
}

pub fn cloning_and_moving(first: String, second: String) {
    let moved = first;
    let cloned = second.clone();
    println!("Moved: {}", moved);
    println!("Cloned: {}", cloned);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soma() {
        assert_eq!(soma(2, 3), 5);
    }

    #[test]
    fn test_divisao_segura() {
        assert_eq!(divisao_segura(10, 2), Some(5.0));
        assert_eq!(divisao_segura(10, 0), None);
    }

    #[test]
    fn test_change_string() {
        let mut test_string = String::from("Hello");
        change_string(&mut test_string, ", world!");
        assert_eq!(test_string, "Hello, world!");
    }

    #[test]
    fn test_cloning_and_moving() {
        let first = String::from("Hello");
        let second = String::from("World");
        cloning_and_moving(first.clone(), second.clone());
        // Check if original strings still exist after cloning
        assert_eq!(first, "Hello");
        assert_eq!(second, "World");
    }
}
