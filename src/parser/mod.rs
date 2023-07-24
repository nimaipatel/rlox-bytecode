use std::str;

mod parse_error;
mod tests;

use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::token::Token;
use crate::token_type::TokenType;

use self::parse_error::ParseError;

// program        → declaration* EOF ;

// statement      → exprStmt
//                | forStmt
//                | ifStmt
//                | printStmt
//                | printStmt
//                | whileStmt
//                | block ;

// returnStmt     → "return" expression? ";" ;

// forStmt        → "for" "(" ( varDecl | exprStmt | ";" )
//                  expression? ";"
//                  expression? ")" statement ;

// whileStmt      → "while" "(" expression ")" statement ;

// ifStmt         → "if" "(" expression ")" statement
//                ( "else" statement )? ;

// block          → "{" declaration* "}" ;

// declaration    → funDecl
//                | varDecl
//                | statement ;

// funDecl        → "fun" function ;
// function       → IDENTIFIER "(" parameters? ")" block ;
// parameters     → IDENTIFIER ( "," IDENTIFIER )* ;

// varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;

// exprStmt       → expression ";" ;
// printStmt      → "print" expression ";" ;

// expression     → assignment ;
// assignment     → IDENTIFIER "=" assignment
//                | logic_or ;
// logic_or       → logic_and ( "or" logic_and )* ;
// logic_and      → equality ( "and" equality )* ;

// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary | call ;
// call           → primary ( "(" arguments? ")" )* ;
//                | primary ;
// arguments      → expression ( "," expression )* ;
// primary        → "true" | "false" | "nil"
//                | NUMBER | STRING
//                | "(" expression ")"
//                | IDENTIFIER ;

pub fn parse<'a>(tokens: &'a [Token<'a>]) -> (Vec<Stmt<'a>>, Vec<ParseError>) {
    let mut statements = Vec::new();
    let mut errors = Vec::new();
    let mut pos: usize = 0;
    loop {
        if tokens[pos].token_type == TokenType::Eof {
            break;
        } else {
            match parse_declaration(tokens, pos) {
                Ok((statement, new_pos)) => {
                    pos = new_pos;
                    statements.push(statement);
                }
                Err(e) => {
                    errors.push(e);
                    pos = synchronize(tokens, pos);
                }
            }
        }
    }
    (statements, errors)
}

fn synchronize(tokens: &[Token<'_>], pos: usize) -> usize {
    let mut pos = pos + 1;
    loop {
        match tokens[pos].token_type {
            TokenType::Eof => break,
            TokenType::Class => break,
            TokenType::Fun => break,
            TokenType::Var => break,
            TokenType::For => break,
            TokenType::While => break,
            TokenType::Print => break,
            TokenType::Return => break,
            TokenType::Semicolon => {
                pos += 1;
                break;
            }
            _ => pos += 1,
        }
    }
    pos
}

fn parse_declaration<'a>(
    tokens: &'a [Token<'a>],
    pos: usize,
) -> Result<(Stmt<'a>, usize), ParseError> {
    match tokens[pos].token_type {
        TokenType::Fun => parse_function(tokens, pos + 1),
        TokenType::Var => parse_var_declaration(tokens, pos + 1),
        _ => parse_statement(tokens, pos),
    }
}

fn parse_function<'a>(
    tokens: &'a [Token<'a>],
    pos: usize,
) -> Result<(Stmt<'a>, usize), ParseError> {
    let (name, pos) = consume(tokens, pos, TokenType::Identifier)?;
    let (_, mut pos) = consume(tokens, pos, TokenType::LeftParen)?;

    let mut params = Vec::new();
    if tokens[pos].token_type != TokenType::RightParen {
        loop {
            let (param, new_pos) = consume(tokens, pos, TokenType::Identifier)?;
            pos = new_pos;
            params.push(param);
            if tokens[pos].token_type == TokenType::Comma {
                pos += 1;
            } else {
                break;
            }
        }
    }
    let (_, pos) = consume(tokens, pos, TokenType::RightParen)?;

    let (_, pos) = consume(tokens, pos, TokenType::LeftBrace)?;
    let (body, pos) = parse_block(tokens, pos)?;

    Ok((
        Stmt::Function {
            name,
            params,
            body: Box::new(body),
        },
        pos,
    ))
}

