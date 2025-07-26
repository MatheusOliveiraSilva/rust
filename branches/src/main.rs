fn main() {
    println!("=== RANGES EM RUST ===\n");

    println!("1. Range Exclusivo (a..b) - não inclui o fim");
    println!("Range 1..4:");
    for i in 1..4 {
        print!("{i} ");
    }
    println!("(não inclui o 4)\n");

    println!("2. Range Inclusivo (a..=b) - inclui o fim");
    println!("Range 1..=4:");
    for i in 1..=4 {
        print!("{i} ");
    }
    println!("(inclui o 4)\n");

    println!("3. Range Reverso com .rev()");
    println!("(1..4).rev():");
    for i in (1..4).rev() {
        print!("{i} ");
    }
    println!();
    
    println!("(1..=4).rev():");
    for i in (1..=4).rev() {
        print!("{i} ");
    }
    println!("\n");

    println!("4. Ranges Parciais");
    let arr = [10, 20, 30, 40, 50];
    
    println!("Array: {:?}", arr);
    println!("arr[1..4]: {:?}", &arr[1..4]);    // Do índice 1 até 3
    println!("arr[..3]: {:?}", &arr[..3]);      // Do início até índice 2
    println!("arr[2..]: {:?}", &arr[2..]);      // Do índice 2 até o fim
    println!("arr[..]: {:?}", &arr[..]);        // Array inteiro
    println!();

    println!("5. Ranges como Valores");
    let range1 = 0..5;
    let range2 = 0..=5;
    
    println!("Testando se 5 está em 0..5: {}", range1.contains(&5));     // false
    println!("Testando se 5 está em 0..=5: {}", range2.contains(&5));    // true
    println!();

    println!("6. Métodos Úteis de Ranges");
    let range_exclusive = 1..10;
    let range_inclusive = 1..=10;
    
    println!("Tamanho do range 1..10: {}", range_exclusive.len());
    println!("Contando range 1..=10: {}", range_inclusive.clone().count());
    println!("Range vazio? {}", (5..5).is_empty());
    println!("Range não-vazio? {}", (1..5).is_empty());
    println!();

    println!("7. Ranges com Chars");
    println!("Letras de 'a' a 'e':");
    for c in 'a'..='e' {
        print!("{c} ");
    }
    println!("\n");

    println!("8. Collect em Vector");
    let numeros: Vec<i32> = (1..6).collect();
    println!("Range 1..6 coletado: {:?}", numeros);
    
    let reverso: Vec<i32> = (1..6).rev().collect();
    println!("Range 1..6 reverso: {:?}", reverso);
    println!();

    println!("9. Step (passo) com .step_by()");
    println!("Números pares de 0 a 10:");
    for i in (0..=10).step_by(2) {
        print!("{i} ");
    }
    println!();
    
    println!("Números ímpares de 1 a 10:");
    for i in (1..=10).step_by(2) {
        print!("{i} ");
    }
    println!("\n");

    println!("10. Range Infinito (a partir de um valor)");
    println!("Primeiros 5 números a partir de 100:");
    for i in (100..).take(5) {
        print!("{i} ");
    }
    println!("\n");

    // Exemplo original (contagem regressiva)
    println!("=== CONTAGEM REGRESSIVA ===");
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}