struct Home <PetType:Animal> {
    rooms:u8,
    members:u8,
    pet:PetType
}

trait Animal {
    fn make_sound(&self)->();
}
struct Dog {}
impl Animal for Dog {
    fn make_sound(&self)->() {
        println!("Bark");
    }
}
struct Cat {}
impl Animal for Cat {
    fn make_sound(&self)->() {
        println!("Meow");
    }
}

pub fn test_home() {
    let home = Home {
        rooms:3,
        members:4,
        pet:Cat{}
    };
    home.pet.make_sound();
    println!("{} {}", home.rooms, home.members,);
}