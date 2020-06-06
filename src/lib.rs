pub struct Board{
    width: u32,
    height: u32,
    data: Vec<bool>
}

impl Board {
    pub fn new(width: u32, height: u32) -> Board {
        Board { width, height, data: vec![false; (width * height) as usize] }
    }
}