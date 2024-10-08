pub struct Numbers{
    pub x: u32,
    pub y: u32,
}

impl Numbers{
    fn new(x: u32, y: u32) -> Self{
        Self{ x: x, y: y }
    }

    fn add(&self) -> u32{
        self.x + self.y
    }

    fn del(&self) -> u32{
        self.x - self.y
    }

    fn multip(&self) -> u32{
        self.x * self.y
    }
}

fn main() {
    let numbers_obj= Numbers::new(9, 2);

    let add= Numbers::add(&numbers_obj);
    let del= Numbers::del(&numbers_obj);
    let multip= Numbers::multip(&numbers_obj);

    println!("Add: {}, del: {}, multip: {}", add, del, multip);
}
