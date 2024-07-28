use crate::environment::environment::Environment;

use super::{expression::Expression, primitive::Primitive, scope::Scope};

pub struct Func<'a> {
    argument: Vec<&'a str>,
    environment: Environment<'a>,
    scope: Scope<'a>,
    rt_val: Primitive,
}
