use crate::helpers::namehelpers::get_full_name;

pub fn test_rust_iterators() {
    let fruit_list = vec!["Strawberry", "Blueberry", "Mango", "Orange", "Apple"];

    let nut_list = vec!["Walnut", "Almonds", "Pecans", "Brazil Nuts"];

    let mut fruit_iter = fruit_list.iter();

    // for fruit in fruit_iter {
    //     println!("{}", fruit);
    // }

    fruit_iter.next();
    fruit_iter.next();
    let item01 = fruit_iter.next();
    println!("First item in iterator is: {}", item01.unwrap());

    let aggregate_foods = fruit_list.iter().chain(&nut_list);

    let all_foods: Vec<&&str> = aggregate_foods.clone().collect();

    for food in aggregate_foods {
        println!("Eating {}", food);
    }

    let fruit_list_strings = fruit_list.iter().map(|e: &&str| String::from(*e));
    let new_fruits = fruit_list_strings.map(|mut e: String| {e.push_str(" fruit"); return e;});

    new_fruits.clone().for_each(|e: String| println!("{}", e));

    println!("Last fruit is: {}", new_fruits.clone().last().unwrap());
    
    let mut stepby = new_fruits.clone().step_by(2);
    println!("Step: {}", stepby.next().unwrap());
    println!("Step: {}", stepby.next().unwrap());
    println!("Step: {}", stepby.next().unwrap());

    let first_names = vec!["Trevor", "Shannon", "James", "Tasha"];
    let first_names_strings = first_names.iter().map(|e| String::from(*e));

    let last_names = vec!["Jones", "Sullivan", "Tanner", "Redman"];
    let last_names_strings = last_names.iter().map(|e| String::from(*e));

    let full_names = first_names_strings.zip(last_names_strings);
    full_names.clone().for_each(|e| println!("{} {}", e.0, e.1));

    // for (index, value) in full_names.enumerate() {
    //     println!("Index: {0} value: {1} {2}", index, value.0, value.1);
    // }

    // full_names.skip(1).for_each(|e: (String, String)| println!("Did not skip: {} {}", e.0, e.1));
    full_names.skip(2).take(1).for_each(|e: (String, String)| println!("Did not skip: {} {}", e.0, e.1));

    let foods = vec![("potatoes", 10), ("Strawberries", 25), ("Burgers", 31)];

    let food_quantity = foods.clone().iter().fold(0u32, |mut a: u32, e: &(&str, u32)| a + e.1);
    println!("Your total food quantity is: {}", food_quantity);
 
    println!("#######################################################");
    foods.iter().peekable().next();
    let food_iter = foods.iter();
    let mut mypeekable = food_iter.peekable();
    mypeekable.next();
    let food = mypeekable.peek();
    println!("Peeking at {}", food.unwrap().0);

}

