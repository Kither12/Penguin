use anyhow::{anyhow, Result};
use pest::error::{Error, ErrorVariant, InputLocation};
use pest::{iterators::Pairs, pratt_parser::PrattParser, Parser};
use pest::{Position, Span};
use pest_derive::Parser;
use std::cell::RefCell;
use std::rc::Rc;
use std::{iter::from_fn, sync::OnceLock};

use crate::environment::environment::{Environment, Var};
use crate::ProgramState;

use super::node::expression::{Expr, ExpressionPool};
use super::node::function::{ArgumentType, Func, FunctionCall};
use super::{
    ast::ASTNode,
    node::{
        conditional::IfElse,
        declaration::{AssignOperation, Assignment, Declaration},
        expression::{ExprAtom, Expression, OpType},
        io::Output,
        looping::WhileLoop,
        primitive::Primitive,
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
            .op(Op::infix(Rule::or_op, Left))
            .op(Op::infix(Rule::and_op, Left))
            .op(Op::infix(Rule::bit_or, Left))
            .op(Op::infix(Rule::bit_xor, Left))
            .op(Op::infix(Rule::bit_and, Left))
            .op(Op::infix(Rule::equal_op, Left) | Op::infix(Rule::nequal_op, Left))
            .op(Op::infix(Rule::lt_op, Left)
                | Op::infix(Rule::gt_op, Left)
                | Op::infix(Rule::lte_op, Left)
                | Op::infix(Rule::gte_op, Left))
            .op(Op::infix(Rule::shift_left, Left) | Op::infix(Rule::shift_right, Left))
            .op(Op::infix(Rule::add_op, Left) | Op::infix(Rule::sub_op, Left))
            .op(Op::infix(Rule::mul_op, Left)
                | Op::infix(Rule::div_op, Left)
                | Op::infix(Rule::mod_op, Left))
            .op(Op::prefix(Rule::pos_op)
                | Op::prefix(Rule::neg_op)
                | Op::prefix(Rule::opp_op)
                | Op::prefix(Rule::bit_not))
    })
}

fn parse_expr<'a>(
    pairs: Pairs<'a, Rule>,
    expr_pool: &RefCell<ExpressionPool>,
    environment: &RefCell<Environment<'a>>,
) -> Result<usize> {
    pratt_parser()
        .map_primary(|primary| match primary.as_rule() {
            Rule::function_call => {
                parse_function_call(primary.into_inner(), expr_pool, environment).map(|v| {
                    expr_pool.borrow_mut().add(Expression::Literal {
                        lhs: ExprAtom::FunctionCall(v),
                    })
                })
            }
            Rule::integer => Ok(expr_pool.borrow_mut().add(Expression::Literal {
                lhs: ExprAtom::Primitive(Primitive::Integer(primary.as_str().parse::<i64>()?)),
            })),
            Rule::boolean => Ok(expr_pool.borrow_mut().add(Expression::Literal {
                lhs: ExprAtom::Primitive(Primitive::Boolean(primary.as_str().parse::<bool>()?)),
            })),
            Rule::identifier => Ok(expr_pool.borrow_mut().add(Expression::Literal {
                lhs: ExprAtom::Var(environment.borrow_mut().register(primary.as_str())),
            })),
            Rule::expr => parse_expr(primary.into_inner(), expr_pool, environment),
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
                Rule::mod_op => OpType::Mod,
                Rule::gte_op => OpType::Gte,
                Rule::lte_op => OpType::Lte,
                Rule::gt_op => OpType::Gt,
                Rule::lt_op => OpType::Lt,
                Rule::equal_op => OpType::Eq,
                Rule::nequal_op => OpType::Neq,
                Rule::shift_left => OpType::ShiftLeft,
                Rule::shift_right => OpType::ShiftRight,
                Rule::bit_and => OpType::BitAnd,
                Rule::bit_or => OpType::BitOr,
                Rule::bit_xor => OpType::BitXor,
                _ => unreachable!(),
            };

            Ok(expr_pool.borrow_mut().add(Expression::Binary {
                lhs: Expr(lhs?),
                op: op_type,
                rhs: Expr(rhs?),
            }))
        })
        .map_prefix(|op, lhs| {
            let op_type = match op.as_rule() {
                Rule::pos_op => OpType::Add,
                Rule::neg_op => OpType::Sub,
                Rule::opp_op => OpType::Opp,
                Rule::bit_not => OpType::BitNot,
                _ => unreachable!(),
            };
            Ok(expr_pool.borrow_mut().add(Expression::Unary {
                lhs: Expr(lhs?),
                op: op_type,
            }))
        })
        .parse(pairs)
}
fn parse_function_declaration<'a>(
    pairs: Pairs<'a, Rule>,
    expr_pool: &RefCell<ExpressionPool>,
    environment: &RefCell<Environment<'a>>,
) -> Result<Func> {
    let mut pairs = pairs.peekable();
    let argument = from_fn(|| pairs.next_if(|pair| pair.as_rule().eq(&Rule::identifier)))
        .map(|v| environment.borrow_mut().register(v.as_str()))
        .collect::<Box<[Var]>>();
    let scope = parse_scope(pairs.next().unwrap().into_inner(), expr_pool, environment)?;
    Ok(Func::new(argument, scope))
}
fn parse_declaration<'a>(
    mut pairs: Pairs<'a, Rule>,
    expr_pool: &RefCell<ExpressionPool>,
    environment: &RefCell<Environment<'a>>,
) -> Result<Declaration> {
    let identifier = pairs.next().unwrap().as_str();
    let val = pairs.next().unwrap();
    match val.as_rule() {
        Rule::expr => {
            let v = parse_expr(val.into_inner(), expr_pool, environment)?;
            Ok(Declaration::Expression {
                var: environment.borrow_mut().register(identifier),
                expr: Expr(v),
            })
        }
        Rule::function_declaration => {
            let var = environment.borrow_mut().register(identifier);
            Ok(Declaration::Function {
                var,
                func: Rc::new(parse_function_declaration(
                    val.into_inner(),
                    expr_pool,
                    environment,
                )?),
            })
        }
        _ => unreachable!(),
    }
}

