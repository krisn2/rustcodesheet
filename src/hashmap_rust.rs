use std::collections::HashMap;

pub fn test_hashmap(){
    let mut stock_list:HashMap<String,f32> = HashMap::new();

    stock_list.insert("ITC".to_string(), 474.55);
    stock_list.insert("IRFC".to_string(), 205.77);
    stock_list.insert("TCS".to_string(), 4302.40);
    stock_list.insert("LIC".to_string(), 1108.25);

    println!("{:#?}", stock_list);

    stock_list.remove(&("LIC".to_string()));
    println!("{:#?}", stock_list);
}