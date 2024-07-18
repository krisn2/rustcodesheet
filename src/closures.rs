// closure is an anonymous function that can be passed as an argument to another function
// closure is a function that can be stored in a variable or passed as an argument to another function
// it's like callback function in javascript


pub fn test_closure(){
    // add closure 
    let add = |x:u32, y:u32|{
        println!("x:{} y:{}", x, y);
        x+y
    };
    let result = add(3,5);

    // print closure
    let print_result = || println!("Result:{}", result);
    print_result();

    // mut closure
    let mut  name = String::from("Krishna");
    let mut change_name = || {
        name = String::from("John");
    };
    change_name();
    println!("Name: {}", name);
}