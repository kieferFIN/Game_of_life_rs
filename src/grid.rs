use std::ops::{Index, IndexMut};

use crate::{DataType, GError, GResult, IndexType, PrintableDataType, Size};

#[derive(Clone)]
pub struct Grid<D> {
    width: u16,
    height: u16,
    data: Box<[D]>,
}

impl<D: DataType> Grid<D> {
    pub fn init_with_data(init_data: Vec<D>, width: u16) -> GResult<Grid<D>> {
        let size = init_data.len();
        let uw = width as usize;
        if size % uw != 0 {
            Err(GError::InitializationError { size, width })
        } else {
            Ok(Grid {
                width,
                height: (size / uw) as u16,
                data: init_data.into_boxed_slice(),
            })
        }
    }

    pub fn get_size(&self) -> Size {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn wrap(&self, index: IndexType) -> (usize, usize) {
        (
            index.0.rem_euclid(self.width as i32) as usize,
            index.1.rem_euclid(self.height as i32) as usize,
        )
    }

    pub fn get_area(&self, index: IndexType, size: u8) -> Vec<&D> {
        let mut v = Vec::new();
        let x = index.0 as i32;
        let y = index.1 as i32;
        let half_size = (size / 2) as i32;
        for h in y - half_size..=y + half_size {
            for w in x - half_size..=x + half_size {
                let (x, y) = self.wrap((w, h));
                v.push(&self.data[y * (self.width as usize) + x]);
            }
        }
        v
    }

    pub fn get_raw_data(&self) -> &[D] {
        &self.data
    }
    pub fn get_raw_mut_data(&mut self) -> &mut [D] {
        &mut self.data
    }
}

impl<D: PrintableDataType> Grid<D> {
    pub fn print(&self) {
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
