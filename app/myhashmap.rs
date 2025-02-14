use std::collections::HashMap;

pub fn test_hashmap_basic() {
    // let stock_list: HashMap<String, f32> = HashMap::new();

    let mut stock_list = HashMap::<String, f32>::new();
    println!("{}", stock_list.len());
    println!("{}", stock_list.is_empty());

    stock_list.insert("NVDA".to_string(), 478.52);
    stock_list.insert("AAPL".to_string(), 232.92);
    stock_list.insert("AMSC".to_string(), 50.78);


    stock_list.insert("AAPL".to_string(), 233.47);


    stock_list.entry("META".to_string()).or_insert(346.34);
    stock_list.entry("META".to_string()).or_insert(358.34);

    print!("{:#?}", stock_list);
    
    stock_list.remove(&("AAPL".to_string()));
    
    println!("{:#?}", stock_list);

    for (ticker, current_value) in stock_list {
        println!("{} is trading at {}", ticker, current_value);
    }
}

