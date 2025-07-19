use crate::ast::{Expr, Value};
use crate::runtime::Runtime;

pub fn eval(expr: Expr, runtime: &mut Runtime) -> Result<Value, String> {
    match expr {
        Expr::StringLiteral(s) => Ok(Value::Str(s)),
        Expr::Number(n) => Ok(Value::Num(n)),
        Expr::Variable(name) => Err(format!("undefined variable: '{name}'")),
        Expr::Call(name, args) => {
            let func = runtime.get_function(&name)?;
            let evaluated_args = evaluate_args(args, runtime)?;
            func(evaluated_args)
        }
    }
}

fn evaluate_args(args: Vec<Expr>, runtime: &mut Runtime) -> Result<Vec<Value>, String> {
    args.into_iter()
        .map(|arg| eval(arg, runtime))
        .collect()
}

