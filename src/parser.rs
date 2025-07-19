use crate::ast::{Expr};

pub fn parse(input: &str) -> Result<Expr, String> {
    let tokens: Vec<&str> = input.trim().split_whitespace().collect();
    if tokens.is_empty() {
        return Err("empty input".to_string());
    }

    let head = tokens[0].to_string();
    let args = tokens[1..]
        .iter()
        .map(|t| {
            if let Ok(n) = t.parse::<f64>() {
                Ok(Expr::Number(n))
            } else if t.starts_with('"') && t.ends_with('"') {
                Ok(Expr::StringLiteral(t.trim_matches('"').to_string()))
            } else {
                Ok(Expr::Variable(t.to_string()))
            }
        })
        .collect::<Result<Vec<_>, String>>()?;

    Ok(Expr::Call(head, args))
}