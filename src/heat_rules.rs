use game_of_life::{DataType, ColoredDataType, RandomInit, RuleSet, Color};

#[derive(Clone)]
pub struct HeatData {
    value: f32,
    factor: f32,
}

impl DataType for HeatData {}

impl ColoredDataType for HeatData {
    fn get_color(&self) -> Color {
        if self.value<0.5 {
            let v = (self.value*2.0*255.0) as u8;
            (v,v,0,255)
        }else {
            let v = ((2.0 - self.value*2.0) * 255.0) as u8;
            (255, v, 0, 255)
        }

    }
}

impl RandomInit for HeatData {
    fn rnd() -> Self {
        let value = rand::random::<f32>();
        let factor = rand::random::<f32>() * 0.3 + 0.05;
        HeatData { value, factor }
    }
}

#[derive(Clone)]
pub struct HeatRules {}

impl RuleSet for HeatRules {
    type Data = HeatData;
    const SOURCE_SIZE: u8 = 3;

    fn next(&self, source: &[&Self::Data]) -> Self::Data {
        let me = source[4].value;
        let factor = source[4].factor;
        let avg = (source.iter().fold(0.0, |acc, d| acc + d.value) - me) / 8.0;
        let new_value = (avg - me) * factor + me;
        HeatData { value: new_value, factor }
        //HeatData{value: source[4].value}
    }

}

fn generate_heat_data(size: (u16, u16)) -> Vec<HeatData> {
    let l = size.0 as usize* size.1 as usize;
    let mut data = vec![HeatData{value:0.0,factor:0.5};l];
    data[l/2].value=1.0;

    data
}