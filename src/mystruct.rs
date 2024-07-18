use std::cell::Cell;

/// Structure in rust
/// for making struct we use struct keyword
/// below is an examples 
/// for making mutable struct we use cell type 
/// 
/// Here we use 2 attrubutes #[allow(dead_code)] and #[derive(Debug)]

#[allow(dead_code)]
#[derive(Debug)]
enum Color {
    Sliver,
    White,
    Black, 
}
#[allow(dead_code)]
#[derive(Debug)]
struct Car {
    manufacturer: String,
    model: String,
    year: u16,
    color:Color
}

fn new_car() -> Car {
    let c1 = Car{
        manufacturer: String::from("Mustang"),
        model: String::from("1969"),
        year: 1969,
        color: Color::Black
    };
    return c1;
}

pub fn test_create_car() {
    let c1 = new_car();
    println!("{:?}",c1);
}

// #[allow(dead_code)]
pub struct Person<'a> {
    pub first_name: Cell<&'a str>,
    pub last_name: String,
    pub birth_year: u16,
    pub birth_month: u8,
}
 fn new_person() -> Person<'static> {
    let p1 = Person{
        // first_name: Cell::new("Krishna"),
        first_name: Cell::from("Krishna"),
        last_name: String::from("sarone"),
        birth_year: 2004,
        birth_month: 12
    };
    p1.first_name.set("Ragnar");
    return p1;
}

pub fn test_create_person() {
    let p1 = new_person();
    println!("fist name: {}, last name: {}, birth year: {}, birth month: {}", p1.first_name.get(), p1.last_name, p1.birth_year, p1.birth_month);
}