fn parse_var_declaration<'a>(
    tokens: &'a [Token<'a>],
    pos: usize,
) -> Result<(Stmt<'a>, usize), ParseError> {
    let (name, pos) = consume(tokens, pos, TokenType::Identifier)?;

    match matchh(tokens, pos, vec![TokenType::Equal]) {
        Some((_, pos)) => {
            let (initializer, pos) = parse_expression(tokens, pos)?;
            let (_, pos) = consume(tokens, pos, TokenType::Semicolon)?;
            Ok((Stmt::Var(name, Some(initializer)), pos))
        }
        None => {
            let (_, pos) = consume(tokens, pos, TokenType::Semicolon)?;
            Ok((Stmt::Var(name, None), pos))
        }
    }
}

fn parse_statement<'a>(
    tokens: &'a [Token<'a>],
    pos: usize,
) -> Result<(Stmt<'a>, usize), ParseError> {
    match tokens[pos].token_type {
        TokenType::For => parse_for_statement(tokens, pos + 1),
        TokenType::If => parse_if_statment(tokens, pos + 1),
        TokenType::Print => parse_print_statement(tokens, pos + 1),
        TokenType::Return => parse_return_statement(tokens, pos),
        TokenType::While => parse_while_statement(tokens, pos + 1),
        TokenType::LeftBrace => parse_block(tokens, pos + 1),
        _ => parse_expression_statement(tokens, pos),
    }
}

