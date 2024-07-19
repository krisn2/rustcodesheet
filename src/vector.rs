pub fn text_vec_int() {
    let mut my_int: Vec<i32> = Vec::new();
    //let my_int = vec![1,2,3,4,5];
    my_int.push(2);
    my_int.push(3);
    my_int.push(4);
    my_int.push(5);
    my_int.push(7);
    my_int.push(9);

    println!("my_int size: {}", my_int.len());
    println!("my_int_capacity: {}", my_int.capacity());
    println!("my_int: {:?}", my_int);

    println!("my_int[0]: {}", my_int[0]);
    println!("as_slice: {:?}", &(&my_int).as_slice()[0..5]); // slice the vector in given value

    println!("get method{:?}", my_int.get(1)); //this method return value or none if not found
}


pub fn text_vec_string(){
   let first_names = vec!["krishn".to_string(),
    "pallavi".to_string(), 
   "palash".to_string(), 
   "kanvi".to_string()];

    for first_name in first_names.iter() // iter is borrow the value from the vector so it's not use after this scope
    // use : - first_name.clone() or first_name.as_slice()
     {
        println!("Processing {}......", first_name);
    }
}