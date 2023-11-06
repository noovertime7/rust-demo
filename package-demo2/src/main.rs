use std::collections::HashMap;
use package_demo2::house::house::House;

fn main() {
    let mut  hourses = Vec::new();
    hourses.push(String::from("1"));
    hourses.push(String::from("2"));
    hourses.push(String::from("3"));

   let res =  hourses.pop();
    match res {
        Some(val) => println!("pop val = {}",val),
        None => println!("None"),
    }
    let mut scores = HashMap::new();

    for hourse in &hourses {
        let h  = House::NewHouse(String::from(hourse));
        h.getRoomName();
        scores.insert(hourse,10);
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
