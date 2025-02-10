#[derive(Debug)]
enum VehicleColor {
    Silver,
    Blue,
    Red,
    Black,
    White,
    Green
}

struct VehicleTuple(
    String,
    String,
    u16
);

#[derive(Debug)]
struct Vehicle {
    manufacturer: String,
    model: String,
    year: u16,
    color: VehicleColor,
}

fn new_vehicle() -> Vehicle {
    let v1 = Vehicle{
        manufacturer: "Porsche".to_string(),
        model: "911".to_string(),
        year: 1991,
        color: VehicleColor::Silver,
    };
    return v1
}

pub fn create_vehicle() {
    let myvehicle = new_vehicle();
    println!("{:#?}", myvehicle);
}

struct Person {
    first_name: String,
    last_name: String,
    birth_month: u8,
    birth_year: u16,
    visited_europe: bool,
}

fn new_person() -> Person {
    let p1 = Person{
        first_name: "Trevor".to_string(),
        last_name: "Sullivan".to_string(),
        birth_month: 6,
        birth_year: 1986,
        visited_europe: false,
    };
    return p1;
}

pub fn test_create_person() {
    let myperson = new_person();
    println!("First name: {0}, last name: {1}, birth month: {2}, birth year: {3}, visited europe {4}",
        myperson.first_name,
        myperson.last_name,
        myperson.birth_month,
        myperson.birth_year,
        myperson.visited_europe
    );
}

