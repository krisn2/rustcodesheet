use std::collections::HashSet;  

pub fn test_hashset(){
    let  planet_list:HashSet<&str> = HashSet::from(["Mercury", "Venus", "Earth"]);
    let planet_list2:HashSet<&str> = HashSet::from(["Earth", "Mars", "Jupiter",]);
    
    let planet_diff =planet_list.difference(&planet_list2);

    for planet in planet_diff {
        println!(" Adding..{}", planet);
    }
}
   