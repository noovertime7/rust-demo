use crate::types::{GetName, GetAge};

pub struct student {
    age: i32,
    name: String,
}

impl student {
    pub fn new(age: i32, name: String) -> student {
        student {
            age,
            name,
        }
    }
}

impl GetName for student {
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl GetAge for student {
    fn get_age(&self) -> i32 {
        self.age
    }
}