// #[allow(dead_code)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub birth_year: u16,
    pub birth_month: u8,
}

pub fn new_person() -> Person {
    let p1 = Person{
        first_name: String::from("Krishna"),
        last_name: String::from("sarone"),
        birth_year: 2004,
        birth_month: 12
    };
    return p1;
}