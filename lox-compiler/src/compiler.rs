use std::collections::HashMap;

use lalrpop_util::lexer::Token;
use lalrpop_util::ParseError;
use lox_generated::ast::Assignment;
use lox_generated::ast::Call;
use lox_generated::ast::Comparison;
use lox_generated::ast::ComparisonOperator;
use lox_generated::ast::Declaration;
use lox_generated::ast::Equality;
use lox_generated::ast::EqualityOperator;
use lox_generated::ast::Factor;
use lox_generated::ast::FactorOperator;
use lox_generated::ast::LogicAnd;
use lox_generated::ast::LogicOr;
use lox_generated::ast::Primary;
use lox_generated::ast::Program;
use lox_generated::ast::Term;
use lox_generated::ast::TermOperator;
use lox_generated::ast::Unary;
use lox_generated::ast::UnaryOperator;
use lox_generated::lox::ProgramParser;
use types::Type;

use self::errors::RuntimeError;

pub(crate) mod errors;
mod types;

#[derive(Debug)]
struct Symbol {
    name: String,
    value: Type,
}

pub struct Compiler<'input> {
    program: Program<'input>,
    stack: Vec<Type>,
    symbols: HashMap<String, Symbol>,
}

impl<'input> Compiler<'input> {
    pub fn new(input: &'input str) -> Result<Self, ParseError<usize, Token<'input>, &'static str>> {
        Ok(Self {
            program: ProgramParser::new().parse(input)?,
            stack: vec![],
            symbols: HashMap::new(),
        })
    }

    pub fn execute(&mut self) -> Result<(), RuntimeError> {
        self.execute_program(&self.program)
    }

    fn execute_program(&mut self, program: &Program<'input>) -> Result<(), RuntimeError> {
        self.execute_declarations(&program.0)
    }