fn parse_assignment<'a>(
    mut pairs: Pairs<'a, Rule>,
    expr_pool: &RefCell<ExpressionPool>,
    environment: &RefCell<Environment<'a>>,
) -> Result<Assignment> {
    let identifier = pairs.next().unwrap().as_str();
    let op = match pairs.next().unwrap().as_rule() {
        Rule::assign_op => AssignOperation::AssignOp,
        Rule::cum_add => AssignOperation::AssignAdd,
        Rule::cum_sub => AssignOperation::AssignSub,
        Rule::cum_mul => AssignOperation::AssignMul,
        Rule::cum_div => AssignOperation::AssignDiv,
        _ => unreachable!(),
    };
    let v = parse_expr(pairs.next().unwrap().into_inner(), expr_pool, environment)?;
    Ok(Assignment::new(
        environment.borrow_mut().register(identifier),
        op,
        Expr(v),
    ))
}
fn parse_print_statement<'a>(
    mut pairs: Pairs<'a, Rule>,
    expr_pool: &RefCell<ExpressionPool>,
    environment: &RefCell<Environment<'a>>,
) -> Result<Output> {
    let output_type = pairs.next().unwrap().as_rule();
    match output_type {
        Rule::print => parse_expr(pairs.next().unwrap().into_inner(), expr_pool, environment)
            .map(|v| Output::new(Expr(v), String::from(""))),
        Rule::println => parse_expr(pairs.next().unwrap().into_inner(), expr_pool, environment)
            .map(|v| Output::new(Expr(v), String::from("\n"))),
        _ => unreachable!(),
    }
}
fn parse_scope<'a>(
    pairs: Pairs<'a, Rule>,
    expr_pool: &RefCell<ExpressionPool>,
    environment: &RefCell<Environment<'a>>,
) -> Result<Scope> {
    Ok(Scope::new(
        pairs
            .map(|pair| match pair.as_rule() {
                Rule::expr => parse_expr(pair.into_inner(), expr_pool, environment)
                    .map(|v| ASTNode::Expr(Expr(v))),
                Rule::assignment => parse_assignment(pair.into_inner(), expr_pool, environment)
                    .map(ASTNode::Assignment),
                Rule::declaration => parse_declaration(pair.into_inner(), expr_pool, environment)
                    .map(ASTNode::Declaration),
                Rule::scope => {
                    parse_scope(pair.into_inner(), expr_pool, environment).map(ASTNode::Scope)
                }
                Rule::ifelse => {
                    parse_if_else(pair.into_inner(), expr_pool, environment).map(ASTNode::IfElse)
                }
                Rule::while_loop => parse_while_loop(pair.into_inner(), expr_pool, environment)
                    .map(ASTNode::WhileLoop),
                Rule::print_statement => {
                    parse_print_statement(pair.into_inner(), expr_pool, environment)
                        .map(ASTNode::Output)
                }
                Rule::continue_statement => Ok(ASTNode::ContinueStatement),
                Rule::break_statement => Ok(ASTNode::BreakStatement),
                Rule::return_statement => parse_expr(pair.into_inner(), expr_pool, environment)
                    .map(|v| ASTNode::ReturnStatement(Expr(v))),
                _ => unreachable!(),
            })
            .collect::<Result<Box<[ASTNode]>>>()?,
    ))
}

