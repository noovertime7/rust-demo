// pub mod room;
pub struct House {
    width:i32,
    height:i32
}

impl House {
    pub fn NewTest2(width:i32, height:i32) -> House {
        return House{
            width,
            height
        }
    }

    pub fn Add(&self) -> i32 {
        return self.height + self.width
    }
}