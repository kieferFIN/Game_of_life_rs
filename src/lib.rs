use std::ops::{Index, IndexMut};
use std::sync::Arc;
use std::thread;
pub use crate::error_handling::GError;

mod error_handling;

#[cfg(feature = "graphics-ggez")]
mod ggez_graphics;

#[cfg(feature = "graphics-piston")]
mod piston_graphics;

#[cfg(feature = "graphics-pixels")]
mod pixels_graphics;

type IndexType = (i32, i32);

pub type Color = (u8,u8, u8, u8);


pub trait RuleSet {
    type Data:DataType;
    fn next(source: &[&Self::Data]) -> Self::Data;
    fn source_size() -> u8;
}

pub trait DataType: Clone + Send + Sync +'static {}

pub trait ColoredDataType: DataType{
    fn get_color(&self) -> Color;
}

pub trait PrintableDataType: DataType{
    fn get_char(&self) -> char;
}

pub trait RandomInit {
    fn rnd() -> Self;
}

#[derive(Clone)]
struct Grid<D> {
    width: u16,
    height: u16,
    data: Vec<D>,
}

impl<D: DataType> Grid<D> {
    pub fn init_with_data(init_data: &[D], width: u16) -> Result<Grid<D>,GError> {
        let size = init_data.len();
        let uw = width as usize;
        if size % uw != 0 {
            Err(GError::InitializationError {size,width})
        } else {
            Ok(Grid { width, height: (size / uw) as u16, data: init_data.to_vec() })
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
                let (x,y) = self.wrap((w,h));
                v.push(&self.data[y * (self.width as usize) + x]);
            }
        }
        v
    }

    fn get_raw_data(&self) -> &Vec<D> {
        &self.data
    }
    fn get_raw_mut_data(&mut self) -> &mut Vec<D> {
        &mut self.data
    }
}

impl<D:PrintableDataType> Grid<D> {
    fn print(&self) {
        for (i, v) in self.data.iter().enumerate() {
            print!("{}", v.get_char());
            if (i + 1) % self.width as usize == 0 {
                print!("\n");
            }
        }
    }
}

impl<D: DataType> Index<IndexType> for Grid<D> {
    type Output = D;
    fn index(&self, index: IndexType) -> &Self::Output {
        //let (x, y) = self.wrap(index);
        let x = index.0 as usize;
        let y = index.1 as usize;
        &self.data[y * (self.width as usize) + x]
    }
}

impl<D: DataType> IndexMut<IndexType> for Grid<D> {
    fn index_mut(&mut self, index: IndexType) -> &mut Self::Output {
        //let (x, y) = self.wrap(index);
        let x = index.0 as usize;
        let y = index.1 as usize;
        &mut self.data[y * (self.width as usize) + x]
    }
}


pub struct Game<R>
    where R: RuleSet {
    grid: Grid<R::Data>,
}

struct CoordIter {
    width: u16,
    height: u16,
    x: u16,
    y: u16,
}

impl Iterator for CoordIter {
    type Item = IndexType;

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

impl <R> Game<R>
    where R:RuleSet,
          R::Data: RandomInit {
    pub fn init_random(game_size:(u16,u16)) -> Game<R>{
        let total_size = game_size.0 as usize * game_size.1 as usize;
        let mut data = Vec::with_capacity(total_size);
        for _ in 0..total_size {
            data.push(R::Data::rnd())
        };
        Game::init_with_data(&data, game_size.0).expect("Internal Error")
    }
}

impl <R> Game<R>
    where R: RuleSet{
    pub fn init_with_data(init_data: &[R::Data], width: u16) -> Result<Game<R>, GError> {
        Grid::init_with_data(init_data, width).map( |grid| Game { grid})
    }

    fn get_coord_iter(&self) -> CoordIter {
        CoordIter { width: self.grid.width, height: self.grid.height, x: 0, y: 0 }
    }

    pub fn next_step(&mut self) {
        const NUMBER_OF_THREADS:u16 = 4;
        let grid_copy = Arc::new(self.grid.clone());
        let mut handles = vec![];
        let height = self.grid.height;
        let width = self.grid.width;
        let source_size = R::source_size();
        for index in 0..NUMBER_OF_THREADS{
            let y_start = index * height/NUMBER_OF_THREADS;
            let y_end = (index+1) * height/NUMBER_OF_THREADS;
            let grid_copy = Arc::clone(&grid_copy);
            let handle = thread::spawn( move ||{
                let iter = CoordIter{width, height:y_end, x:0, y:y_start};
                let mut v = Vec::with_capacity(((y_end-y_start)*width) as usize);
                for c in iter{
                    let area = grid_copy.get_area(c,source_size);
                    v.push(R::next(area.as_slice()));
                }
                (y_start,y_end,v)
            });
            handles.push(handle);
        }
        let data = self.grid.get_raw_mut_data();
        for h in handles{
            let (start,end,v) = h.join().unwrap();
            let start = start as usize * width as usize;
            let end = end as usize * width as usize;
            data[start..end].clone_from_slice(&v)

        }
    }
}

impl<R> Game<R>
    where R: RuleSet,
          R::Data: PrintableDataType {

    pub fn print(&self) {
        self.grid.print();
    }
}

impl<'a,R> IntoIterator for &'a Game<R>
    where R: RuleSet{
    type Item = (IndexType,&'a R::Data);
    type IntoIter = GameIter<'a,R::Data>;

    fn into_iter(self) -> Self::IntoIter {
        GameIter{coord: self.get_coord_iter(), data: &self.grid.get_raw_data(), i:0 }
    }
}

impl<R> Game<R>
    where R: RuleSet,
    R::Data: ColoredDataType{

    #[cfg(feature = "graphics-ggez")]
    pub fn run_with_ggez(&mut self, window_size:(u32,u32))-> Result<(),GError>{
        ggez_graphics::run(window_size,self)
    }

    #[cfg(feature = "graphics-piston")]
    pub fn run_with_piston(&mut self, window_size:(u32,u32))-> Result<(),GError>{
        piston_graphics::run(window_size,self).map_err(|e|GError::PistonError {source:e})
    }

    #[cfg(feature = "graphics-pixels")]
    pub fn run_with_pixels(&mut self, window_size:(u32,u32))->Result<(), GError>{
        pixels_graphics::run(window_size,self)
    }

    pub fn to_raw_colors(&self) -> Vec<u8>{
        let capacity = self.grid.width as usize * self.grid.height as usize * 4;
        let mut v = Vec::with_capacity(capacity);
        for (_,d) in self.into_iter(){
            let (r, g, b, a) = d.get_color();
            v.push(r);
            v.push(g);
            v.push(b);
            v.push(a);

        };
        v
    }
}

pub struct GameIter<'a,D>{
    coord: CoordIter,
    data: &'a Vec<D>,
    i: usize
}

impl<'a,D> Iterator for GameIter<'a,D>
    where D:DataType{

    type Item = (IndexType,&'a D);

    fn next(&mut self) -> Option<Self::Item> {
        self.coord.next().map(|c|{
            let i = self.i;
            self.i +=1 ;
            (c,&self.data[i])
        })
    }
}
