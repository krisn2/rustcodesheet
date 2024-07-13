pub mod data_type_mod;
pub mod controlflow_mod;

fn main() {
     // All data type in that module
     data_type_mod::data_types();

     // All control flow in that module like conditional and loops
     controlflow_mod::conditional::test_if();
}