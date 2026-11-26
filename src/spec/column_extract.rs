use std::ops::ControlFlow;

use anyhow::Result;
use sqlparser::{ast::Expr, ast::visit_expressions, dialect::GenericDialect, parser::Parser};

/// Parse sql expressions from the config to determine required columns
pub(crate) fn extract_required_columns(sql_expr: &str) -> Result<Vec<String>> {
    let dialect = GenericDialect {};
    let expr = Parser::new(&dialect).try_with_sql(sql_expr)?.parse_expr()?;

    let mut cols: Vec<String> = Vec::new();

    let _ = visit_expressions(&expr, |e| {
        let name = match e {
            Expr::Identifier(id) => Some(id.value.clone()),
            Expr::CompoundIdentifier(parts) => Some(
                parts
                    .iter()
                    .map(|p| p.value.as_str())
                    .collect::<Vec<_>>()
                    .join("."),
            ),
            _ => None,
        };

        if let Some(n) = name {
            if !cols.contains(&n) {
                cols.push(n);
            }
        }
        ControlFlow::<()>::Continue(())
    });
    Ok(cols)
}
