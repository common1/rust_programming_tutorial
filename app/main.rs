// #![allow(warnings)]

pub mod helpers;

fn main() {
    println!("Hello from Rust, Harrie smulders");
    // test_func();
    
    // let myresult = helpers::namehelpers::get_full_name("Harrie", "Smulders");
    // println!("Hello from {0}", myresult);

    // let new_age = helpers::private_fns::get_age_plus_5(23);
    // println!("The new age is {0}", new_age);

    // test_if();
    // test_while();
    // test_loop();
    test_for();
}

fn test_for() {
    let ages = [14, 18, 26, 35, 41];
    let age_to_drive = 16i32;

    for value in ages {
        println!("The current value is {0}", value);
        if value >= age_to_drive {
            println!("You are old enough to drive!");
        }
        else {
            println!("You need to wait a little bit more ...");
        }
    }
}

#[allow(dead_code)]
fn test_loop() {
    let mut x = 1;
    loop {
        println!("Hello from Rust!");
        if x > 5 {
            break;
        }
        x += 1;
    }
}

#[allow(dead_code)]
fn test_while() {
    let age_to_drive = 16u8;

    let mut current_age = 0u8;
    while current_age < age_to_drive {
        println!("Waiting ... ");
        current_age += 1;
        if current_age == 6 {
            break;
        }
    }
}

#[allow(dead_code)]
fn test_if() {
    let age_to_drive = 16u8;

    println!("Enter the person's age:");
    let myinput = &mut String::from("");
    std::io::stdin().read_line(myinput).unwrap();

    // let age = myinput.replace("\n", "").parse::<u8>().unwrap();
    let age = myinput.replace("\n", "").parse::<u8>().unwrap();
    if age > age_to_drive{
        println!("Issuing driver's license, because they are old enough");
    }
    else if age == 16 {
        println!("You are just on the verge of being old enough! Wait one more year");
    }
    else {
        println!("Wait a bit longer,m you eren't old enough for a driver's license");
    }

    let drivers_license = if age >= 16 { true } else { false };
    println!("{}", drivers_license);
}

#[allow(dead_code)]
fn test_func() {
    // let x:  u8 = 225;
    // println!("{:?}", x);

    // let x:  f32 = 225.0;
    // println!("{:?}", x);

    // let x: u8 = 225;
    // let y: u8 = x - 5;
    // println!("x: {:?} y: {:?}", x, y);

    // let x: f32 = 225.0;
    // let y: u8 = x as u8 - 5;
    // println!("x: {:?} y: {:?}", x, y);

    // let mut iamold: bool = true;
    // iamold = false;
    // println!("{}",iamold);

    // let mystr: char = 'A';
    // println!("{}", mystr);

    // let mut first_name = "Trevor";
    // println!("{}", first_name);
    // first_name = "Bob";
    // println!("{}", first_name);

    // let name = ("Trevor", "Sullivan", 40 as u8);
    // println!("{:?}", name);

    // let ages: [u16; 3] = [40, 45, 50];
    // println!("{:#?}", ages);

    let ages: [u16; 6] = [40, 45, 50, 55, 60, 65];
    println!("{:#?}", ages);

    let new_ages = &ages[0..=5];
    println!("{:#?}", new_ages);

}