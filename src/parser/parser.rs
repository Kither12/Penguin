use anyhow::{anyhow, Result};
use pest::error::{Error, ErrorVariant, InputLocation};
use pest::{iterators::Pairs, pratt_parser::PrattParser, Parser};
use pest::{Position, Span};
use pest_derive::Parser;
use std::cell::RefCell;
use std::rc::Rc;
use std::{iter::from_fn, sync::OnceLock};

use super::node::expression::ExpressionPool;
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

fn parse_expr<'a>(pairs: Pairs<'a, Rule>, pool: Rc<RefCell<ExpressionPool<'a>>>) -> Result<usize> {
    pratt_parser()
        .map_primary(|primary| match primary.as_rule() {
            Rule::function_call => parse_function_call(primary.into_inner()).map(|v| {
                pool.borrow_mut().add(Expression::Literal {
                    lhs: ExprAtom::FunctionCall(v),
                })
            }),
            Rule::integer => Ok(pool.borrow_mut().add(Expression::Literal {
                lhs: ExprAtom::Primitive(Primitive::Integer(primary.as_str().parse::<i64>()?)),
            })),
            Rule::boolean => Ok(pool.borrow_mut().add(Expression::Literal {
                lhs: ExprAtom::Primitive(Primitive::Boolean(primary.as_str().parse::<bool>()?)),
            })),
            Rule::identifier => Ok(pool.borrow_mut().add(Expression::Literal {
                lhs: ExprAtom::Identifier(primary.as_str()),
            })),
            Rule::expr => parse_expr(primary.into_inner(), Rc::clone(&pool)),
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

            Ok(pool.borrow_mut().add(Expression::Binary {
                lhs: lhs?,
                op: op_type,
                rhs: rhs?,
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
            Ok(pool.borrow_mut().add(Expression::Unary {
                lhs: lhs?,
                op: op_type,
            }))
        })
        .parse(pairs)
}
fn parse_function_declaration<'a>(pairs: Pairs<'a, Rule>) -> Result<Func<'a>> {
    let mut pairs = pairs.peekable();
    let argument = from_fn(|| pairs.next_if(|pair| pair.as_rule().eq(&Rule::identifier)))
        .map(|v| v.as_str())
        .collect::<Vec<&str>>();
    let scope = parse_scope(pairs.next().unwrap().into_inner())?;
    Ok(Func::new(argument, scope))
}
fn parse_declaration<'a>(mut pairs: Pairs<'a, Rule>) -> Result<Declaration<'a>> {
    let identifier = pairs.next().unwrap().as_str();
    let val = pairs.next().unwrap();
    match val.as_rule() {
        Rule::expr => {
            let expr_pool = Rc::new(RefCell::new(ExpressionPool::new()));
            parse_expr(val.into_inner(), Rc::clone(&expr_pool))?;
            Ok(Declaration::Expression {
                identifier,
                expr_pool: Rc::try_unwrap(expr_pool).unwrap().into_inner(),
            })
        }
        Rule::function_declaration => Ok(Declaration::Function {
            identifier,
            func: Rc::new(parse_function_declaration(val.into_inner())?),
        }),
        _ => unreachable!(),
    }
}

