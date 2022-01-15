use super::Error;
use super::{
	BinaryOperator, Expression, ExpressionNode, OperatorTokenType, Statement, TernaryOperator,
	UnaryOperator,
};
use super::{Literal, Token, TokenType};

type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
	MissingExpr(Token),
	MissingVariableName(Token),
}

impl ParseError {
	fn print(&self) {
		let (token, message) = match self {
			ParseError::MissingExpr(token) => (token, String::from("Expect expression")),
			ParseError::MissingVariableName(token) => (token, String::from("Expect variable name")),
		};
		eprintln!(
			"[line {}] Error at {}: {}.",
			token.line, token.lexeme, message
		);
	}
}

pub struct Parser {
	pub tokens: Vec<Token>,
	current_token: usize,
}

impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		Self {
			tokens,
			current_token: 0,
		}
	}
	pub fn parse(&mut self) -> Vec<Statement> {
		let mut statements: Vec<Statement> = vec![];
		while !self.is_at_end() {
			match self.declaration() {
				Ok(expr) => statements.push(expr),
				Err(e) => {
					e.print();
				}
			}
		}
		statements
	}

	fn declaration(&mut self) -> ParseResult<Statement> {
		let statement = if self.peek().typ == TokenType::VAR {
			self.advance();
			return self.var_declaration();
		} else {
			self.statement()
		};
		if statement.is_err() {
			self.synchronize();
		}
		statement
	}

	fn var_declaration(&mut self) -> ParseResult<Statement> {
		let var_name = self.consume(TokenType::IDENTIFIER, String::from("Expect variable name."));
		let mut init = None;
		if self.peek().typ == TokenType::EQUAL {
			self.advance();
			init = Some(self.expression()?);
		}
		self.consume(
			TokenType::SEMICOLON,
			String::from("Expect ';' after variable declaration."),
		);
		match (var_name, init) {
			(Some(name), Some(expr)) => {
				return Ok(Statement::VariableDeclaration(name, Some(Box::new(expr))))
			}
			(Some(name), None) => return Ok(Statement::VariableDeclaration(name, None)),
			(None, _) => return Err(ParseError::MissingVariableName(self.previous())),
		}
	}

	fn statement(&mut self) -> ParseResult<Statement> {
		let current_token = self.peek();
		if current_token.typ == TokenType::IF {
			self.advance();
			return self.if_statement();
		} else if current_token.typ == TokenType::PRINT {
			self.advance();
			return self.print_statement();
		} else if current_token.typ == TokenType::LEFTBRACE {
			self.advance();
			return Ok(Statement::BlockStatement(self.block()));
		}
		self.expression_statement()
	}

	fn if_statement(&mut self) -> ParseResult<Statement> {
		self.consume(TokenType::LEFTPAREN, String::from("Expect '(' after if."));
		let condition = self.expression()?;
		self.consume(
			TokenType::RIGHTPAREN,
			String::from("Expect ')' after if condition."),
		);

		let then_branch = self.statement()?;
		let mut else_branch = None;
		self.advance();
		if self.peek().typ == TokenType::ELSE {
			let else_statement = self.statement()?;
			else_branch = Some(Box::new(else_statement))
		}

		Ok(Statement::IfStatement(
			Box::new(condition),
			Box::new(then_branch),
			else_branch,
		))
	}

	fn print_statement(&mut self) -> ParseResult<Statement> {
		let expr = self.expression()?;
		self.consume(
			TokenType::SEMICOLON,
			String::from("Expect ';' after value."),
		);
		Ok(Statement::PrintStatement(Box::new(expr)))
	}

	fn expression_statement(&mut self) -> ParseResult<Statement> {
		let expr = self.expression()?;
		self.consume(
			TokenType::SEMICOLON,
			String::from("Expect ';' after expression."),
		);
		Ok(Statement::ExpressionStatement(Box::new(expr)))
	}

	fn block(&mut self) -> Vec<Statement> {
		let mut statements: Vec<Statement> = vec![];
		while self.peek().typ != TokenType::RIGHTBRACE && !self.is_at_end() {
			self
				.declaration()
				.map(|statement| statements.push(statement));
		}
		self.consume(
			TokenType::RIGHTBRACE,
			String::from("Expect '}' after block."),
		);
		statements
	}

	fn expression(&mut self) -> ParseResult<ExpressionNode> {
		self.comma()
	}

	fn comma(&mut self) -> ParseResult<ExpressionNode> {
		let expr = self.assignment()?;

		if let Some(binary_op) = self.match_operator_type(vec![BinaryOperator::COMMA]) {
			let right_expr = self.expression()?;

			return Ok(ExpressionNode::new(
				self.current_line(),
				Expression::BinaryExpression(Box::new(expr), binary_op, Box::new(right_expr)),
			));
		}
		Ok(expr)
	}

	fn assignment(&mut self) -> ParseResult<ExpressionNode> {
		let expr = self.ternary()?;

		if self.peek().typ == TokenType::EQUAL {
			let token = self.advance();
			let value_expr = self.assignment()?;

			match expr.expression() {
				Expression::Variable(name) => {
					return Ok(ExpressionNode::new(
						self.current_line(),
						Expression::Assignment(name.clone(), Box::new(value_expr)),
					))
				}
				_ => return Err(ParseError::MissingExpr(token)),
			}
		}
		Ok(expr)
	}

	fn ternary(&mut self) -> ParseResult<ExpressionNode> {
		let expr = self.equality()?;

		if let Some(ternary_op) = self.match_operator_type(vec![TernaryOperator::QUESTIONMARK]) {
			let left_expr = self.expression()?;

			self.consume(
				TokenType::COLON,
				String::from("Expect ':' after then branch of ternary expression."),
			);

			let right_expr = self.expression()?;
			return Ok(ExpressionNode::new(
				self.current_line(),
				Expression::TernaryExpression(
					Box::new(expr),
					ternary_op,
					Box::new(left_expr),
					Box::new(right_expr),
				),
			));
		}
		Ok(expr)
	}

	fn equality(&mut self) -> ParseResult<ExpressionNode> {
		let expr = self.comparison()?;

		if let Some(binary_op) =
			self.match_operator_type(vec![BinaryOperator::BANGEQUAL, BinaryOperator::EQUALEQUAL])
		{
			let right_expr = self.comparison()?;

			return Ok(ExpressionNode::new(
				self.current_line(),
				Expression::BinaryExpression(Box::new(expr), binary_op, Box::new(right_expr)),
			));
		}
		Ok(expr)
	}

	fn comparison(&mut self) -> ParseResult<ExpressionNode> {
		let expr = self.term()?;

		if let Some(binary_op) = self.match_operator_type(vec![
			BinaryOperator::GREATER,
			BinaryOperator::GREATEREQUAL,
			BinaryOperator::LESS,
			BinaryOperator::LESSEQUAL,
		]) {
			let right_expr = self.term()?;

			return Ok(ExpressionNode::new(
				self.current_line(),
				Expression::BinaryExpression(Box::new(expr), binary_op, Box::new(right_expr)),
			));
		}
		Ok(expr)
	}

	fn term(&mut self) -> ParseResult<ExpressionNode> {
		let expr = self.factor()?;

		if let Some(binary_op) =
			self.match_operator_type(vec![BinaryOperator::MINUS, BinaryOperator::PLUS])
		{
			let right_expr = self.factor()?;

			return Ok(ExpressionNode::new(
				self.current_line(),
				Expression::BinaryExpression(Box::new(expr), binary_op, Box::new(right_expr)),
			));
		}
		Ok(expr)
	}

	fn factor(&mut self) -> ParseResult<ExpressionNode> {
		let expr = self.unary()?;

		if let Some(binary_op) =
			self.match_operator_type(vec![BinaryOperator::SLASH, BinaryOperator::STAR])
		{
			let right_expr = self.unary()?;

			return Ok(ExpressionNode::new(
				self.current_line(),
				Expression::BinaryExpression(Box::new(expr), binary_op, Box::new(right_expr)),
			));
		}
		Ok(expr)
	}

	fn unary(&mut self) -> ParseResult<ExpressionNode> {
		if let Some(unary_op) =
			self.match_operator_type(vec![UnaryOperator::BANG, UnaryOperator::MINUS])
		{
			let right_expr = self.unary()?;
			return Ok(ExpressionNode::new(
				self.current_line(),
				Expression::Unary(unary_op, Box::new(right_expr)),
			));
		}
		return self.primary();
	}

	fn primary(&mut self) -> ParseResult<ExpressionNode> {
		let current_token = self.peek();
		let current_line = self.current_line();

		match current_token.typ {
			TokenType::TRUE => {
				self.advance();
				return Ok(ExpressionNode::new(
					current_line,
					Expression::Literal(Literal::BOOLEAN(true)),
				));
			}
			TokenType::FALSE => {
				self.advance();
				return Ok(ExpressionNode::new(
					current_line,
					Expression::Literal(Literal::BOOLEAN(false)),
				));
			}
			TokenType::NIL => {
				self.advance();
				return Ok(ExpressionNode::new(
					current_line,
					Expression::Literal(Literal::NIL),
				));
			}
			TokenType::NUMBER | TokenType::STRING => {
				self.advance();
				return Ok(ExpressionNode::new(
					current_line,
					Expression::Literal(current_token.literal.unwrap()),
				));
			}
			TokenType::LEFTPAREN => {
				self.advance();
				let expr = self.expression()?;
				self.consume(
					TokenType::RIGHTPAREN,
					String::from("Expect ')' after expression."),
				);
				return Ok(ExpressionNode::new(
					current_line,
					Expression::Grouping(Box::new(expr)),
				));
			}
			TokenType::IDENTIFIER => {
				self.advance();
				return Ok(ExpressionNode::new(
					current_line,
					Expression::Variable(current_token),
				));
			}
			_ => {
				return Err(ParseError::MissingExpr(current_token));
			}
		};
	}

	fn match_operator_type<T: OperatorTokenType>(&mut self, types: Vec<T>) -> Option<T> {
		for typ in types {
			if self.peek().typ == OperatorTokenType::token_type(&typ) {
				self.advance();
				return Some(typ);
			}
		}
		None
	}

	fn check_token_type(&self, typ: TokenType) -> bool {
		if self.is_at_end() {
			false;
		}
		self.peek().typ == typ
	}

	fn peek(&self) -> Token {
		self.tokens.get(self.current_token).unwrap().clone()
	}

	fn is_at_end(&self) -> bool {
		self.peek().typ == TokenType::EOF
	}

	fn advance(&mut self) -> Token {
		if !self.is_at_end() {
			self.current_token = self.current_token + 1;
		}
		self.previous()
	}

	fn current_line(&mut self) -> u16 {
		self.peek().line
	}

	fn previous(&self) -> Token {
		if self.current_token == 0 {
			return self.tokens.get(self.current_token).unwrap().clone();
		} else {
			self.tokens.get(self.current_token - 1).unwrap().clone()
		}
	}

	fn consume(&mut self, typ: TokenType, message: String) -> Option<Token> {
		if self.check_token_type(typ) {
			return Some(self.advance());
		}
		self.error(self.peek(), message);
		None
	}

	fn error(&self, token: Token, message: String) {
		Error::error(token.line, message);
	}

	fn synchronize(&mut self) {
		self.advance();
		while !self.is_at_end() {
			if self.previous().typ == TokenType::SEMICOLON {
				break;
			}
			match self.peek().typ {
				TokenType::CLASS
				| TokenType::FUN
				| TokenType::VAR
				| TokenType::FOR
				| TokenType::IF
				| TokenType::WHILE
				| TokenType::PRINT
				| TokenType::RETURN => break,
				_ => {}
			}
			self.advance();
		}
	}
}