fn parse_if_else<'a>(
    pairs: Pairs<'a, Rule>,
    expr_pool: &RefCell<ExpressionPool>,
    environment: &RefCell<Environment<'a>>,
) -> Result<IfElse> {
    let mut pairs = pairs.peekable();
    let if_clause = from_fn(|| pairs.next_if(|pair| pair.as_rule().ne(&Rule::r#else)))
        .map(|pair| {
            let mut inner = pair.into_inner();
            let expr_parsed =
                parse_expr(inner.next().unwrap().into_inner(), expr_pool, environment);
            let scope_parsed =
                parse_scope(inner.next().unwrap().into_inner(), expr_pool, environment);
            expr_parsed.and_then(|a| scope_parsed.map(|b| (Expr(a), b)))
        })
        .collect::<Result<Box<[(Expr, Scope)]>>>()?;

    let else_clause = pairs
        .next()
        .map(|v| parse_scope(v.into_inner(), expr_pool, environment))
        .transpose()?;
    Ok(IfElse::new(if_clause, else_clause))
}

fn parse_while_loop<'a>(
    mut pairs: Pairs<'a, Rule>,
    expr_pool: &RefCell<ExpressionPool>,
    environment: &RefCell<Environment<'a>>,
) -> Result<WhileLoop> {
    let v = parse_expr(pairs.next().unwrap().into_inner(), expr_pool, environment)?;
    let scope_parsed = parse_scope(pairs.next().unwrap().into_inner(), expr_pool, environment)?;
    Ok(WhileLoop::new(Expr(v), scope_parsed))
}

fn handle_parse_error(code: &str, e: Error<Rule>) -> anyhow::Error {
    let err = match e.location {
        InputLocation::Pos(v) => Error::new_from_pos(
            ErrorVariant::<()>::CustomError {
                message: String::from("Parse error occurred"),
            },
            Position::new(code, v).unwrap(),
        ),
        InputLocation::Span((u, v)) => Error::new_from_span(
            ErrorVariant::<()>::CustomError {
                message: String::from("Parse error occurred"),
            },
            Span::new(code, u, v).unwrap(),
        ),
    };
    anyhow!(err)
}

pub fn parse_function_call<'a>(
    mut pairs: Pairs<'a, Rule>,
    expr_pool: &RefCell<ExpressionPool>,
    environment: &RefCell<Environment<'a>>,
) -> Result<FunctionCall> {
    let identifier = pairs.next().unwrap().as_str();
    let argument_input = pairs
        .map(|v| match v.as_rule() {
            Rule::function_declaration => {
                parse_function_declaration(v.into_inner(), expr_pool, environment)
                    .map(ArgumentType::Func)
            }
            Rule::expr => parse_expr(v.into_inner(), expr_pool, environment)
                .map(|v| ArgumentType::Expr(Expr(v))),
            Rule::ref_var => Ok(ArgumentType::Ref(
                environment
                    .borrow_mut()
                    .register(v.into_inner().next().unwrap().as_str()),
            )),
            _ => unreachable!(),
        })
        .collect::<Result<Box<[ArgumentType]>>>()?;
    Ok(FunctionCall::new(
        environment.borrow_mut().register(identifier),
        argument_input,
    ))
}

pub fn parse_ast(code: &str) -> Result<(ASTNode, ProgramState)> {
    let pairs = CParser::parse(Rule::code, code)
        .map_err(|e| handle_parse_error(code, e))?
        .next()
        .unwrap()
        .into_inner();
    let expr_pool = RefCell::new(ExpressionPool::new());
    let environment = RefCell::new(Environment::default());

    parse_scope(pairs, &expr_pool, &environment).map(|v| {
        (
            ASTNode::Scope(v),
            ProgramState::new(expr_pool.into_inner(), environment),
        )
    })
}
