pub mod conditional {
    pub fn test_if (){
        let age_to_drive:u8 = 18u8;
        
        println!("Enter your age: ");
        let mut age = String::new();
        std::io::stdin().read_line(&mut age).unwrap();

        if age.trim().parse::<u8>().unwrap() >= age_to_drive {
            println!("You can drive");
        } else {
            println!("You cannot drive");
        }

    }
}
 