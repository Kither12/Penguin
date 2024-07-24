use anyhow::{Context, Result};
use pest::{
    iterators::Pairs,
    pratt_parser::{Op, PrattParser},
    Parser,
};
use pest_derive::Parser;
use std::{borrow::BorrowMut, iter::from_fn, str::FromStr, sync::OnceLock};

use crate::parser::node::expression;

use super::{
    ast::ASTNode,
    node::{
        conditional::IfElse,
        declaration::{Assignment, Declaration},
        expression::{ExprAtom, Expression, OpType},
        primitive::Integer,
        scope::Scope,
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
            .op(Op::infix(Rule::equal_op, Left) | Op::infix(Rule::nequal_op, Left))
            .op(Op::infix(Rule::lt_op, Left)
                | Op::infix(Rule::gt_op, Left)
                | Op::infix(Rule::lte_op, Left)
                | Op::infix(Rule::gte_op, Left))
            .op(Op::infix(Rule::add_op, Left) | Op::infix(Rule::sub_op, Left))
            .op(Op::infix(Rule::mul_op, Left) | Op::infix(Rule::div_op, Left))
            .op(Op::prefix(Rule::pos_op) | Op::prefix(Rule::neg_op) | Op::prefix(Rule::opp_op))
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
            _ => unreachable!(),
        })
        .map_infix(|lhs, op, rhs| {
            let op_type = match op.as_rule() {
                Rule::add_op => OpType::Add,
                Rule::sub_op => OpType::Sub,
                Rule::mul_op => OpType::Mul,
                Rule::div_op => OpType::Div,
                Rule::and_op => OpType::And,
                Rule::or_op => OpType::Or,
                Rule::gte_op => OpType::Gte,
                Rule::lte_op => OpType::Lte,
                Rule::gt_op => OpType::Gte,
                Rule::lt_op => OpType::Lte,
                Rule::equal_op => OpType::Eq,
                Rule::nequal_op => OpType::Neq,
                _ => unreachable!(),
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
                Rule::opp_op => OpType::Opp,
                _ => unreachable!(),
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
pub fn parse_scope<'a>(pairs: &mut Pairs<'a, Rule>) -> Result<Scope<'a>> {
    Ok(Scope::new(
        pairs
            .map(|pair| match pair.as_rule() {
                Rule::expr => {
                    parse_expr(pair.into_inner()).and_then(|v| Ok(Box::new(ASTNode::Expr(v))))
                }
                Rule::assignment => parse_assignment(pair.into_inner().borrow_mut())
                    .and_then(|v| Ok(Box::new(ASTNode::Assignment(v)))),
                Rule::declaration => parse_declaration(pair.into_inner().borrow_mut())
                    .and_then(|v| Ok(Box::new(ASTNode::Declaration(v)))),
                Rule::scope => parse_scope(pair.into_inner().borrow_mut())
                    .and_then(|v| Ok(Box::new(ASTNode::Scope(v)))),
                Rule::ifelse => parse_if_else(pair.into_inner().borrow_mut())
                    .and_then(|v| Ok(Box::new(ASTNode::IfElse(v)))),
                _ => unreachable!(),
            })
            .collect::<Result<Vec<Box<ASTNode>>>>()?,
    ))
}

pub fn parse_if_else<'a>(pairs: &mut Pairs<'a, Rule>) -> Result<IfElse<'a>> {
    let mut pairs = pairs.peekable();
    let if_clause = from_fn(|| {
        pairs.next_if(|pair| match pair.as_rule() {
            Rule::r#if | Rule::elif => true,
            Rule::r#else => false,
            _ => unreachable!(),
        })
    })
    .map(|pair| {
        let mut inner = pair.into_inner();
        let expr_parsed = parse_expr(inner.next().unwrap().into_inner());
        let scope_parsed = parse_scope(inner.next().unwrap().into_inner().borrow_mut());
        expr_parsed.and_then(|a| scope_parsed.map(|b| (a, b)))
    })
    .collect::<Result<Vec<(Expression, Scope)>>>()?;

    let else_clause = pairs
        .next()
        .and_then(|v| Some(parse_scope(v.into_inner().borrow_mut())))
        .transpose()?;
    Ok(IfElse::new(if_clause, else_clause))
}

pub fn parse_ast(code: &str) -> Result<Box<ASTNode>> {
    let pairs = CParser::parse(Rule::code, code)
        .context("Failed to parser")?
        .next()
        .unwrap()
        .into_inner();
    Ok(Box::new(ASTNode::Scope(Scope::new(
        pairs
            .map(|pair| match pair.as_rule() {
                Rule::expr => {
                    parse_expr(pair.into_inner()).and_then(|v| Ok(Box::new(ASTNode::Expr(v))))
                }
                Rule::assignment => parse_assignment(pair.into_inner().borrow_mut())
                    .and_then(|v| Ok(Box::new(ASTNode::Assignment(v)))),
                Rule::declaration => parse_declaration(pair.into_inner().borrow_mut())
                    .and_then(|v| Ok(Box::new(ASTNode::Declaration(v)))),
                Rule::scope => parse_scope(pair.into_inner().borrow_mut())
                    .and_then(|v| Ok(Box::new(ASTNode::Scope(v)))),
                Rule::ifelse => parse_if_else(pair.into_inner().borrow_mut())
                    .and_then(|v| Ok(Box::new(ASTNode::IfElse(v)))),
                rule => unreachable!("Unexpected rule: {:?}", rule),
            })
            .collect::<Result<Vec<Box<ASTNode>>>>()?,
    ))))
}
