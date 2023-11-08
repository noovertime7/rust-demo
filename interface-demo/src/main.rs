trait House {
    fn get_name(&self) -> String;
    fn get_height(&self) -> i32;
    fn get_width(&self) -> i32;
}

struct house {
    name: String,
    height: i32,
    width: i32,
}

impl House for house {
    fn get_name(&self) -> String {
        let other = || { format!("{} by other", &self.name) };
        other()
    }
    fn get_height(&self) -> i32 {
        self.height
    }
    fn get_width(&self) -> i32 {
        self.width
    }
}

impl house {
    fn new(name: &str, height: i32, width: i32) -> house {
        return house {
            name: name.to_string(),
            height,
            width,
        };
    }
}

fn NewHouse(name: &str, height: i32, width: i32) -> Box<dyn House> {
    return Box::new(house::new(name, height, width));
}

fn main() {
    let h = NewHouse("test1", 10, 20);
    println!("house name = {}", h.get_name());
    println!("house height = {}", h.get_height());
    println!("house width = {}", h.get_width());
}