fn parse_assignment<'a>(mut pairs: Pairs<'a, Rule>) -> Result<Assignment<'a>> {
    let identifier = pairs.next().unwrap().as_str();
    let op = match pairs.next().unwrap().as_rule() {
        Rule::assign_op => AssignOperation::AssignOp,
        Rule::cum_add => AssignOperation::AssignAdd,
        Rule::cum_sub => AssignOperation::AssignSub,
        Rule::cum_mul => AssignOperation::AssignMul,
        Rule::cum_div => AssignOperation::AssignDiv,
        _ => unreachable!(),
    };
    let expr_pool = Rc::new(RefCell::new(ExpressionPool::new()));
    parse_expr(pairs.next().unwrap().into_inner(), Rc::clone(&expr_pool))?;
    Ok(Assignment::new(
        identifier,
        op,
        Rc::try_unwrap(expr_pool).unwrap().into_inner(),
    ))
}
fn parse_print_statement<'a>(mut pairs: Pairs<'a, Rule>) -> Result<Output<'a>> {
    let output_type = pairs.next().unwrap().as_rule();
    let expr_pool = Rc::new(RefCell::new(ExpressionPool::new()));
    match output_type {
        Rule::print => parse_expr(pairs.next().unwrap().into_inner(), Rc::clone(&expr_pool))
            .map(|_| Output::new(Rc::try_unwrap(expr_pool).unwrap().into_inner(), "")),
        Rule::println => parse_expr(pairs.next().unwrap().into_inner(), Rc::clone(&expr_pool))
            .map(|_| Output::new(Rc::try_unwrap(expr_pool).unwrap().into_inner(), "\n")),
        _ => unreachable!(),
    }
}
fn parse_scope<'a>(pairs: Pairs<'a, Rule>) -> Result<Scope<'a>> {
    Ok(Scope::new(
        pairs
            .map(|pair| match pair.as_rule() {
                Rule::expr => {
                    let expr_pool = Rc::new(RefCell::new(ExpressionPool::new()));
                    parse_expr(pair.into_inner(), Rc::clone(&expr_pool))
                        .map(|_| ASTNode::Expr(Rc::try_unwrap(expr_pool).unwrap().into_inner()))
                }
                Rule::assignment => {
                    parse_assignment(pair.into_inner()).map(|v| ASTNode::Assignment(v))
                }
                Rule::declaration => {
                    parse_declaration(pair.into_inner()).map(|v| ASTNode::Declaration(v))
                }
                Rule::scope => parse_scope(pair.into_inner()).map(|v| ASTNode::Scope(v)),
                Rule::ifelse => parse_if_else(pair.into_inner()).map(|v| ASTNode::IfElse(v)),
                Rule::while_loop => {
                    parse_while_loop(pair.into_inner()).map(|v| ASTNode::WhileLoop(v))
                }
                Rule::print_statement => {
                    parse_print_statement(pair.into_inner()).map(|v| ASTNode::Output(v))
                }
                Rule::continue_statement => Ok(ASTNode::ContinueStatement),
                Rule::break_statement => Ok(ASTNode::BreakStatement),
                Rule::return_statement => {
                    let expr_pool = Rc::new(RefCell::new(ExpressionPool::new()));
                    parse_expr(pair.into_inner(), Rc::clone(&expr_pool)).map(|_| {
                        ASTNode::ReturnStatement(Rc::try_unwrap(expr_pool).unwrap().into_inner())
                    })
                }
                _ => unreachable!(),
            })
            .collect::<Result<Vec<ASTNode>>>()?,
    ))
}

fn parse_if_else<'a>(pairs: Pairs<'a, Rule>) -> Result<IfElse<'a>> {
    let mut pairs = pairs.peekable();
    let if_clause = from_fn(|| pairs.next_if(|pair| pair.as_rule().ne(&Rule::r#else)))
        .map(|pair| {
            let expr_pool = Rc::new(RefCell::new(ExpressionPool::new()));
            let mut inner = pair.into_inner();
            let expr_parsed = parse_expr(inner.next().unwrap().into_inner(), Rc::clone(&expr_pool));
            let scope_parsed = parse_scope(inner.next().unwrap().into_inner());
            expr_parsed.and_then(|_| {
                scope_parsed.map(|b| (Rc::try_unwrap(expr_pool).unwrap().into_inner(), b))
            })
        })
        .collect::<Result<Vec<(ExpressionPool, Scope)>>>()?;

    let else_clause = pairs
        .next()
        .map(|v| parse_scope(v.into_inner()))
        .transpose()?;
    Ok(IfElse::new(if_clause, else_clause))
}

fn parse_while_loop<'a>(mut pairs: Pairs<'a, Rule>) -> Result<WhileLoop<'a>> {
    let expr_pool = Rc::new(RefCell::new(ExpressionPool::new()));
    parse_expr(pairs.next().unwrap().into_inner(), Rc::clone(&expr_pool))?;
    let scope_parsed = parse_scope(pairs.next().unwrap().into_inner())?;
    Ok(WhileLoop::new(
        Rc::try_unwrap(expr_pool).unwrap().into_inner(),
        scope_parsed,
    ))
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

pub fn parse_function_call<'a>(mut pairs: Pairs<'a, Rule>) -> Result<FunctionCall<'a>> {
    let identifier = pairs.next().unwrap().as_str();
    let argument_input = pairs
        .map(|v| match v.as_rule() {
            Rule::function_declaration => {
                parse_function_declaration(v.into_inner()).map(|v| ArgumentType::Func(v))
            }
            Rule::expr => {
                let expr_pool = Rc::new(RefCell::new(ExpressionPool::new()));
                parse_expr(v.into_inner(), Rc::clone(&expr_pool))
                    .map(|_| ArgumentType::Expr(Rc::try_unwrap(expr_pool).unwrap().into_inner()))
            }
            Rule::ref_var => Ok(ArgumentType::Ref(v.into_inner().next().unwrap().as_str())),
            _ => unreachable!(),
        })
        .collect::<Result<Vec<ArgumentType>>>()?;
    Ok(FunctionCall::new(identifier, argument_input))
}

pub fn parse_ast(code: &str) -> Result<ASTNode> {
    let pairs = CParser::parse(Rule::code, code)
        .map_err(|e| handle_parse_error(code, e))?
        .next()
        .unwrap()
        .into_inner();
    parse_scope(pairs).map(|v| ASTNode::Scope(v))
}
