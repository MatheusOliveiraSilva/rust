// Este arquivo é apenas para testar nossa biblioteca
use restaurant::front_of_house;

fn main() {
    println!("=== Testando o restaurante ===");
    
    // Usando funções diretamente dos módulos
    front_of_house::hosting::add_to_waitlist();
    front_of_house::serving::take_order();
    front_of_house::serving::serve_order();
    front_of_house::hosting::seat_at_table();
    front_of_house::serving::take_payment();
    
    println!("Cliente atendido com sucesso!");
}