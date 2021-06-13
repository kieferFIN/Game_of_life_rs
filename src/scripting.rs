use crate::DataType;

#[derive(Clone)]
pub struct ScriptedData{
	data: rhai::Map
}

//impl DataType for ScriptedData {}