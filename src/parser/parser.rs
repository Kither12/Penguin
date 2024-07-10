use anyhow::{anyhow, Context, Result};
use pest::{
    iterators::{Pair, Pairs},
    pratt_parser::PrattParser,
    Parser,
};
use pest_derive::Parser;
use std::sync::OnceLock;

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

#[derive(Debug)]
enum OpType {
    Add,
    Sub,
    Mul,
    Div,
}
#[derive(Debug)]
enum ASTNode {
    Root(Box<ASTNode>),
    ChessNotation(String),
    Integer(String),
    Expr {
        lhs: Box<ASTNode>,
        op: Box<ASTNode>,
        rhs: Box<ASTNode>,
    },
    Operation {
        op_type: OpType,
        chess_notation: Box<ASTNode>,
    },
}
fn parse_chess_notation(pair: Pair<Rule>) -> Result<Box<ASTNode>> {
    match pair.as_rule() {
        Rule::chess_notation => Ok(Box::new(ASTNode::ChessNotation(pair.as_str().to_owned()))),
        rule => Err(anyhow!("Unexpected rule: {:?}", rule)),
    }
}
fn parse_expr(pairs: Pairs<Rule>) -> Result<Box<ASTNode>> {
    pratt_parser()
        .map_primary(|primary| match primary.as_rule() {
            Rule::integer => Ok(Box::new(ASTNode::Integer(primary.as_str().to_owned()))),
            Rule::expr => parse_expr(primary.into_inner()),
            rule => unreachable!("Unexpected rule: {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let op_node = ASTNode::Operation {
                op_type: match op.as_rule() {
                    Rule::add_op => OpType::Add,
                    Rule::sub_op => OpType::Sub,
                    Rule::mul_op => OpType::Mul,
                    Rule::div_op => OpType::Div,
                    rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
                },
                chess_notation: parse_chess_notation(op.into_inner().next().unwrap())?,
            };
            Ok(Box::new(ASTNode::Expr {
                lhs: lhs?,
                op: Box::new(op_node),
                rhs: rhs?,
            }))
        })
        .parse(pairs)
}
fn parse_ast(line: &String) -> Result<Box<ASTNode>> {
    let pair = CParser::parse(Rule::code, line)
        .context("Failed to parser")?
        .next()
        .unwrap()
        .into_inner()
        .next()
        .unwrap();
    match pair.as_rule() {
        Rule::expr => parse_expr(pair.into_inner()),
        rule => Err(anyhow!("Unexpected rule: {:?}", rule)),
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
}
