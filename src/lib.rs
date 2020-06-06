use std::ops::{Index, IndexMut};
use simple::{Window, Rect};

type IndexType = (i32, i32);

pub trait RuleSet<D> {
    fn next(&self, source: &[&D]) -> D;
    fn source_size(&self) -> u8;
}

pub trait DataType: Clone {
    fn get_color(&self) -> (u8, u8, u8, u8);
    fn get_char(&self) -> char;
}

#[derive(Clone)]
struct Grid<D> {
    width: u32,
    height: u32,
    data: Vec<D>,
}

impl<D: DataType> Grid<D> {
    pub fn init(init_data: &[D], width: u32) -> Option<Grid<D>> {
        let size = init_data.len();
        let uw = width as usize;
        if size % uw != 0 {
            None
        } else {
            Some(Grid { width, height: (size / uw) as u32, data: init_data.to_vec() })
        }
    }

    fn wrap(&self, index: IndexType) -> (usize, usize) {
        (index.0.rem_euclid(self.width as i32) as usize, index.1.rem_euclid(self.height as i32) as usize)
    }

    fn get_area(&self, index: IndexType, size: u8) -> Vec<&D> {
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

    fn print(&self){
        for (i,v) in self.data.iter().enumerate() {
            print!("{}",v.get_char());
            if (i+1) % self.width as usize  == 0{
                print!("\n");
            }
        }

    }
}

impl<D: DataType> Index<IndexType> for Grid<D> {
    type Output = D;
    fn index(&self, index: IndexType) -> &Self::Output {
        let (x, y) = self.wrap(index);
        &self.data[y * (self.width as usize) + x]
    }
}

impl<D: DataType> IndexMut<IndexType> for Grid<D> {
    fn index_mut(&mut self, index: IndexType) -> &mut Self::Output {
        let (x, y) = self.wrap(index);
        &mut self.data[y * (self.width as usize) + x]
    }
}


pub struct Game<D, R> {
    grid: Grid<D>,
    rules: R,
}

struct Coord{
    width:u32,
    height:u32,
    x:u32,
    y:u32,
}

impl Iterator for Coord {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let o_x = self.x as i32;
        let o_y = self.y as i32;

        if self.y >= self.height {
            None
        } else {
            self.x += 1;
            if self.x >= self.width {
                self.x -= self.width;
                self.y += 1;
            }
            Some((o_x, o_y))
        }
    }
}

impl<D, R> Game<D, R>
    where D: DataType,
          R: RuleSet<D> {
    pub fn new(init_data: &[D], width: u32, rules: R) -> Option<Game<D, R>> {
        Grid::init(init_data, width).map_or(None, |grid| Some(Game { grid, rules }))
    }

    pub fn next(&mut self) {
        let grid_copy = self.grid.clone();
        for coord in self.get_coord() {
            let area = grid_copy.get_area(coord, self.rules.source_size());
            self.grid[coord] = self.rules.next(area.as_slice());
        }
    }

    pub fn draw(&self, screen: &mut Window, size: (u32, u32)) {
        for coord in self.get_coord() {
            let (r, g, b, a) = self.grid[coord].get_color();
            screen.set_color(r, g, b, a);
            screen.fill_rect(Rect::new(coord.0*size.0 as i32, coord.1*size.1 as i32, size.0, size.1));
        }
    }

    pub fn print(&self) {
        self.grid.print();
    }

    fn get_coord(&self) ->Coord{
        Coord{width:self.grid.width , height:self.grid.height, x:0,y:0}
    }
}
