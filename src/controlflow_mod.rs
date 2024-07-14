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

pub mod loops {

    pub fn test_for(){
        for i in 0..5 {
            print!("{}",i);
        }
    }

    pub fn test_while(){
        let i = 3;
        while i > 0 {
            print!("Waiting...;");
        }
    }

    pub fn test_loop(){
        let mut i = 5;
        loop  {
            print!("Waiting...;");

            if i == 5 {
                break;
            }
            i += 1;
        }
    }
}
