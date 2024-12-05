use crate::symbol::*;
use crate::manager::SymbolManager;
use crate::error::{SemanticError, SemanticErrorManager};

impl<T> Symbol<T> {
    pub fn new(id: i32, is_global: bool, identifier: String, symbol_type: T) -> Symbol<T>{
        Symbol {
            id,
            is_global,
            identifier,
            symbol_type,
        }
    }
}


// From for VarSymbol
impl VarSymbol {
    pub fn primitive(manager: &mut SymbolManager, identifier: String, type_t: BasicType, is_global: bool) -> VarSymbol {
        manager.new_var_symbol(identifier, VarType::Primitive(type_t), is_global)
    }
    
    pub fn array(manager: &mut SymbolManager, identifier: String, type_t: BasicType, dimensions: Vec<usize>, is_global: bool) -> VarSymbol {
        manager.new_var_symbol(identifier, VarType::Array((type_t, dimensions)), is_global)
    }

    pub fn struct_type(manager: &mut SymbolManager, identifier: String, fields: (Vec<String>, Vec<VarType>), is_global: bool) -> VarSymbol {
        manager.new_var_symbol(identifier, VarType::Struct(fields), is_global)
    }

    pub fn get_primitive(&self) -> Option<BasicType> {
        match &self.symbol_type {
            VarType::Primitive(t) => Some(*t),
            _ => None,
        }
    }

    pub fn get_array_dimensions(&self) -> Option<Vec<usize>> {
        match &self.symbol_type {
            VarType::Array((_, d)) => Some(d.clone()),
            _ => None,
        }
    }

    pub fn get_struct_field(&self, field: String) -> Option<BasicType> {
        match &self.symbol_type {
            VarType::Struct((fields, types)) => {
                for i in 0..fields.len() {
                    if fields[i] == field {
                        return match &types[i] {
                            VarType::Primitive(t) => Some(*t),
                            _ => None,
                        }
                    }
                }
                None
            },
            _ => None,
        }
    } 
}

// From for FuncSymbol
impl FuncSymbol {
    fn define(manager: &mut SymbolManager, identifier: String, return_type: FuncReturnType, parameters: Vec<VarType>) -> FuncSymbol {
        manager.new_func_symbol(identifier, (return_type, parameters), true)
    }

    fn get_return_type(&self) -> FuncReturnType {
        match &self.symbol_type {
            (t, _) => t.clone(),
        }
    }

    fn get_parameters(&self) -> Vec<VarType> {
        match &self.symbol_type {
            (_, p) => p.clone(),
        }
    }
}