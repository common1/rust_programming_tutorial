pub fn test_vec_int() {
    let mut my_ints: Vec<u32> = Vec::new();

    my_ints.push(30);
    my_ints.push(40);
    my_ints.push(50);
    my_ints.push(60);
    my_ints.push(70);
    my_ints.push(80);
    my_ints.len();

    println!("Size of Vec: {:?}", my_ints.len());
    println!("Capacity of Vec: {:?}", my_ints.capacity());
    println!("{:?}", my_ints);

    // println!("First item in Vec is: {:?}", &(&my_ints).as_slice()[10]);

    println!("First element is: {:?}", my_ints.get(1));
}

pub fn test_vec_string() {
    let first_names = vec!["Trevor", "Nancy", "Shannon", "Billy", "Rachel"];

    for first_name in first_names.as_slice() {
        println!("Processing {} ...", first_name);
    }

    println!("{:?}", first_names);
}

#[derive(Debug)]
struct Car {
    manufacturer: String,
    model: String
}

pub fn test_vec_car() {
    // let car_list = vec!["Trevor"; 10];

    let mut car_list: Vec<Car> = vec![];

    let mut car_list2: Vec<Car> = vec![];

    for _ in 1..=100u8 {
        car_list.push(Car{manufacturer: "Porsche".to_string(), model: "Panamera".to_string()});
    }

    for _ in 1..=100u8 {
        car_list2.push(Car{manufacturer: "Hyundai".to_string(), model: "Sonata".to_string()});
    }

    car_list.append(&mut car_list2);
    car_list.insert(0, Car{manufacturer: "Lamborghini".to_string(), model: "Avantador".to_string()});
    car_list.remove(0);

    let keep = |e: &Car| {if e.manufacturer == "Porsche" {return true;} else {return false}};
    car_list.retain(keep);

    println!("{:?}", car_list);
    println!("{:?}", car_list.len());
    println!("{:?}", car_list2.len());
    println!("{:?}", car_list.capacity());
    println!("{:?}", car_list2.capacity());

    println!("{:?}", car_list.get(0).unwrap());


}

