pub mod ast;
pub mod lox;

#[cfg(test)]
mod tests {
    use crate::ast::*;
    use crate::lox::ProgramParser;
    #[test]
    fn empty_program() {
        let program = ProgramParser::new().parse("").unwrap();
        assert!(program.0.is_empty());
    }

    #[test]
    fn var_decl() {
        let program = ProgramParser::new().parse("var x;").unwrap();
        assert_eq!(
            program,
            Program(vec![Declaration::Variable(VariableDeclaration {
                name: "x",
                initializer: None
            })])
        )
    }

    #[test]
    fn var_decl_with_initializer() {
        let program = ProgramParser::new().parse("var x = 3;").unwrap();
        assert_eq!(
            program,
            Program(vec![Declaration::Variable(VariableDeclaration {
                name: "x",
                initializer: Some(Expression::Assignment(Assignment::LogicOr(LogicOr {
                    left: LogicAnd {
                        left: Equality {
                            left: Comparison {
                                left: Term {
                                    left: Factor {
                                        left: Unary::Call(Call {
                                            target: Primary::Number("3"),
                                            rhs: vec![]
                                        }),
                                        rest: vec![]
                                    },
                                    rest: vec![]
                                },
                                rest: vec![]
                            },
                            rest: vec![]
                        },
                        rest: vec![]
                    },
                    rest: vec![]
                },)))
            })])
        )
    }

    #[test]
    fn fun_decl() {
        let program = ProgramParser::new().parse("fun test() { }").unwrap();
        assert_eq!(
            program,
            Program(vec![Declaration::Function(FunctionDeclaration(Function {
                name: "test",
                parameters: None,
                body: Block(vec![]),
            }))])
        )
    }

    #[test]
    fn class_decl() {
        let program = ProgramParser::new().parse("class Test < Base { }").unwrap();
        assert_eq!(
            program,
            Program(vec![Declaration::Class(ClassDeclaration {
                class_name: "Test",
                base: Some("Base"),
                members: vec![]
            })])
        )
    }

    #[test]
    fn top_level_statement() {
        let program = ProgramParser::new().parse("3;").unwrap();
        assert_eq!(
            program,
            Program(vec![Declaration::Statement(Statement::Expression(
                ExprStatement(Expression::Assignment(Assignment::LogicOr(LogicOr {
                    left: LogicAnd {
                        left: Equality {
                            left: Comparison {
                                left: Term {
                                    left: Factor {
                                        left: Unary::Call(Call {
                                            target: Primary::Number("3"),
                                            rhs: vec![],
                                        }),
                                        rest: vec![]
                                    },
                                    rest: vec![]
                                },
                                rest: vec![]
                            },
                            rest: vec![]
                        },
                        rest: vec![]
                    },
                    rest: vec![]
                })))
            ))])
        )
    }

    #[test]
    fn addition_expr() {
        let program = ProgramParser::new().parse("3 + 4;").unwrap();
        assert_eq!(
            program.0[0],
            Declaration::Statement(Statement::Expression(ExprStatement(
                Expression::Assignment(Assignment::LogicOr(LogicOr {
                    rest: vec![],
                    left: LogicAnd {
                        rest: vec![],
                        left: Equality {
                            rest: vec![],
                            left: Comparison {
                                rest: vec![],
                                left: Term {
                                    rest: vec![(
                                        TermOperator::Plus,
                                        Factor {
                                            rest: vec![],
                                            left: Unary::Call(Call {
                                                rhs: vec![],
                                                target: Primary::Number("4")
                                            })
                                        }
                                    )],
                                    left: Factor {
                                        rest: vec![],
                                        left: Unary::Call(Call {
                                            rhs: vec![],
                                            target: Primary::Number("3")
                                        })
                                    }
                                }
                            }
                        }
                    }
                }))
            )))
        )
    }
}
