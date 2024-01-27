use std::{ops::Index, sync::Arc, thread};

use crate::{
    backends::{Backend, BackendStatic},
    grid::Grid,
    ColoredDataType, DataType, GResult, IndexType, PrintableDataType, RandomInit, RuleSet, Size,
};

pub struct Game<R>
where
    R: RuleSet,
{
    grid: Grid<R::Data>,
}

impl<R> Game<R>
where
    R: RuleSet,
{
    pub fn init_with_data(init_data: Vec<R::Data>, width: u16) -> GResult<Game<R>> {
        Grid::init_with_data(init_data, width).map(|grid| Game { grid })
    }

    fn get_coord_iter(&self) -> CoordIter {
        let size = self.grid.get_size();
        CoordIter {
            width: size.width,
            height: size.height,
            x: 0,
            y: 0,
        }
    }
    pub fn next_step(&mut self) {
        const NUMBER_OF_THREADS: u16 = 4;
        let grid_copy = Arc::new(self.grid.clone());
        //let rules_copy = Arc::new(self.rules.clone());
        let mut handles = vec![];
        let size = self.grid.get_size();
        let height = size.height;
        let width = size.width;
        let source_size = R::SOURCE_SIZE;
        for index in 0..NUMBER_OF_THREADS {
            let y_start = index * height / NUMBER_OF_THREADS;
            let y_end = (index + 1) * height / NUMBER_OF_THREADS;
            let grid_copy = Arc::clone(&grid_copy);
            //let rules_copy = Arc::clone(&rules_copy);
            let handle = thread::spawn(move || {
                let iter = CoordIter {
                    width,
                    height: y_end,
                    x: 0,
                    y: y_start,
                };
                let mut v = Vec::with_capacity(((y_end - y_start) * width) as usize);
                for c in iter {
                    let area = grid_copy.get_area(c, source_size);
                    v.push(R::next(area.as_slice()));
                }
                (y_start, y_end, v)
            });
            handles.push(handle);
        }
        let data = self.grid.get_raw_mut_data();
        for h in handles {
            let (start, end, v) = h.join().unwrap();
            let start = start as usize * width as usize;
            let end = end as usize * width as usize;
            data[start..end].clone_from_slice(&v)
        }
    }

    pub fn get_size(&self) -> Size {
        self.grid.get_size()
    }
}

/*impl<R> Game<R>
    where R: InitRuleSet {
    pub fn init_with_data(init_data: Vec<R::Data>, width: u16) -> GResult<Game<R>> {
        Grid::init_with_data(init_data, width).map(|grid| Game { grid, rules: R::init() })
    }
}*/

impl<R> Game<R>
where
    R: RuleSet,
    R::Data: RandomInit,
{
    pub fn init_random_data(game_size: (u16, u16)) -> GResult<Game<R>> {
        let total_size = game_size.0 as usize * game_size.1 as usize;
        let mut data = Vec::with_capacity(total_size);
        for _ in 0..total_size {
            data.push(R::Data::rnd())
        }
        Game::init_with_data(data, game_size.0)
    }
}

impl<R> Game<R>
where
    R: RuleSet,
    R::Data: PrintableDataType,
{
    pub fn print(&self) {
        self.grid.print();
    }
}

impl<R> Game<R>
where
    R: RuleSet,
    R::Data: ColoredDataType,
{
    pub fn to_raw_colors(&self) -> (Vec<u8>, Size) {
        let size = self.grid.get_size();
        let capacity = size.width as usize * size.height as usize * 4;
        let mut v = Vec::with_capacity(capacity);
        for (_, d) in self.into_iter() {
            let (r, g, b, a) = d.get_color();
            v.push(r);
            v.push(g);
            v.push(b);
            v.push(a);
        }
        (v, size)
    }
}

impl<R> Game<R>
where
    R: RuleSet,
    R::Data: DataType,
{
    pub fn run<B: Backend<R>>(&mut self, window_size: (u32, u32)) -> GResult<()> {
        B::run(window_size, self).map_err(|e| e.into())
    }
    pub fn run_owned<B: BackendStatic<R>>(self, window_size: (u32, u32)) -> GResult<()> {
        B::run(window_size, self).map_err(|e| e.into())
    }
}

pub struct GameIter<'a, D> {
    coord: CoordIter,
    data: &'a [D],
    i: usize,
}

impl<'a, R> IntoIterator for &'a Game<R>
where
    R: RuleSet,
{
    type Item = (IndexType, &'a R::Data);
    type IntoIter = GameIter<'a, R::Data>;

    fn into_iter(self) -> Self::IntoIter {
        GameIter {
            coord: self.get_coord_iter(),
            data: &self.grid.get_raw_data(),
            i: 0,
        }
    }
}

impl<'a, D> Iterator for GameIter<'a, D>
where
    D: DataType,
{
    type Item = (IndexType, &'a D);

    fn next(&mut self) -> Option<Self::Item> {
        self.coord.next().map(|c| {
            let i = self.i;
            self.i += 1;
            (c, &self.data[i])
        })
    }
}

impl<R: RuleSet> Index<IndexType> for Game<R> {
    type Output = R::Data;

    fn index(&self, index: IndexType) -> &Self::Output {
        &self.grid[index]
    }
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
