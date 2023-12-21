

pub struct RingMem{
    pub buffer: Vec<i32>,
    size: usize,
    current_idx: usize
}

impl RingMem{
    pub fn new(size: usize)->Self{
        let buffer: Vec<i32> = vec![0;size];
        let current_idx = 0;
        Self {buffer, size,current_idx}
    }
    pub fn add(&mut self,val:i32){
        self.buffer[self.current_idx] = val;
        self.current_idx = (self.current_idx+ 1) % self.size;
    }
}