use game_of_life::{DataType, ColoredDataType, RandomInit, RuleSet};
use rand::Rng;

#[derive(Clone)]
pub enum RGBData{
	Red,
	Green,
	Blue
}

impl DataType for RGBData {}

impl ColoredDataType for RGBData{
	fn get_color(&self) -> (u8, u8, u8, u8) {
		match self {
			RGBData::Red => (150,150,0,255),
			RGBData::Green => (75,150,0,255),
			RGBData::Blue => (25,50,0,255)
		}
	}
}

impl RandomInit for RGBData{
	fn rnd() -> Self {
		let mut rnd = rand::thread_rng();
		match rnd.gen_range(0..=2) {
			0 => RGBData::Red,
			1 => RGBData::Green,
			_ => RGBData::Blue
		}
	}
}

#[derive(Clone)]
pub struct RGBRules{}

impl RuleSet for RGBRules {
	type Data = RGBData;
	const SOURCE_SIZE: u8 = 3;

	fn next( source: &[&Self::Data]) -> Self::Data {
		const LIMIT:u8 = 3;
		let me = source[4];
		let neighbour = source.iter().fold((0,0,0), |acc, d|
			match d {
				RGBData::Red => (acc.0+1,acc.1,acc.2),
				RGBData::Green => (acc.0,acc.1+1,acc.2),
				RGBData::Blue => (acc.0,acc.1,acc.2+1)
			});
		match me{
			RGBData::Red => if neighbour.2 >= LIMIT{ RGBData::Blue} else { RGBData::Red }
			RGBData::Green => if neighbour.0 >= LIMIT{ RGBData::Red} else { RGBData::Green }
			RGBData::Blue => if neighbour.1 >= LIMIT { RGBData::Green} else { RGBData::Blue }

		}
	}
}

/*impl InitRuleSet for RGBRules{
	fn init() -> Self {
		RGBRules{}
	}
}*/