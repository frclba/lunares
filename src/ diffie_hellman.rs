const fn modpow(g: u32, exp: u128, n: u32) -> u32 {
    let mut exponente = exp;
    let mut resultado = 1;
    let mut double = g;
    while exponente > 0 {
        if exponente % 2 == 1 {
            resultado *= double;
            resultado %= n;
        }
        exponente /= 2;
        double *= double;
        double %= n;
    }
    resultado
}

fn main() {
    const PRIME: u32 = 997;
    const BASE: u32 = 3;

    const ALICE_SECRET: u128 = 123;
    const BOB_SECRET: u128 = 987;

    let alice_public = modpow(BASE, ALICE_SECRET, PRIME);
    println!("Public key of Alice: {alice_public}");

    let bob_public = modpow(BASE, BOB_SECRET, PRIME);
    println!("Public key of Bob: {bob_public}");

    let alice_shared_secret = modpow(bob_public, ALICE_SECRET, PRIME);
    println!("Shared secret of Alice: {alice_shared_secret}");

    let bob_shared_secret = modpow(alice_public, BOB_SECRET, PRIME);
    println!("Shared secret of Bob: {bob_shared_secret}");

    assert_eq!(alice_shared_secret, bob_shared_secret);
    println!("The shared secret key is: {alice_shared_secret}");
}
