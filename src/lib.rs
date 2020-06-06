use std::ops::{Index, IndexMut};

type IndexType = (i32, i32);

#[derive(Clone)]
struct Grid<D> {
    width: u32,
    height: u32,
    data: Vec<D>,

}

impl<D: Clone> Grid<D> {
    pub fn init(init_data: &[D], width: u32) -> Option<Grid<D>> {
        let size = init_data.len() as u32;
        if size % width != 0 {
            None
        } else {
            Some(Grid { width, height: size / width, data: init_data.to_vec() })
        }
    }

    fn wrap(&self, index: IndexType) -> (usize, usize) {
        (index.0.rem_euclid( self.width as i32) as usize, index.1.rem_euclid( self.width as i32) as usize)
    }

    fn get_area(&self, index: IndexType, size: u32) -> Vec<&D> {
        let mut v = Vec::new();
        let x = index.0 as i32;
        let y = index.1 as i32;
        let half_size = (size / 2) as i32;
        for h in y - half_size..=y + half_size {
            for w in x - half_size..=x + half_size {
                v.push(self.index((w, h)));
            }
        }
        v
    }
}

impl<D: Clone> Index<IndexType> for Grid<D> {
    type Output = D;
    fn index(&self, index: IndexType) -> &Self::Output {
        let (x, y) = self.wrap(index);
        &self.data[y * (self.width as usize) + x]
    }
}

impl<D: Clone> IndexMut<IndexType> for Grid<D> {
    fn index_mut(&mut self, index: IndexType) -> &mut Self::Output {
        let (x, y) = self.wrap(index);
        &mut self.data[y * (self.width as usize) + x]
    }
}

pub trait RuleSet<D>{
    fn update(&self, source:&[&D]) -> D;
    fn source_size(&self) -> u32;
}


pub struct Game<D, R>
    where R: RuleSet<D>{
    grid: Grid<D>,
    rules: R,
}

impl<D: Clone, R> Game<D, R>
    where R: RuleSet<D> {
    pub fn new(init_data: &[D], width: u32, rules: R) -> Option<Game<D,R>> {
        Grid::init(init_data, width).map_or(None, |grid| Some(Game { grid, rules }))
    }

    pub fn next(&mut self) {
        let grid_copy = self.grid.clone();
        for y in 0..grid_copy.height as i32{
            for x in 0..grid_copy.width as i32{
                let area = grid_copy.get_area((x, y), self.rules.source_size());
                self.grid[(x, y)] = self.rules.update(area.as_slice());
            }
        }
    }
}
