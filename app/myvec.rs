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
