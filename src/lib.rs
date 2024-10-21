use core::str;

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
