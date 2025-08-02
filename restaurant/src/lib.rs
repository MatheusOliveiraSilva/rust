pub mod front_of_house;

pub use crate::front_of_house::hosting;

// Usando o `use` que trouxemos para o escopo
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}