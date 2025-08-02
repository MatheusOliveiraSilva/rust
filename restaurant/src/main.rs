// Este arquivo é apenas para testar nossa biblioteca
use restaurant::{eat_at_restaurant, front_of_house};

fn main() {
    println!("=== Testando o restaurante ===");
    
    // Usando a função principal da biblioteca
    eat_at_restaurant();
    
    println!("\n=== Simulando atendimento completo ===");
    
    // Usando funções diretamente dos módulos
    front_of_house::hosting::add_to_waitlist();
    front_of_house::serving::take_order();
    front_of_house::serving::serve_order();
    front_of_house::hosting::seat_at_table();
    front_of_house::serving::take_payment();
    
    println!("Cliente atendido com sucesso!");
}