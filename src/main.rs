pub mod modules;
pub mod data_type_mod;

fn main() {
     data_type_mod::data_types();
     let value_of_fn = modules::get_name("krishn", "sarone");
     println!("value of fn get name: {}", value_of_fn);
}