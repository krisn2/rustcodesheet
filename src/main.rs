pub mod data_type_mod;
pub mod controlflow_mod;
pub mod closures;

fn main() {
     // All data type in that module
     data_type_mod::data_types();

     // All control flow in that module like conditional and loops
     controlflow_mod::conditional::test_if();
     controlflow_mod::loops::test_for();
     controlflow_mod::loops::test_while();
     controlflow_mod::loops::test_loop();

     // All closure in that module
     closures::test_closure();
}