fn parse_return_statement<'a>(
    tokens: &'a [Token<'a>],
    mut pos: usize,
) -> Result<(Stmt<'a>, usize), ParseError<'a>> {
    let keyword = &tokens[pos];
    pos += 1;
    let value = {
        match &tokens[pos].token_type {
            TokenType::Semicolon => None,
            _ => {
                let (value, new_pos) = parse_expression(tokens, pos)?;
                pos = new_pos;
                Some(value)
            }
        }
    };
    let (_, pos) = consume(tokens, pos, TokenType::Semicolon)?;
    Ok((Stmt::Return { keyword, value }, pos))
}

fn parse_for_statement<'a>(
    tokens: &'a [Token<'a>],
    pos: usize,
) -> Result<(Stmt<'a>, usize), ParseError<'a>> {
    let (_, mut pos) = consume(tokens, pos, TokenType::LeftParen)?;

    let initializer: Option<Stmt> = {
        match &tokens[pos].token_type {
            TokenType::Semicolon => None,
            TokenType::Var => {
                let (var_declaration, new_pos) = parse_var_declaration(tokens, pos + 1)?;
                pos = new_pos;
                Some(var_declaration)
            }
            _ => {
                let (expression_statement, new_pos) = parse_expression_statement(tokens, pos)?;
                pos = new_pos;
                Some(expression_statement)
            }
        }
    };

    let condition: Option<Expr> = {
        match &tokens[pos].token_type {
            TokenType::Semicolon => None,
            _ => {
                let (condition, new_pos) = parse_expression(tokens, pos)?;
                pos = new_pos;
                Some(condition)
            }
        }
    };
    let (_, mut pos) = consume(tokens, pos, TokenType::Semicolon)?;

    let increment: Option<Expr> = {
        match &tokens[pos].token_type {
            TokenType::RightParen => None,
            _ => {
                let (condition, new_pos) = parse_expression(tokens, pos)?;
                pos = new_pos;
                Some(condition)
            }
        }
    };
    let (_, pos) = consume(tokens, pos, TokenType::RightParen)?;

    let (mut body, pos) = parse_statement(tokens, pos)?;

    // we desugar the FOR loop to a WHILE loop

    // add the increment statement if any to the end of the body
    if let Some(increment) = increment {
        body = Stmt::Block(vec![body, Stmt::Expression(increment)]);
    }

    // create while loop based on condition
    if let Some(condition) = condition {
        body = Stmt::While {
            condition,
            body: Box::new(body),
        }
    } else {
        body = Stmt::While {
            condition: Expr::BoolLiteral(true),
            body: Box::new(body),
        }
    }

    // add the initializer statement to the beginning
    if let Some(initializer) = initializer {
        body = Stmt::Block(vec![initializer, body])
    }

    Ok((body, pos))
}

fn parse_while_statement<'a>(
    tokens: &'a [Token<'a>],
    pos: usize,
) -> Result<(Stmt<'a>, usize), ParseError<'a>> {
    let (_, pos) = consume(tokens, pos, TokenType::LeftParen)?;
    let (condition, pos) = parse_expression(tokens, pos)?;
    let (_, pos) = consume(tokens, pos, TokenType::RightParen)?;
    let (body, pos) = parse_statement(tokens, pos)?;
    Ok((
        Stmt::While {
            condition,
            body: Box::new(body),
        },
        pos,
    ))
}

fn parse_if_statment<'a>(
    tokens: &'a [Token<'a>],
    pos: usize,
) -> Result<(Stmt<'a>, usize), ParseError<'a>> {
    let (_, pos) = consume(tokens, pos, TokenType::LeftParen)?;
    let (condition, pos) = parse_expression(tokens, pos)?;
    let (_, pos) = consume(tokens, pos, TokenType::RightParen)?;
    let (then_branch, pos) = parse_statement(tokens, pos)?;
    match matchh(tokens, pos, vec![TokenType::Else]) {
        Some((_, pos)) => {
            let (else_branch, pos) = parse_statement(tokens, pos)?;
            Ok((
                Stmt::If {
                    condition,
                    then_branch: Box::new(then_branch),
                    else_branch: Some(Box::new(else_branch)),
                },
                pos,
            ))
        }
        None => Ok((
            Stmt::If {
                condition,
                then_branch: Box::new(then_branch),
                else_branch: None,
            },
            pos,
        )),
    }
}

fn parse_block<'a>(tokens: &'a [Token<'a>], pos: usize) -> Result<(Stmt<'a>, usize), ParseError> {
    let mut pos = pos;
    let mut statements = Vec::new();

    loop {
        match tokens[pos].token_type {
            TokenType::Eof | TokenType::RightBrace => {
                let (_, pos) = consume(tokens, pos, TokenType::RightBrace)?;
                return Ok((Stmt::Block(statements), pos));
            }
            _ => {
                let (statement, new_pos) = parse_declaration(tokens, pos)?;
                pos = new_pos;
                statements.push(statement);
            }
        }
    }
}

fn parse_expression_statement<'a>(
    tokens: &'a [Token<'a>],
    pos: usize,
) -> Result<(Stmt<'a>, usize), ParseError> {
    let (expr, pos) = parse_expression(tokens, pos)?;
    let (_, pos) = consume(tokens, pos, TokenType::Semicolon)?;
    return Ok((Stmt::Expression(expr), pos));
}

fn parse_print_statement<'a>(
    tokens: &'a [Token<'a>],
    pos: usize,
) -> Result<(Stmt<'a>, usize), ParseError> {
    let (value, pos) = parse_expression(tokens, pos)?;
    let (_, pos) = consume(tokens, pos, TokenType::Semicolon)?;
    return Ok((Stmt::Print(value), pos));
}

// TODO: replace all instances of the consuming pattern with this function
fn consume<'a>(
    tokens: &'a [Token<'a>],
    pos: usize,
    expected: TokenType,
) -> Result<(&'a Token<'a>, usize), ParseError<'a>> {
    if tokens[pos].token_type == expected {
        Ok((&tokens[pos], pos + 1))
    } else {
        Err(ParseError::ExpectedSomething {
            actual: &tokens[pos],
            expected,
        })
    }
}

fn matchh<'a>(
    tokens: &'a [Token<'a>],
    pos: usize,
    expected: Vec<TokenType>,
) -> Option<(&'a Token<'a>, usize)> {
    for expected in expected.iter() {
        if tokens[pos].token_type == *expected {
            return Some((&tokens[pos], pos + 1));
        }
    }
    None
}

pub fn parse_expression<'a>(
    tokens: &'a [Token<'a>],
    pos: usize,
) -> Result<(Expr<'a>, usize), ParseError> {
    parse_assignment(tokens, pos)
}

