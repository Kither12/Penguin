use anyhow::{anyhow, Result};
use environment::environment::Environment;
use parser::{
    ast::ASTNode,
    node::scope::{FlowStatement, ScopeError},
    parser::parse_ast,
};

pub mod environment;
pub mod error;
pub mod parser;

pub fn run_code(code: &str) -> Result<()> {
    let ast_root = parse_ast(code)?;
    let mut environment = Environment::default();
    if let ASTNode::Scope(v) = ast_root {
        for node in v.code.iter() {
            let mut flow_statement: Option<FlowStatement> = None;
            match node {
                ASTNode::Expr(v) => environment = v.execute(environment)?.0,
                ASTNode::Declaration(v) => environment = v.execute(environment)?,
                ASTNode::Assignment(v) => environment = v.execute(environment)?,
                ASTNode::Scope(v) => (environment, flow_statement) = v.execute(environment)?,
                ASTNode::IfElse(v) => (environment, flow_statement) = v.execute(environment)?,
                ASTNode::WhileLoop(v) => (environment, flow_statement) = v.execute(environment)?,
                ASTNode::BreakStatement => flow_statement = Some(FlowStatement::Break),
                ASTNode::ReturnStatement => flow_statement = Some(FlowStatement::Return),
                ASTNode::ContinueStatement => flow_statement = Some(FlowStatement::Continue),
            }
            match flow_statement {
                Some(FlowStatement::Break) => Err(anyhow!(ScopeError::BreakOutsideLoop))?,
                Some(FlowStatement::Return) => Err(anyhow!(ScopeError::ReturnOutsideFunction))?,
                Some(FlowStatement::Continue) => Err(anyhow!(ScopeError::ContinueOutsideLoop))?,
                None => {}
            };
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn while_loop_should_work() {
        let res = run_code(
            "   
                gimme i = 0;
                while  i < 10{
                    i += 1;
                }
                i;
            ",
        );
        assert!(res.is_ok());
    }

    #[test]
    fn nested_if_else_should_work() {
        let res = run_code(
            "
                if 2 != 2{
                    3 + 3;
                }
                elif 3 == 3{
                    if 4 != 4{
                        9 + 9;
                    }
                    else{
                        5 + 5;
                    }
                }
                else{
                    4 + 4;
                }
            ",
        );
        assert!(res.is_ok());
    }

    #[test]
    fn if_else_should_work() {
        let res = run_code(
            "
                if 2 != 2{
                    3 + 3;
                }
                else{
                    4 + 4;
                }
            ",
        );
        assert!(res.is_ok());
    }

    #[test]
    fn if_should_work() {
        let res = run_code(
            "
                if 2 == 2{
                    3 + 3;
                }
            ",
        );
        assert!(res.is_ok());
    }

    #[test]
    fn assign_should_work() {
        let res = run_code(
            "
                gimme a = 3;
                {
                    a = 2;
                }
                a;
            ",
        );
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    #[test]
    fn shadowing_should_work() {
        let res = run_code(
            "
                gimme a = 3;
                {
                    gimme a = 1;
                    a;
                }
                a;
            ",
        );
        assert!(res.is_ok());
    }

    #[test]
    fn not_declare_should_fail() {
        let res = run_code(
            "
                a = 3;
            ",
        );
        assert!(res.is_err());
    }
    #[test]
    fn reassign_should_fail() {
        let res = run_code(
            "
                gimme a = 2;
                gimme a = 3;
            ",
        );
        assert!(res.is_err());
    }
    #[test]
    fn function_should_work() {
        let res = run_code(
            "
                gimme a = () => {
                    gimme a = 2 * 2 + 2;
                    a;
                };
                a();
            ",
        );
        assert!(res.is_ok());
    }
    #[test]
    fn function_with_parameter_should_works() {
        let res = run_code(
            "
                gimme a = (a, b) => {
                    a + b;
                };
                a(2, 3);
            ",
        );
        assert!(res.is_ok());
    }
    #[test]
    fn function_with_redeclare_parameter_should_work() {
        let res = run_code(
            "
                gimme a = (a) => {
                    gimme a = 4;
                    a;
                };
                a(2);
            ",
        );
        assert!(res.is_ok());
    }
    #[test]
    fn function_missing_parameter_should_fail() {
        let res = run_code(
            "
                gimme a = (a) => {
                    gimme a = 4;
                    a;
                };
                a();
            ",
        );
        assert!(res.is_err());
    }
    #[test]
    fn function_too_many_parameter_should_fail() {
        let res = run_code(
            "
                gimme a = (a) => {
                    gimme a = 4;
                    a;
                };
                a(2, 3);
            ",
        );
        assert!(res.is_err());
    }
    #[test]
    fn function_local_variable_should_drop() {
        let res = run_code(
            "
                gimme a = (a) => {
                    gimme b = 4;
                    b;
                };
                a(2);
                b;
            ",
        );
        assert!(res.is_err());
    }
    #[test]
    fn function_use_outside_variable_should_work() {
        let res = run_code(
            "   
                gimme c = 5;
                gimme a = (a) => {
                    c;
                };
                c;
                a(2);
            ",
        );
        assert!(res.is_ok());
    }
    #[test]
    fn function_use_in_expr_should_work() {
        let res = run_code(
            "   
                gimme a = (a) => {};
                2 + a(2);
            ",
        );
        assert!(res.is_ok());
    }
}
