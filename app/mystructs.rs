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

impl Vehicle {
    fn paint(&mut self, new_color: VehicleColor) {
        self.color = new_color;
    }

    fn create_vehicle() -> Vehicle{
        let new_vehicle = Vehicle {
            manufacturer: "default".to_string(),
            model: "default".to_string(),
            year: 1990,
            color: VehicleColor::Blue
        };
        return new_vehicle;
    }
}

fn new_vehicle() -> Vehicle {
    let mut v1 = Vehicle{
        manufacturer: "Porsche".to_string(),
        model: "911".to_string(),
        year: 1991,
        color: VehicleColor::Green,
    };
    v1.paint(VehicleColor::White);
    return v1
}

pub fn create_vehicle() {
    // let myvehicle = new_vehicle();
    let mut myvehicle = Vehicle::create_vehicle();
    myvehicle.paint(VehicleColor::White);
    println!("{:#?}", myvehicle);
}

struct Person {
    first_name: String,
    last_name: String,
    birth_month: u8,
    birth_year: u16,
    visited_europe: bool,
    meters_walked: u32
}

impl Person {
    fn walk_meters(&mut self, meters: u32) {
        self.meters_walked += meters;
    }
}


fn new_person() -> Person {
    let p1 = Person{
        first_name: "Trevor".to_string(),
        last_name: "Sullivan".to_string(),
        birth_month: 6,
        birth_year: 1986,
        visited_europe: false,
        meters_walked: 0
    };
    // p1.first_name.set("Shannon");
    // p1.last_name = "Jones".to_string();
    return p1;
}

pub fn test_create_person() {
    let mut myperson : Person = new_person();
    myperson.walk_meters(8);
    myperson.walk_meters(20);
    println!("First name: {0}, last name: {1}, meters walked: {2}",
        myperson.first_name,
        myperson.last_name,
        myperson.meters_walked
    );
}

