use anyhow::{anyhow, Context, Result};
use pest::{
    iterators::{Pair, Pairs},
    pratt_parser::PrattParser,
    Parser,
};
use pest_derive::Parser;
use std::{str::FromStr, sync::OnceLock};

use super::{
    ast::ASTNode,
    node::{
        chess_notation::ChessNotation,
        expression::{Expression, OpType, Operation},
        primitive::Integer,
    },
};

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
struct CParser;

fn pratt_parser() -> &'static PrattParser<Rule> {
    use pest::pratt_parser::{Assoc::*, Op};
    static PRATT_PARSER: OnceLock<PrattParser<Rule>> = OnceLock::new();
    PRATT_PARSER.get_or_init(|| {
        PrattParser::new()
            .op(Op::infix(Rule::add_op, Left) | Op::infix(Rule::sub_op, Left))
            .op(Op::infix(Rule::mul_op, Left) | Op::infix(Rule::div_op, Left))
    })
}

fn parse_expr(pairs: Pairs<Rule>) -> Result<Expression> {
    pratt_parser()
        .map_primary(|primary| match primary.as_rule() {
            Rule::integer => Ok(Expression::Literal {
                lhs: Box::new(Integer::from_str(primary.as_str())?),
            }),
            Rule::expr => parse_expr(primary.into_inner()),
            rule => unreachable!("Expr: Unexpected rule: {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let op_node = Operation {
                op_type: match op.as_rule() {
                    Rule::add_op => OpType::Add,
                    Rule::sub_op => OpType::Sub,
                    Rule::mul_op => OpType::Mul,
                    Rule::div_op => OpType::Div,
                    rule => unreachable!("Expr: Unexpected rule: {:?}", rule),
                },
                chess_notation: ChessNotation::new(
                    op.into_inner().next().unwrap().as_str().to_owned(),
                ),
            };
            Ok(Expression::Binary {
                lhs: Box::new(lhs?),
                op: op_node,
                rhs: Box::new(rhs?),
            })
        })
        .parse(pairs)
}
pub fn parse_ast(line: &String) -> Result<Box<ASTNode>> {
    let pair = CParser::parse(Rule::code, line)
        .context("Failed to parser")?
        .next()
        .unwrap()
        .into_inner()
        .next()
        .unwrap();
    match pair.as_rule() {
        Rule::expr => Ok(Box::new(ASTNode::Expr(parse_expr(pair.into_inner())?))),
        rule => unreachable!("Expr: Unexpected rule: {:?}", rule),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_expression() {
        let ast = parse_ast(&String::from("2 `e4`+ (2 `e5`+ 2) `e4`* 4;")).unwrap();
        println!("{:#?}", ast.as_ref());
    }
    #[test]
    fn test_evaluate_expression() {
        let ast = parse_ast(&String::from("2 `e4`+ (2 `e5`+ 2) `e4`* 4;")).unwrap();
        if let ASTNode::Expr(expr) = ast.as_ref() {
            let eval = expr.evaluation().unwrap();
            println!("{:#?}", eval)
        }
    }
}