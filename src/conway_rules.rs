use game_of_life::{DataType, ColoredDataType, RandomInit, RuleSet, PrintableDataType, Color};
use std::collections::VecDeque;

pub struct ClassicConway {}

impl RuleSet for ClassicConway {
    type Data = BoolData;
    const SOURCE_SIZE: u8 = 3;

    fn next(source: &[&BoolData]) -> BoolData {
        let me = source[4].value;
        let all: i32 = source.iter().map(|x| x.value as i32).sum();
        let neighbours = all - me as i32;

        match (me, neighbours) {
            (true, 2) | (_, 3) => BoolData { value: true },
            _ => BoolData { value: false }
        }
    }
}

#[derive(Clone)]
pub struct BoolData {
    value: bool
}

impl DataType for BoolData {}

impl RandomInit for BoolData {
    fn rnd() -> BoolData {
        BoolData { value: rand::random::<bool>() }
    }
}

impl ColoredDataType for BoolData {
    fn get_color(&self) -> Color {
        if self.value {
            (255, 255, 255, 255)
        } else {
            (0, 0, 0, 255)
        }
    }
}

impl PrintableDataType for BoolData {
    fn get_char(&self) -> char {
        if self.value {
            '*'
        } else {
            '_'
        }
    }
}

//************************


#[derive(Clone)]
pub struct BoolHist {
    current: bool,
    history: VecDeque<bool>,
}

impl RandomInit for BoolHist {
    fn rnd() -> BoolHist {
        BoolHist {
            history: VecDeque::from(vec![false; 5]),
            current: rand::random::<bool>(),
        }
    }
}

impl DataType for BoolHist {}

impl ColoredDataType for BoolHist {
    fn get_color(&self) -> Color{
        if self.current {
            (255, 255, 255, 255)
        } else {
            let s: i32 = self.history.iter().map(|x| *x as i32).sum();
            let gray = (s * 40) as u8;
            (gray, gray, gray, 255)
        }
    }
}

pub struct ConwayWithHistory {}

impl RuleSet for ConwayWithHistory {
    type Data = BoolHist;
    const SOURCE_SIZE: u8 = 3;

    fn next(source: &[&BoolHist]) -> BoolHist {
        let me = source[4];
        let all: i32 = source.iter().map(|x| x.current as i32).sum();
        let neighbours = all - me.current as i32;

        let current = match (me.current, neighbours) {
            (true, 2) | (_, 3) => true,
            _ => false
        };
        let mut history = me.history.clone();
        history.push_back(me.current);
        history.pop_front();
        BoolHist { current, history }
    }
}

//**************************************************************
#[derive(Clone)]
pub struct ColorData {
    r: bool,
    g: bool,
    b: bool,
}

impl RandomInit for ColorData {
    fn rnd() -> Self {
        ColorData { r: rand::random::<bool>(), g: rand::random::<bool>(), b: rand::random::<bool>() }
    }
}

impl DataType for ColorData {}

impl ColoredDataType for ColorData {
    fn get_color(&self) -> Color {
        (self.r as u8 * 255, self.g as u8 * 255, self.b as u8 * 255, 255)
    }
}

pub struct ConwayColors {}

impl RuleSet for ConwayColors {
    type Data = ColorData;
    const SOURCE_SIZE: u8 = 3;

    fn next(source: &[&ColorData]) -> ColorData {
        let me = source[4];
        let all = source.iter().fold((0, 0, 0), |acc, d| (acc.0 + d.r as i8, acc.1 + d.g as i8, acc.2 + d.b as i8));
        let neighbours = (all.0 - me.r as i8, all.1 - me.g as i8, all.2 - me.b as i8);

        let r = match (me.r, neighbours.0) {
            (true, 2) | (_, 3) => true,
            _ => false
        };
        let g = match (me.g, neighbours.1) {
            (true, 2) | (_, 3) => true,
            _ => false
        };
        let b = match (me.b, neighbours.2) {
            (true, 2) | (_, 3) => true,
            _ => false
        };

        ColorData { r, g, b }
    }

}