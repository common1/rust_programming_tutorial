// #![allow(warnings)]

pub mod helpers;

fn main() {
    println!("Hello from Rust, Harrie smulders");
    // test_func();
    
    let myresult = helpers::namehelpers::get_full_name("Harrie", "Smulders");
    println!("Hello from {0}", myresult);

    let new_age = helpers::private_fns::get_age_plus_5(23);
    println!("The new age is {0}", new_age);
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