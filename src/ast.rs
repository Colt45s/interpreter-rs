pub struct Program {
    pub statements: Vec<Statement>,
}

type Ident = String;
pub enum Statement {
    Let(Ident, Expression)
}

enum Expression {

}