fn parse_assignment<'a>(
    tokens: &'a [Token<'a>],
    pos: usize,
) -> Result<(Expr<'a>, usize), ParseError> {
    let (expr, pos) = parse_or(tokens, pos)?;
    match matchh(tokens, pos, vec![TokenType::Equal]) {
        Some((equals, pos)) => {
            let (value, pos) = parse_assignment(tokens, pos)?;
            match expr {
                Expr::Variable(name) => Ok((
                    Expr::Assign {
                        name,
                        value: Box::new(value),
                    },
                    pos,
                )),
                _ => Err(ParseError::InvalidAssignment { equals }),
            }
        }
        None => Ok((expr, pos)),
    }
}

fn parse_or<'a>(tokens: &'a [Token<'a>], pos: usize) -> Result<(Expr<'a>, usize), ParseError<'a>> {
    let (mut expr, mut pos) = parse_and(tokens, pos)?;
    loop {
        match matchh(tokens, pos, vec![TokenType::Or]) {
            None => break,
            Some((tok_or, new_pos)) => {
                let (right, new_pos) = parse_and(tokens, new_pos)?;
                pos = new_pos;
                expr = Expr::Logical {
                    left: Box::new(expr),
                    op: tok_or,
                    right: Box::new(right),
                }
            }
        }
    }
    Ok((expr, pos))
}

fn parse_and<'a>(tokens: &'a [Token<'a>], pos: usize) -> Result<(Expr<'a>, usize), ParseError<'a>> {
    let (mut expr, mut pos) = parse_equality(tokens, pos)?;
    loop {
        match matchh(tokens, pos, vec![TokenType::And]) {
            None => break,
            Some((tok_or, new_pos)) => {
                let (right, new_pos) = parse_equality(tokens, new_pos)?;
                pos = new_pos;
                expr = Expr::Logical {
                    left: Box::new(expr),
                    op: tok_or,
                    right: Box::new(right),
                }
            }
        }
    }
    Ok((expr, pos))
}

fn parse_equality<'a>(
    tokens: &'a [Token<'a>],
    pos: usize,
) -> Result<(Expr<'a>, usize), ParseError> {
    let (mut expr, mut pos) = parse_comp(tokens, pos)?;
    loop {
        let comp_token = tokens.get(pos).ok_or(ParseError::UnexpectedEndOfInput {
            expected: "equality",
        })?;
        if comp_token.token_type != TokenType::EqualEqual
            && comp_token.token_type != TokenType::BangEqual
        {
            break;
        }
        let (right, new_pos) = parse_comp(tokens, pos + 1)?;
        pos = new_pos;
        expr = Expr::Binary {
            left: Box::new(expr),
            op: comp_token,
            right: Box::new(right),
        };
    }
    Ok((expr, pos))
}

fn parse_comp<'a>(tokens: &'a [Token<'a>], pos: usize) -> Result<(Expr<'a>, usize), ParseError> {
    let (mut expr, mut pos) = parse_term(tokens, pos)?;
    loop {
        let comp_token = tokens.get(pos).ok_or(ParseError::UnexpectedEndOfInput {
            expected: "comparision",
        })?;
        if comp_token.token_type != TokenType::Greater
            && comp_token.token_type != TokenType::GreaterEqual
            && comp_token.token_type != TokenType::Less
            && comp_token.token_type != TokenType::LessEqual
        {
            break;
        }
        let (right, new_pos) = parse_term(tokens, pos + 1)?;
        pos = new_pos;
        expr = Expr::Binary {
            left: Box::new(expr),
            op: comp_token,
            right: Box::new(right),
        };
    }
    Ok((expr, pos))
}

fn parse_term<'a>(tokens: &'a [Token<'a>], pos: usize) -> Result<(Expr<'a>, usize), ParseError> {
    let (mut expr, mut pos) = parse_factor(tokens, pos)?;
    loop {
        let op_token = tokens.get(pos).ok_or(ParseError::UnexpectedEndOfInput {
            expected: "term operator",
        })?;
        if op_token.token_type != TokenType::Plus && op_token.token_type != TokenType::Minus {
            break;
        }
        let (right, new_pos) = parse_factor(tokens, pos + 1)?;
        pos = new_pos;
        expr = Expr::Binary {
            left: Box::new(expr),
            op: op_token,
            right: Box::new(right),
        }
    }
    Ok((expr, pos))
}

