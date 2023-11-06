

pub struct Room {
    name: String
}

impl Room {
    pub fn NewRoom(name:String) -> Room {
        return Room{name}
    }
    pub fn get_name(&self){
        println!("room name :{}",self.name)
    }
}