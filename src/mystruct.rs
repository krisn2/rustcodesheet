#[allow(dead_code)]
#[derive(Debug)]
enum Color {
    sliver,
    white,
    black, 
}
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
        color: Color::black
    };
    return c1;
}

pub fn test_create_car() {
    let c1 = new_car();
    println!("{:?}",c1);
}

// #[allow(dead_code)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub birth_year: u16,
    pub birth_month: u8,
}
 fn new_person() -> Person {
    let p1 = Person{
        first_name: String::from("Krishna"),
        last_name: String::from("sarone"),
        birth_year: 2004,
        birth_month: 12
    };
    return p1;
}

pub fn test_create_person() {
    let p1 = new_person();
    println!("fist name: {}, last name: {}, birth year: {}, birth month: {}", p1.first_name, p1.last_name, p1.birth_year, p1.birth_month);
}