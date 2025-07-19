use std::collections::HashMap;
use std::sync::Arc;
use crate::ast::{Value};

pub type BuiltinFn = Arc<dyn Fn(Vec<Value>) ->
Result<Value, String> + Send + Sync>;

pub struct Runtime {
    pub functions: HashMap<String, BuiltinFn>,
}

impl Runtime {
    pub fn new() -> Self {
        let mut functions = HashMap::new();

        functions.insert("say".to_string(), Arc::new(|args: Vec<Value>| -> Result<Value, String> {
            for a in args {
                print!("{a} ");
            }
            println!();
            Ok(Value::Unit)
        }) as Arc<dyn Fn(Vec<Value>) -> Result<Value, String> + Send + Sync>);

        Self { functions }
    }
    pub fn get_function(&self, name: &str) -> Result<BuiltinFn, String> {
        self.functions
            .get(name)
            .cloned()
            .ok_or_else(|| format!("unknown function: '{name}'"))
    }
}