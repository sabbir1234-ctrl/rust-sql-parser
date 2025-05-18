use crate::statement::*;
use crate::token::{Keyword, Token};
use crate::tokenizer::Tokenizer;

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(mut tokenizer: Tokenizer<'a>) -> Self {
        let current_token = tokenizer.next_token();
        Parser {
            tokenizer,
            current_token,
        }
    }

    fn advance(&mut self) {
        self.current_token = self.tokenizer.next_token();
    }

    fn expect(&mut self, expected: Token) -> bool {
        if self.current_token == expected {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        match &self.current_token {
            Token::Keyword(Keyword::Select) => Some(Statement::Select(self.parse_select())),
            Token::Keyword(Keyword::Create) => Some(Statement::Create(self.parse_create_table())),
            _ => None,
        }
    }

    fn parse_select(&mut self) -> SelectStatement {
        self.advance(); // Skip SELECT

        let mut columns = Vec::new();
        loop {
            match &self.current_token {
                Token::Identifier(ident) => columns.push(ident.clone()),
                Token::Star => columns.push("*".to_string()),
                _ => break,
            }
            self.advance();

            if !self.expect(Token::Comma) {
                break;
            }
        }

        if !self.expect(Token::Keyword(Keyword::From)) {
            panic!("Expected FROM after SELECT columns");
        }

        let table = match &self.current_token {
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();
                name
            }
            _ => panic!("Expected table name after FROM"),
        };

        let mut where_clause = None;
        if self.current_token == Token::Keyword(Keyword::Where) {
            self.advance();
            where_clause = Some(self.parse_expression(0));
        }

        let mut order_by = None;
        if self.current_token == Token::Keyword(Keyword::Order) {
            self.advance();
            if !self.expect(Token::Keyword(Keyword::By)) {
                panic!("Expected BY after ORDER");
            }

            let column = match &self.current_token {
                Token::Identifier(col) => {
                    let col = col.clone();
                    self.advance();
                    col
                }
                _ => panic!("Expected column name in ORDER BY"),
            };

            let direction = match &self.current_token {
                Token::Keyword(Keyword::Asc) => {
                    self.advance();
                    OrderDirection::Asc
                }
                Token::Keyword(Keyword::Desc) => {
                    self.advance();
                    OrderDirection::Desc
                }
                _ => OrderDirection::Asc,
            };

            order_by = Some(OrderBy { column, direction });
        }

        SelectStatement {
            columns,
            table,
            where_clause,
            order_by,
        }
    }

    fn parse_create_table(&mut self) -> CreateTableStatement {
        self.advance(); // Skip CREATE

        if !self.expect(Token::Keyword(Keyword::Table)) {
            panic!("Expected TABLE after CREATE");
        }

        let table_name = match &self.current_token {
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();
                name
            }
            _ => panic!("Expected table name"),
        };

        if !self.expect(Token::LeftParentheses) {
            panic!("Expected '(' after table name");
        }

        let mut columns = Vec::new();

        loop {
            let name = match &self.current_token {
                Token::Identifier(name) => {
                    let name = name.clone();
                    self.advance();
                    name
                }
                _ => panic!("Expected column name"),
            };

            let data_type = match &self.current_token {
                Token::Keyword(Keyword::Int) => {
                    self.advance();
                    SqlType::Int
                }
                Token::Keyword(Keyword::Bool) => {
                    self.advance();
                    SqlType::Bool
                }
                Token::Keyword(Keyword::Varchar) => {
                    self.advance();
                    SqlType::Varchar
                }
                _ => panic!("Expected data type"),
            };

            let mut constraints = Vec::new();
            loop {
                match &self.current_token {
                    Token::Keyword(Keyword::Primary) => {
                        self.advance();
                        if !self.expect(Token::Keyword(Keyword::Key)) {
                            panic!("Expected KEY after PRIMARY");
                        }
                        constraints.push(ColumnConstraint::PrimaryKey);
                    }
                    Token::Keyword(Keyword::Not) => {
                        self.advance();
                        if !self.expect(Token::Keyword(Keyword::Null)) {
                            panic!("Expected NULL after NOT");
                        }
                        constraints.push(ColumnConstraint::NotNull);
                    }
                    Token::Keyword(Keyword::Null) => {
                        self.advance();
                        constraints.push(ColumnConstraint::Null);
                    }
                    Token::Keyword(Keyword::Check) => {
                        self.advance();
                        if !self.expect(Token::LeftParentheses) {
                            panic!("Expected '(' after CHECK");
                        }
                        let expr = self.parse_expression(0);
                        if !self.expect(Token::RightParentheses) {
                            panic!("Expected ')' after CHECK expression");
                        }
                        constraints.push(ColumnConstraint::Check(expr));
                    }
                    _ => break,
                }
            }

            columns.push(ColumnDefinition {
                name,
                data_type,
                constraints,
            });

            if !self.expect(Token::Comma) {
                break;
            }
        }

        if !self.expect(Token::RightParentheses) {
            panic!("Expected ')' at the end of CREATE TABLE");
        }

        CreateTableStatement {
            table_name,
            columns,
        }
    }

    fn parse_expression(&mut self, min_prec: u8) -> Expression {
        let mut left = match &self.current_token {
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Expression::Identifier(name)
            }
            Token::Number(n) => {
                let n = *n;
                self.advance();
                Expression::Literal(Literal::Number(n))
            }
            Token::String(s) => {
                let s = s.clone();
                self.advance();
                Expression::Literal(Literal::String(s))
            }
            Token::Keyword(Keyword::True) => {
                self.advance();
                Expression::Literal(Literal::Boolean(true))
            }
            Token::Keyword(Keyword::False) => {
                self.advance();
                Expression::Literal(Literal::Boolean(false))
            }
            Token::Keyword(Keyword::Null) => {
                self.advance();
                Expression::Literal(Literal::Null)
            }
            Token::Keyword(Keyword::Not) => {
                self.advance();
                let expr = self.parse_expression(10);
                Expression::UnaryOp {
                    op: UnaryOperator::Not,
                    expr: Box::new(expr),
                }
            }
            Token::Minus => {
                self.advance();
                let expr = self.parse_expression(10);
                Expression::UnaryOp {
                    op: UnaryOperator::Neg,
                    expr: Box::new(expr),
                }
            }
            Token::LeftParentheses => {
                self.advance();
                let expr = self.parse_expression(0);
                if !self.expect(Token::RightParentheses) {
                    panic!("Expected ')' after expression");
                }
                expr
            }
            _ => panic!("Unexpected token in expression: {:?}", self.current_token),
        };

        while let Some((op, prec)) = self.get_infix_op() {
            if prec < min_prec {
                break;
            }
            self.advance();
            let right = self.parse_expression(prec + 1);
            left = Expression::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        left
    }

    fn get_infix_op(&self) -> Option<(BinaryOperator, u8)> {
        match &self.current_token {
            Token::Equal => Some((BinaryOperator::Eq, 4)),
            Token::NotEqual => Some((BinaryOperator::NotEq, 4)),
            Token::LessThan => Some((BinaryOperator::Lt, 4)),
            Token::LessThanOrEqual => Some((BinaryOperator::Lte, 4)),
            Token::GreaterThan => Some((BinaryOperator::Gt, 4)),
            Token::GreaterThanOrEqual => Some((BinaryOperator::Gte, 4)),
            Token::Plus => Some((BinaryOperator::Add, 5)),
            Token::Minus => Some((BinaryOperator::Sub, 5)),
            Token::Star => Some((BinaryOperator::Mul, 6)),
            Token::Divide => Some((BinaryOperator::Div, 6)),
            Token::Keyword(Keyword::And) => Some((BinaryOperator::And, 2)),
            Token::Keyword(Keyword::Or) => Some((BinaryOperator::Or, 1)),
            _ => None,
        }
    }
}
