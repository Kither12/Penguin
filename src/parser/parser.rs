use anyhow::{Context, Result};
use pest::{iterators::Pairs, pratt_parser::PrattParser, Parser};
use pest_derive::Parser;
use std::{borrow::BorrowMut, str::FromStr, sync::OnceLock};

use super::{
    ast::ASTNode,
    node::{
        declaration::{Assignment, Declaration},
        expression::{ExprAtom, Expression, OpType},
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
            .op(Op::prefix(Rule::pos_op) | Op::prefix(Rule::neg_op))
    })
}

fn parse_expr(pairs: Pairs<Rule>) -> Result<Expression> {
    pratt_parser()
        .map_primary(|primary| match primary.as_rule() {
            Rule::integer => Ok(Expression::Literal {
                lhs: ExprAtom::Primitive(Box::new(Integer::from_str(primary.as_str())?)),
            }),
            Rule::identifier => Ok(Expression::Literal {
                lhs: ExprAtom::Identifier(primary.as_str()),
            }),
            Rule::expr => parse_expr(primary.into_inner()),
            rule => unreachable!("Expr: Unexpected rule: {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let op_type = match op.as_rule() {
                Rule::add_op => OpType::Add,
                Rule::sub_op => OpType::Sub,
                Rule::mul_op => OpType::Mul,
                Rule::div_op => OpType::Div,
                rule => unreachable!("Expr: Unexpected rule: {:?}", rule),
            };

            Ok(Expression::Binary {
                lhs: Box::new(lhs?),
                op: op_type,
                rhs: Box::new(rhs?),
            })
        })
        .map_prefix(|op, lhs| {
            let op_type = match op.as_rule() {
                Rule::pos_op => OpType::Add,
                Rule::neg_op => OpType::Sub,
                rule => unreachable!("Expr: Unexpected rule: {:?}", rule),
            };
            Ok(Expression::Unary {
                lhs: Box::new(lhs?),
                op: op_type,
            })
        })
        .parse(pairs)
}
pub fn parse_declaration<'a>(pairs: &mut Pairs<'a, Rule>) -> Result<Declaration<'a>> {
    let identifier = pairs.next().unwrap().as_str();
    let expr = parse_expr(pairs.next().unwrap().into_inner())?;
    Ok(Declaration::new(identifier, expr))
}

pub fn parse_assignment<'a>(pairs: &mut Pairs<'a, Rule>) -> Result<Assignment<'a>> {
    let identifier = pairs.next().unwrap().as_str();
    let expr = parse_expr(pairs.next().unwrap().into_inner())?;
    Ok(Assignment::new(identifier, expr))
}

pub fn parse_ast(code: &str) -> Result<Box<ASTNode>> {
    let pairs = CParser::parse(Rule::code, code)
        .context("Failed to parser")?
        .next()
        .unwrap()
        .into_inner();
    Ok(Box::new(ASTNode::Root(
        pairs
            .map(|pair| match pair.as_rule() {
                Rule::expr => {
                    parse_expr(pair.into_inner()).and_then(|v| Ok(Box::new(ASTNode::Expr(v))))
                }
                Rule::assignment => parse_assignment(pair.into_inner().borrow_mut())
                    .and_then(|v| Ok(Box::new(ASTNode::Assignment(v)))),
                Rule::declaration => parse_declaration(pair.into_inner().borrow_mut())
                    .and_then(|v| Ok(Box::new(ASTNode::Declaration(v)))),
                rule => unreachable!("Unexpected rule: {:?}", rule),
            })
            .collect::<Result<Vec<Box<ASTNode>>>>()?,
    )))
}
