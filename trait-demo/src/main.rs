trait House {
    fn get_name(&self) -> String;
    fn get_height(&self) -> i32;
}

struct HouseStruct {
    name: String,
    height: i32,
}

impl House for HouseStruct {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_height(&self) -> i32 {
        self.height
    }
}

impl HouseStruct {
    fn new(name: String, height: i32) -> HouseStruct {
        HouseStruct {
            name,
            height,
        }
    }
}

fn main() {
    let h = HouseStruct::new(String::from("test"), 32);
    println!("{}", h.get_name());
}
