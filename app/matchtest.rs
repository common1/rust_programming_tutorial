pub fn test_match_string() -> u32 {
    let car_manufacturer = "Porsche";

    match car_manufacturer {
        "Hyundai" => 30000,
        "Porsche" => 90000,
        _ => 0
    }
}

pub fn test_match_array() {
    let prices: [u32; 4]= [30000,50000, 90000, 120000];

    match prices[0..=3] {
        [30000, 50000] => println!("You have some reasonably priced cars"),
        [30000, 50000, ..] => println!("You have a variety of cars"),        
        _ => println!("You don't have any reasonably priced cars"),
    }
}

pub fn test_match_int() {
    let myage: u16 = 1;

    let y: u8 = 5;

    match myage {
        0 => println!("You are a newborn infant"),
        1..=35 if y == 5 => println!("Your age is up to 35, y is 5"),
        1..=35 if y != 5 => println!("Your age is up to 35, y is not 5"),
        1..=35 => println!("Your age is 1 to 35, y is not defined"),
        36..=149 => println!("Age is between 36 and 149"),
        150.. => println!("Your age is over 150"),
    }
}