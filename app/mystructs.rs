use std::cell::Cell;

#[derive(Debug)]
#[allow(dead_code)]
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

fn new_vehicle_tuple() -> VehicleTuple {
    return VehicleTuple("Hyundai".to_string(), "Elantra".to_string(), 2015);
}

pub fn create_vehicle_tuple() {
    let myvehicletuple = new_vehicle_tuple();
    println!("Manufacturer: {0}, model: {1}", myvehicletuple.0, myvehicletuple.1);
}

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

struct Person<'p> {
    first_name: Cell<&'p str>,
    last_name: String,
    birth_month: u8,
    birth_year: u16,
    visited_europe: bool,
}

fn new_person() -> Person<'static> {
    let p1 = Person{
        first_name: Cell::from("Trevor"),
        last_name: "Sullivan".to_string(),
        birth_month: 6,
        birth_year: 1986,
        visited_europe: false,
    };
    p1.first_name.set("Shannon");
    // p1.last_name = "Jones".to_string();
    return p1;
}

pub fn test_create_person() {
    let myperson : Person<'_>= new_person();
    println!("First name: {0}, last name: {1}, birth month: {2}, birth year: {3}, visited europe {4}",
        myperson.first_name.get(),
        myperson.last_name,
        myperson.birth_month,
        myperson.birth_year,
        myperson.visited_europe
    );
}