    fn execute_declarations(&mut self, decls: &[Declaration<'input>]) -> Result<(), RuntimeError> {
        decls
            .into_iter()
            .map(|decl| self.execute_declaration(decl))
            .collect::<Result<Vec<_>, _>>()
            .map(|_| ())
    }

    fn execute_declaration(&mut self, decl: &Declaration<'input>) -> Result<(), RuntimeError> {
        match decl {
            Declaration::Class(class_decl) => Self::execute_class_declaration(class_decl),
            Declaration::Function(fun_decl) => self.execute_function_declaration(fun_decl),
            Declaration::Variable(_) => {}
            Declaration::Statement(_) => {}
        }
    }

    fn check_integer_value(value: Type) -> Result<i32, RuntimeError> {
        match value {
            Type::Integer(i) => Ok(i),
            _ => Err(RuntimeError::InvalidType(value.clone())),
        }
    }

    fn check_boolean_value(value: Type) -> Result<bool, RuntimeError> {
        match value {
            Type::Boolean(b) => Ok(b),
            _ => Err(RuntimeError::InvalidType(value.clone())),
        }
    }

    fn evaluate_assignment(
        &mut self,
        assignment: &Assignment<'input>,
    ) -> Result<Type, RuntimeError> {
        match assignment {
            Assignment::Assignment {
                object,
                target,
                value,
            } => {
                let object = self.evaluate_call(object)?;
            }
            Assignment::LogicOr(_) => {}
        }
    }

    fn evaluate_logic_or(&mut self, logic_or: &LogicOr<'input>) -> Result<Type, RuntimeError> {
        let mut result = self.evaluate_logic_and(&logic_or.left)?;
        for right in &logic_or.rest {
            let right = self.evaluate_logic_and(right)?;
            let b = Self::check_boolean_value(result)?;
            let c = Self::check_boolean_value(right).unwrap();
            result = Type::Boolean(b || c);
        }
        Ok(result)
    }

    fn evaluate_logic_and(&mut self, logic_and: &LogicAnd<'input>) -> Result<Type, RuntimeError> {
        let mut result = self.evaluate_equality(&logic_and.left)?;
        for right in &logic_and.rest {
            let right = self.evaluate_equality(right)?;
            let b = Self::check_boolean_value(result)?;
            let c = Self::check_boolean_value(right)?;
            result = Type::Boolean(b && c);
        }
        Ok(result)
    }

    fn evaluate_equality(&mut self, equality: &Equality<'input>) -> Result<Type, RuntimeError> {
        let mut result = self.evaluate_comparison(&equality.left)?;
        for (op, right) in &equality.rest {
            let right = self.evaluate_comparison(right)?;
            result = match op {
                EqualityOperator::Neq => Type::Boolean(result != right),
                EqualityOperator::Eq => Type::Boolean(result == right),
            }
        }
        Ok(result)
    }

    fn evaluate_comparison(
        &mut self,
        comparison: &Comparison<'input>,
    ) -> Result<Type, RuntimeError> {
        let mut result = self.evaluate_term(&comparison.left)?;
        for (op, right) in &comparison.rest {
            let right = self.evaluate_term(right)?;
            result = match op {
                ComparisonOperator::Gt => {
                    let i = Self::check_integer_value(result)?;
                    let j = Self::check_integer_value(right)?;
                    Type::Boolean(i > j)
                }
                ComparisonOperator::Ge => {
                    let i = Self::check_integer_value(result)?;
                    let j = Self::check_integer_value(right)?;
                    Type::Boolean(i >= j)
                }
                ComparisonOperator::Lt => {
                    let i = Self::check_integer_value(result)?;
                    let j = Self::check_integer_value(right)?;
                    Type::Boolean(i < j)
                }
                ComparisonOperator::Le => {
                    let i = Self::check_integer_value(result)?;
                    let j = Self::check_integer_value(right)?;
                    Type::Boolean(i <= j)
                }
            };
        }
        Ok(result)
    }

    fn evaluate_term(&mut self, term: &Term<'input>) -> Result<Type, RuntimeError> {
        let mut result = self.evaluate_factor(&term.left)?;
        for (op, right) in &term.rest {
            let right = self.evaluate_factor(right)?;
            result = match op {
                TermOperator::Minus => {
                    let i = Self::check_integer_value(result)?;
                    let j = Self::check_integer_value(right)?;
                    Type::Integer(i - j)
                }
                TermOperator::Plus => {
                    let i = Self::check_integer_value(result)?;
                    let j = Self::check_integer_value(right)?;
                    Type::Integer(i + j)
                }
            };
        }
        Ok(result)
    }

    fn evaluate_factor(&mut self, factor: &Factor<'input>) -> Result<Type, RuntimeError> {
        let mut result = self.evaluate_unary(&factor.left)?;
        for (op, right) in &factor.rest {
            let right = self.evaluate_unary(right)?;
            result = match op {
                FactorOperator::Slash => {
                    let i = Self::check_integer_value(result)?;
                    let j = Self::check_integer_value(right)?;
                    Type::Integer(i / j)
                }
                FactorOperator::Star => {
                    let i = Self::check_integer_value(result)?;
                    let j = Self::check_integer_value(right)?;
                    Type::Integer(i * j)
                }
            };
        }
        Ok(result)
    }

    fn evaluate_unary(&mut self, unary: &Unary<'input>) -> Result<Type, RuntimeError> {
        match unary {
            Unary::Unary { operator, right } => {
                let right = self.evaluate_unary(right)?;
                let t = match operator {
                    UnaryOperator::LogicNot => match right {
                        Type::Boolean(b) => Type::Boolean(!b),
                        _ => {
                            return Err(RuntimeError::MismatchedUnaryOperator(
                                *operator,
                                right.clone(),
                            ))
                        }
                    },
                    UnaryOperator::Neg => match right {
                        Type::Integer(i) => Type::Integer(-i),
                        _ => {
                            return Err(RuntimeError::MismatchedUnaryOperator(
                                *operator,
                                right.clone(),
                            ))
                        }
                    },
                };
                Ok(t)
            }
            Unary::Call(c) => self.evaluate_call(c),
        }
    }

    fn evaluate_call(&mut self, call: &Call<'input>) -> Result<Type, RuntimeError> {
        match &call.target {
            Primary::Identifier(i) => {
                let symbol = self
                    .symbols
                    .get(*i)
                    .ok_or_else(|| RuntimeError::UndefinedSymbol(i.to_string()))?;
                match symbol {
                    _ => return Err(RuntimeError::BadCallTarget(symbol.clone())),
                }
                Ok(symbol.value.clone())
            }
            p => {
                if !call.rhs.is_empty() {
                    return Err(RuntimeError::BadCallTarget(self.evaluate_primary(p)?));
                }

                self.evaluate_primary(p)
            }
        }
    }

    fn evaluate_primary(&mut self, primary: &Primary<'input>) -> Result<Type, RuntimeError> {
        let t = match primary {
            Primary::True => Type::Boolean(true),
            Primary::False => Type::Boolean(false),
            Primary::Nil => Type::Nil,
            Primary::This => todo!(),
            Primary::Number(n) => Type::Integer(
                n.parse()
                    .map_err(|_| RuntimeError::BadIntegerLiteral(n.to_string()))?,
            ),
            Primary::String(s) => Type::String(s.to_string()),
            Primary::Identifier(i) => todo!(),
            Primary::Parenthesized(expr) => self.evaluate_expression(expr),
            Primary::SuperMember(m) => todo!(),
        };

        Ok(t)
    }
}
