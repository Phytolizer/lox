#[derive(Debug, PartialEq)]
pub struct Program<'input>(pub Vec<Declaration<'input>>);

#[derive(Debug, PartialEq)]
pub enum Declaration<'input> {
    Class(ClassDeclaration<'input>),
    Function(FunctionDeclaration<'input>),
    Variable(VariableDeclaration<'input>),
    Statement(Statement<'input>),
}

#[derive(Debug, PartialEq)]
pub struct ClassDeclaration<'input> {
    pub class_name: &'input str,
    pub base: Option<&'input str>,
    pub members: Vec<Function<'input>>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration<'input>(pub Function<'input>);

#[derive(Debug, PartialEq)]
pub struct VariableDeclaration<'input> {
    pub name: &'input str,
    pub initializer: Option<Expression<'input>>,
}

#[derive(Debug, PartialEq)]
pub enum Statement<'input> {
    Expression(ExprStatement<'input>),
    For(ForStatement<'input>),
    If(IfStatement<'input>),
    Print(PrintStatement<'input>),
    Return(ReturnStatement<'input>),
    While(WhileStatement<'input>),
    Block(Block<'input>),
}

#[derive(Debug, PartialEq)]
pub struct ExprStatement<'input>(pub Expression<'input>);

#[derive(Debug, PartialEq)]
pub struct ForStatement<'input> {
    pub initializer: ForInitializer<'input>,
    pub condition: Option<Expression<'input>>,
    pub increment: Option<Expression<'input>>,
    pub body: Box<Statement<'input>>,
}

#[derive(Debug, PartialEq)]
pub enum ForInitializer<'input> {
    Declaration(VariableDeclaration<'input>),
    Expression(Box<ExprStatement<'input>>),
    Semicolon,
}

#[derive(Debug, PartialEq)]
pub struct IfStatement<'input> {
    pub condition: Expression<'input>,
    pub body: Box<Statement<'input>>,
}

#[derive(Debug, PartialEq)]
pub struct PrintStatement<'input>(pub Expression<'input>);

#[derive(Debug, PartialEq)]
pub struct ReturnStatement<'input>(pub Option<Expression<'input>>);

#[derive(Debug, PartialEq)]
pub struct WhileStatement<'input> {
    pub condition: Expression<'input>,
    pub body: Box<Statement<'input>>,
}

#[derive(Debug, PartialEq)]
pub struct Block<'input>(pub Vec<Declaration<'input>>);

#[derive(Debug, PartialEq)]
pub enum Expression<'input> {
    Assignment(Assignment<'input>),
}

#[derive(Debug, PartialEq)]
pub enum Assignment<'input> {
    Assignment {
        object: Option<Call<'input>>,
        target: &'input str,
        value: Box<Assignment<'input>>,
    },
    LogicOr(LogicOr<'input>),
}

#[derive(Debug, PartialEq)]
pub struct LogicOr<'input> {
    pub left: LogicAnd<'input>,
    pub rest: Vec<LogicAnd<'input>>,
}

#[derive(Debug, PartialEq)]
pub struct LogicAnd<'input> {
    pub left: Equality<'input>,
    pub rest: Vec<Equality<'input>>,
}

#[derive(Debug, PartialEq)]
pub struct Equality<'input> {
    pub left: Comparison<'input>,
    pub rest: Vec<(EqualityOperator, Comparison<'input>)>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum EqualityOperator {
    Neq,
    Eq,
}

#[derive(Debug, PartialEq)]
pub struct Comparison<'input> {
    pub left: Term<'input>,
    pub rest: Vec<(ComparisonOperator, Term<'input>)>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ComparisonOperator {
    Gt,
    Ge,
    Lt,
    Le,
}

#[derive(Debug, PartialEq)]
pub struct Term<'input> {
    pub left: Factor<'input>,
    pub rest: Vec<(TermOperator, Factor<'input>)>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TermOperator {
    Minus,
    Plus,
}

#[derive(Debug, PartialEq)]
pub struct Factor<'input> {
    pub left: Unary<'input>,
    pub rest: Vec<(FactorOperator, Unary<'input>)>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FactorOperator {
    Slash,
    Star,
}

#[derive(Debug, PartialEq)]
pub enum Unary<'input> {
    Unary {
        operator: UnaryOperator,
        right: Box<Unary<'input>>,
    },
    Call(Call<'input>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnaryOperator {
    LogicNot,
    Neg,
}

#[derive(Debug, PartialEq)]
pub struct Call<'input> {
    pub target: Primary<'input>,
    pub rhs: Vec<CallRhs<'input>>,
}

#[derive(Debug, PartialEq)]
pub enum CallRhs<'input> {
    Call(Arguments<'input>),
    Member(&'input str),
}

#[derive(Debug, PartialEq)]
pub enum Primary<'input> {
    True,
    False,
    Nil,
    This,
    Number(&'input str),
    String(&'input str),
    Identifier(&'input str),
    Parenthesized(Box<Expression<'input>>),
    SuperMember(&'input str),
}

impl<'input> Clone for Primary<'input> {
    fn clone(&self) -> Self {
        match self {
            Primary::True => Primary::True,
            Primary::False => Primary::False,
            Primary::Nil => Primary::Nil,
            Primary::This => Primary::This,
            Primary::Number(n) => Primary::Number(n),
            Primary::String(s) => Primary::String(s),
            Primary::Identifier(i) => Primary::Identifier(i),
            Primary::Parenthesized(_) => panic!("cannot clone parenthesized expression!"),
            Primary::SuperMember(m) => Primary::SuperMember(m),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Function<'input> {
    pub name: &'input str,
    pub parameters: Option<Parameters<'input>>,
    pub body: Block<'input>,
}

#[derive(Debug, PartialEq)]
pub struct Parameters<'input> {
    pub first: &'input str,
    pub rest: Vec<&'input str>,
}

#[derive(Debug, PartialEq)]
pub struct Arguments<'input> {
    pub first: Box<Expression<'input>>,
    pub rest: Vec<Expression<'input>>,
}
