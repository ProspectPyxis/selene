use super::*;
use std::convert::Infallible;

use full_moon::{
    ast::{self, Ast},
    node::Node,
    visitors::Visitor,
};

pub struct DivideByZeroLint;

impl Rule for DivideByZeroLint {
    type Config = ();
    type Error = Infallible;

    fn new(_: Self::Config) -> Result<Self, Self::Error> {
        Ok(DivideByZeroLint)
    }

    fn pass(&self, ast: &Ast, _: &Context) -> Vec<Diagnostic> {
        let mut visitor = DivideByZeroVisitor {
            positions: Vec::new(),
        };

        visitor.visit_ast(&ast);

        visitor
            .positions
            .iter()
            .map(|position| {
                Diagnostic::new(
                    "divide_by_zero",
                    "dividing by zero is not allowed, use math.huge instead".to_owned(),
                    Label::new(*position),
                )
            })
            .collect()
    }

    fn severity(&self) -> Severity {
        Severity::Warning
    }

    fn rule_type(&self) -> RuleType {
        RuleType::Complexity
    }
}

struct DivideByZeroVisitor {
    positions: Vec<(usize, usize)>,
}

fn value_is_zero(value: &ast::Value) -> bool {
    if let ast::Value::Number(token) = value {
        token.to_string() == "0"
    } else {
        false
    }
}

impl Visitor<'_> for DivideByZeroVisitor {
    fn visit_expression(&mut self, node: &ast::Expression) {
        if_chain::if_chain! {
            if let ast::Expression::Value { value, binop } = node;
            if let Some(rhs) = binop;
            if let ast::BinOp::Slash(_) = rhs.bin_op();
            if let ast::Expression::Value {
                value: rhs_value, ..
            } = rhs.rhs();
            if value_is_zero(rhs_value) && !value_is_zero(value);
            then {
                let range = node.range().unwrap();
                self.positions.push((range.0.bytes(), range.1.bytes()));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{super::test_util::test_lint, *};

    #[test]
    fn test_divide_by_zero() {
        test_lint(
            DivideByZeroLint::new(()).unwrap(),
            "divide_by_zero",
            "divide_by_zero",
        );
    }
}
