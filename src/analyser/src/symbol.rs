#[derive(Clone, Debug, PartialEq)]
pub struct Symbol<T> {
	pub id: i32, // Unique identifier
	pub is_global: bool,
	pub identifier: String, // Symbol Table Key
	pub symbol_type: T, // VarType + FuncType
}

pub type VarSymbol = Symbol<VarType>;
pub type FuncSymbol = Symbol<FuncType>;

/*
	There are in total two kinds of Symbol Type:
	- Variable Type `VarType`
		- Primitive Type `(BasicType, Value)`
			- Int
			- Char
			- Float
			- Bool
			- String
		- Array Type `(Vec<usize>, Vec<Value>)`
		- Struct Type `(Vec<VarType>)`
	- Function Type `FuncType`
		- `(FuncReturnType, Vec<VarType>)`
	Different　Symbols are stored in different Symbol Tables.
*/
#[derive(Clone, Debug)]
pub enum VarType {
	Primitive(PrimType),
	Array(ArrayType),
	Struct(StructType)
}

pub type PrimType = (BasicType, Val);
pub type ArrayType = (BasicType, Vec<Val>, Vec<usize>);
pub type StructType = (Vec<String>, Vec<VarType>);
pub type FuncType = (FuncReturnType, Vec<VarType>);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BasicType {
	Int,
	Char,
	Float,
	Bool,
	String,
	Null
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FuncReturnType {
	Int,
	Char,
	Float,
	Bool,
	String,
	#[default]
	Void
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Val {
	Int(i32),
	Char(char),
	Float(f32),
	Bool(bool),
	String(String),
	Array(ArrayType)
}