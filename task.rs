pub struct Task{
    pub title: String,
    pub complete: bool,
}

impl Task{
    pub fn new(title: &str,complete: bool) -> Self{
        Self{
            title,
            complete
        }
    }
    pub fn insert(self)->bool{
        todo!()
    }
    pub fn update(&mut self)->bool{
        todo!()
    }
    pub fn delete(&mut self)->{
        todo!()
    }
    
}
