use std::ops::ControlFlow;

use anyhow::{Result, bail};
use datafusion::sql::sqlparser::tokenizer::{Token, Tokenizer, Whitespace};
// spec/verify_sql.rs imports
use datafusion::logical_expr::sqlparser::ast::{Expr, visit_expressions};
use datafusion::logical_expr::sqlparser::dialect::GenericDialect;
use datafusion::logical_expr::sqlparser::keywords::Keyword;
use datafusion::logical_expr::sqlparser::parser::Parser;

const FORBIDDEN_KEYWORDS: [Keyword; 7] = [
    Keyword::DROP,
    Keyword::DELETE,
    Keyword::INSERT,
    Keyword::UPDATE,
    Keyword::ALTER,
    Keyword::CREATE,
    Keyword::TRUNCATE,
];

/// Parse a sql_expr, enforcing all safety checks. Every other function does through here
pub fn parse(sql: &str) -> Result<Expr> {
    check_tokens(&sql)?;

    let mut parser = Parser::new(&GenericDialect {}).try_with_sql(sql)?;
    let expr = parser.parse_expr()?;

    if parser.peek_token().token != Token::EOF {
        bail!("unexpected trailing input");
    }

    reject_subqueries(&expr)?;
    Ok(expr)
}

/// Parse sql expressions from the config to determine required columns
pub fn extract_required_columns(sql_expr: &str) -> Result<Vec<String>> {
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

fn check_tokens(sql: &str) -> Result<()> {
    for token in Tokenizer::new(&GenericDialect {}, sql).tokenize()? {
        match token {
            Token::SemiColon => bail!("`;` is not allowed"),
            Token::Whitespace(
                Whitespace::SingleLineComment { .. } | Whitespace::MultiLineComment(_),
            ) => bail!("comments are not allowed"),
            Token::Word(w) if FORBIDDEN_KEYWORDS.contains(&w.keyword) => {
                bail!("`{}` is not allowed", w.value.to_uppercase())
            }
            _ => {}
        }
    }
    Ok(())
}

fn reject_subqueries(expr: &Expr) -> Result<()> {
    let found = visit_expressions(expr, |e| {
        if matches!(
            e,
            Expr::Subquery(_) | Expr::Exists { .. } | Expr::InSubquery { .. }
        ) {
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    });
    if found.is_break() {
        bail!("Subqueries are not allowed")
    }
    Ok(())
}
