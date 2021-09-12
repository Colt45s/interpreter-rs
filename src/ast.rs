#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Let(Ident, Expression)
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ident(pub String);


#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Ident(Ident),
    Literal(Literal)
}

#[derive(PartialEq, Clone, Debug)]
pub enum Literal {
    Int(i32)
}
