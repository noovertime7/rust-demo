use crate::house::room;
use crate::house::room::Room;


pub struct House {
    r: room::Room
}

impl House {
    pub fn NewHouse(name: String) -> House {
        let r  = Room::NewRoom(name);
        return House{
            r
        }
    }
    pub fn getRoomName(&self)  {
         self.r.get_name()
    }
}