fn parse_factor<'a>(tokens: &'a [Token<'a>], pos: usize) -> Result<(Expr<'a>, usize), ParseError> {
    let (mut expr, mut pos) = parse_unary(tokens, pos)?;
    loop {
        let op_token = tokens
            .get(pos)
            .ok_or(ParseError::UnexpectedEndOfInput { expected: "factor" })?;
        if op_token.token_type != TokenType::Star && op_token.token_type != TokenType::Slash {
            break;
        }
        let (right, new_pos) = parse_unary(tokens, pos + 1)?;
        pos = new_pos;
        expr = Expr::Binary {
            left: Box::new(expr),
            op: op_token,
            right: Box::new(right),
        }
    }
    Ok((expr, pos))
}

fn parse_unary<'a>(tokens: &'a [Token<'a>], pos: usize) -> Result<(Expr<'a>, usize), ParseError> {
    let operator_token = tokens.get(pos).ok_or(ParseError::UnexpectedEndOfInput {
        expected: "unary operator",
    })?;
    match &operator_token.token_type {
        TokenType::Bang | TokenType::Minus => {
            let (right, pos) = parse_unary(tokens, pos + 1)?;
            Ok((
                Expr::Unary {
                    op: operator_token,
                    expr: Box::new(right),
                },
                pos,
            ))
        }
        _ => parse_call(tokens, pos),
    }
}

fn parse_call<'a>(tokens: &'a [Token<'a>], pos: usize) -> Result<(Expr<'a>, usize), ParseError> {
    let (mut expr, mut pos) = parse_primary(tokens, pos)?;
    loop {
        match &tokens[pos].token_type {
            TokenType::LeftParen => {
                let (new_expr, new_pos) = parse_call_finish(tokens, pos + 1, expr)?;
                expr = new_expr;
                pos = new_pos;
            }
            _ => break,
        }
    }
    Ok((expr, pos))
}

fn parse_call_finish<'a>(
    tokens: &'a [Token<'a>],
    pos: usize,
    callee: Expr<'a>,
) -> Result<(Expr<'a>, usize), ParseError<'a>> {
    let mut pos = pos;
    let mut arguments = Vec::new();
    if tokens[pos].token_type != TokenType::RightParen {
        let (argument, new_pos) = parse_expression(tokens, pos)?;
        arguments.push(argument);
        pos = new_pos;
        loop {
            if tokens[pos].token_type == TokenType::Comma {
                let (argument, new_pos) = parse_expression(tokens, pos + 1)?;
                arguments.push(argument);
                pos = new_pos;
            } else {
                break;
            }
        }
    }

    let (paren, pos) = consume(tokens, pos, TokenType::RightParen)?;

    Ok((
        Expr::Call {
            callee: Box::new(callee),
            paren,
            arguments,
        },
        pos,
    ))
}

fn parse_primary<'a>(tokens: &'a [Token<'a>], pos: usize) -> Result<(Expr<'a>, usize), ParseError> {
    let token = tokens.get(pos).ok_or(ParseError::UnexpectedEndOfInput {
        expected: "literal",
    })?;
    match &token.token_type {
        TokenType::False => Ok((Expr::BoolLiteral(false), pos + 1)),
        TokenType::True => Ok((Expr::BoolLiteral(true), pos + 1)),
        TokenType::Nil => Ok((Expr::NilLiteral, pos + 1)),

        TokenType::Number => Ok((
            Expr::NumericLiteral(str::from_utf8(token.lexeme).unwrap().parse().unwrap()),
            pos + 1,
        )),
        TokenType::String => Ok((Expr::StringLiteral(token.lexeme), pos + 1)),

        TokenType::LeftParen => {
            let (expr, pos) = parse_expression(tokens, pos + 1)?;
            let (_, pos) = consume(tokens, pos, TokenType::RightParen)?;
            Ok(((Expr::Grouping(Box::new(expr))), pos))
        }

        TokenType::Identifier => Ok((Expr::Variable(token), pos + 1)),

        _ => Err(ParseError::InvalidToken { token }),
    }
}
