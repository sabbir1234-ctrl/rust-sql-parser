#[derive(Debug)]
pub enum Statement {
    Select(SelectStatement),
    Create(CreateTableStatement),
}

#[derive(Debug)]
pub struct SelectStatement {
    pub columns: Vec<String>,
    pub table: String,
    pub where_clause: Option<Expression>,
    pub order_by: Option<OrderBy>,
}

#[derive(Debug)]
pub struct CreateTableStatement {
    pub table_name: String,
    pub columns: Vec<ColumnDefinition>,
}

#[derive(Debug)]
pub struct ColumnDefinition {
    pub name: String,
    pub data_type: SqlType,
    pub constraints: Vec<ColumnConstraint>,
}

#[derive(Debug)]
pub enum SqlType {
    Int,
    Bool,
    Varchar,
}

#[derive(Debug)]
pub enum ColumnConstraint {
    PrimaryKey,
    NotNull,
    Null,
    Check(Expression),
}

#[derive(Debug)]
pub struct OrderBy {
    pub column: String,
    pub direction: OrderDirection,
}

#[derive(Debug)]
pub enum OrderDirection {
    Asc,
    Desc,
}

#[derive(Debug)]
pub enum Expression {
    BinaryOp {
        left: Box<Expression>,
        op: BinaryOperator,
        right: Box<Expression>,
    },
    UnaryOp {
        op: UnaryOperator,
        expr: Box<Expression>,
    },
    Identifier(String),
    Literal(Literal),
}

#[derive(Debug)]
pub enum Literal {
    Number(u64),
    String(String),
    Boolean(bool),
    Null,
}

#[derive(Debug)]
pub enum BinaryOperator {
    And,
    Or,
    Eq,
    NotEq,
    Gt,
    Gte,
    Lt,
    Lte,
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum UnaryOperator {
    Not,
    Neg,
}

