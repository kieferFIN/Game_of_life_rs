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
}


pub struct Game<D> {
    grid: Grid<D>
}

impl<D: Clone> Game<D> {
    pub fn new(init_data: &[D], width: u32) -> Option<Game<D>> {
        Grid::init(init_data, width).map_or(None, |grid| Some(Game { grid }))
    }

}
