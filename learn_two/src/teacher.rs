use crate::types::{GetName, GetAge};
pub struct teacher {
    age: i32,
    name: String,
}

impl teacher {
    pub fn new(age: i32, name: String) -> teacher {
        teacher {
            age,
            name,
        }
    }
}

impl GetName for teacher {
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl GetAge for teacher {
    fn get_age(&self) -> i32 {
        self.age
    }
}


