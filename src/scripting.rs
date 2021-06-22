use crate::{DataType, GError, RuleSet};
use std::path::PathBuf;
use rhai::{Engine, Func, AST};

#[derive(Clone)]
pub struct ScriptedData{
	data: rhai::Map
}

impl DataType for ScriptedData {}


#[derive(Clone)]
pub struct ScriptRules<>{
	engine: Engine,
	ast: AST,
	update_fn_name: String,
	color_fn_name: String

}

impl ScriptRules{
	pub fn create(script_file: PathBuf, update_fn_name: & str, color_fn_name: & str)->Result<Self,GError>{
		let mut engine = Engine::new();
		let ast = engine.compile_file(script_file)?;

		//let update_func = Func::<(int,) , int>::create_from_ast(engine,ast,update_fn_name);
		Ok(ScriptRules{engine, ast, update_fn_name: update_fn_name.to_string() , color_fn_name: color_fn_name.to_string()})
	}
}

impl RuleSet for ScriptRules{
	type Data = ScriptedData;
	const SOURCE_SIZE: u8 = 3;

	fn next(&self, source: &[&Self::Data]) -> Self::Data {
		todo!()
	}
}