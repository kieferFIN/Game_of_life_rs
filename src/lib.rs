use std::ops::{Index, IndexMut};

#[derive(Clone)]
struct Grid<D> {
    width: usize,
    height: usize,
    data: Vec<D>,
}

impl<D: Clone> Grid<D> {
    pub fn init(init_data: &[D], width: usize) -> Option<Grid<D>> {
        let size = init_data.len() ;
        if size % width != 0 {
            None
        } else {
            Some(Grid { width, height: size / width, data: init_data.to_vec() })
        }
    }
}

impl<D> Index<(usize, usize)> for Grid<D> {
    type Output = D;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let x = index.0 % self.width;
        let y = index.1 % self.height;

        &self.data[y * (self.width as usize) * x]
    }
}

impl<D> IndexMut<(usize, usize)> for Grid<D>{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let x = index.0 % self.width;
        let y = index.1 % self.height;

        &mut self.data[y * (self.width as usize) * x]
    }
}


pub struct Game<D> {
    grid: Grid<D>
}

impl<D: Clone> Game<D> {
    pub fn new(init_data: &[D], width: usize) -> Option<Game<D>> {
        Grid::init(init_data, width).map_or(None, |grid| Some(Game { grid }))
    }
}
