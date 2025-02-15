use std::collections::HashSet;

pub fn test_hashset_type() {
    let mut planet_list = HashSet::from(["Mercury", "Venus", "Earth"]);

    let planet_list_more = HashSet::from(["Earth", "Mars", "Jupiter"]) ;   

    let planet_diff = planet_list.difference(&planet_list_more);
    let planet_symdiff = planet_list.symmetric_difference(&planet_list_more);

    for planet in planet_diff {
        println!("Thanks for adding {}", planet);
    }

    println!("\n");

    planet_list.insert("Saturn");
    planet_list.insert("Uranus");
    planet_list.insert("Neptune");
    planet_list.insert("Pluto");

    for planet in planet_list {
        println!("Thanks for adding {}", planet);
    }

}

