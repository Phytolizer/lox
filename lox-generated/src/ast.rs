#[derive(Debug, PartialEq)]
pub struct Program<'input>(pub(crate) Vec<Declaration<'input>>);

#[derive(Debug, PartialEq)]
pub(crate) enum Declaration<'input> {
    Class(ClassDeclaration<'input>),
    Function(FunctionDeclaration<'input>),
    Variable(VariableDeclaration<'input>),
    Statement(Statement<'input>),
}

#[derive(Debug, PartialEq)]
pub(crate) struct ClassDeclaration<'input> {
    pub(crate) class_name: &'input str,
    pub(crate) base: Option<&'input str>,
    pub(crate) members: Vec<Function<'input>>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct FunctionDeclaration<'input>(pub(crate) Function<'input>);

#[derive(Debug, PartialEq)]
pub(crate) struct VariableDeclaration<'input> {
    pub(crate) name: &'input str,
    pub(crate) initializer: Option<Expression<'input>>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Statement<'input> {
    Expression(ExprStatement<'input>),
    For(ForStatement<'input>),
    If(IfStatement<'input>),
    Print(PrintStatement<'input>),
    Return(ReturnStatement<'input>),
    While(WhileStatement<'input>),
    Block(Block<'input>),
}

#[derive(Debug, PartialEq)]
pub(crate) struct ExprStatement<'input>(pub(crate) Expression<'input>);

#[derive(Debug, PartialEq)]
pub(crate) struct ForStatement<'input> {
    pub(crate) initializer: ForInitializer<'input>,
    pub(crate) condition: Option<Expression<'input>>,
    pub(crate) increment: Option<Expression<'input>>,
    pub(crate) body: Box<Statement<'input>>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum ForInitializer<'input> {
    Declaration(VariableDeclaration<'input>),
    Expression(Box<ExprStatement<'input>>),
    Semicolon,
}

#[derive(Debug, PartialEq)]
pub(crate) struct IfStatement<'input> {
    pub(crate) condition: Expression<'input>,
    pub(crate) body: Box<Statement<'input>>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct PrintStatement<'input>(pub(crate) Expression<'input>);

#[derive(Debug, PartialEq)]
pub(crate) struct ReturnStatement<'input>(pub(crate) Option<Expression<'input>>);

#[derive(Debug, PartialEq)]
pub(crate) struct WhileStatement<'input> {
    pub(crate) condition: Expression<'input>,
    pub(crate) body: Box<Statement<'input>>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Block<'input>(pub(crate) Vec<Declaration<'input>>);

#[derive(Debug, PartialEq)]
pub(crate) enum Expression<'input> {
    Assignment(Assignment<'input>),
}

#[derive(Debug, PartialEq)]
pub(crate) enum Assignment<'input> {
    Assignment {
        object: Option<Call<'input>>,
        target: &'input str,
        value: Box<Assignment<'input>>,
    },
    LogicOr(LogicOr<'input>),
}

#[derive(Debug, PartialEq)]
pub(crate) struct LogicOr<'input> {
    pub(crate) left: LogicAnd<'input>,
    pub(crate) rest: Vec<LogicAnd<'input>>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct LogicAnd<'input> {
    pub(crate) left: Equality<'input>,
    pub(crate) rest: Vec<Equality<'input>>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Equality<'input> {
    pub(crate) left: Comparison<'input>,
    pub(crate) rest: Vec<(EqualityOperator, Comparison<'input>)>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum EqualityOperator {
    Neq,
    Eq,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Comparison<'input> {
    pub(crate) left: Term<'input>,
    pub(crate) rest: Vec<(ComparisonOperator, Term<'input>)>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum ComparisonOperator {
    Gt,
    Ge,
    Lt,
    Le,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Term<'input> {
    pub(crate) left: Factor<'input>,
    pub(crate) rest: Vec<(TermOperator, Factor<'input>)>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum TermOperator {
    Minus,
    Plus,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Factor<'input> {
    pub(crate) left: Unary<'input>,
    pub(crate) rest: Vec<(FactorOperator, Unary<'input>)>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum FactorOperator {
    Slash,
    Star,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Unary<'input> {
    Unary {
        operator: UnaryOperator,
        right: Box<Unary<'input>>,
    },
    Call(Call<'input>),
}

#[derive(Debug, PartialEq)]
pub(crate) enum UnaryOperator {
    LogicNot,
    Neg,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Call<'input> {
    pub(crate) target: Primary<'input>,
    pub(crate) rhs: Vec<CallRhs<'input>>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum CallRhs<'input> {
    Call(Arguments<'input>),
    Member(&'input str),
}

#[derive(Debug, PartialEq)]
pub(crate) enum Primary<'input> {
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

#[derive(Debug, PartialEq)]
pub(crate) struct Function<'input> {
    pub(crate) name: &'input str,
    pub(crate) parameters: Option<Parameters<'input>>,
    pub(crate) body: Block<'input>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Parameters<'input> {
    pub(crate) first: &'input str,
    pub(crate) rest: Vec<&'input str>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Arguments<'input> {
    pub(crate) first: Box<Expression<'input>>,
    pub(crate) rest: Vec<Expression<'input>>,
}
