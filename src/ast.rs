#[derive(Debug, PartialEq, Clone, Default)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Let(Identifier, Expression)
}

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier(pub String);


#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Ident(Identifier),
    Literal(Literal)
}

#[derive(PartialEq, Clone, Debug)]
pub enum Literal {
    Int(i32)
}
