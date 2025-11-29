// Claude generated AST

// ==================== TOKENIZER ====================

#[derive(Debug, Clone, PartialEq)]
enum TokenType {
    Number,
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    LParen,
    RParen,
    Identifier,
    Comma,
    Eof,
}

#[derive(Debug, Clone)]
struct Token {
    token_type: TokenType,
    value: String,
}

impl Token {
    fn new(token_type: TokenType, value: String) -> Self {
        Token { token_type, value }
    }
}

struct Tokenizer {
    input: Vec<char>,
    position: usize,
}

impl Tokenizer {
    fn new(input: String) -> Self {
        Tokenizer {
            input: input.chars().collect(),
            position: 0,
        }
    }
    
    fn current_char(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }
    
    fn advance(&mut self) {
        self.position += 1;
    }
    
    fn tokenize(mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        
        while let Some(ch) = self.current_char() {
            // Skip whitespace
            if ch.is_whitespace() {
                self.advance();
                continue;
            }
            
            // Numbers (including decimals)
            if ch.is_ascii_digit() {
                let mut num_str = String::new();
                while let Some(c) = self.current_char() {
                    if c.is_ascii_digit() || c == '.' {
                        num_str.push(c);
                        self.advance();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::new(TokenType::Number, num_str));
                continue;
            }
            
            // Identifiers (variables/functions)
            if ch.is_alphabetic() {
                let mut identifier = String::new();
                while let Some(c) = self.current_char() {
                    if c.is_alphanumeric() || c == '_' {
                        identifier.push(c);
                        self.advance();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::new(TokenType::Identifier, identifier));
                continue;
            }
            
            // Operators and punctuation
            let token = match ch {
                '+' => Token::new(TokenType::Plus, ch.to_string()),
                '-' => Token::new(TokenType::Minus, ch.to_string()),
                '*' => Token::new(TokenType::Multiply, ch.to_string()),
                '/' => Token::new(TokenType::Divide, ch.to_string()),
                '^' => Token::new(TokenType::Power, ch.to_string()),
                '(' => Token::new(TokenType::LParen, ch.to_string()),
                ')' => Token::new(TokenType::RParen, ch.to_string()),
                ',' => Token::new(TokenType::Comma, ch.to_string()),
                _ => return Err(format!("Unexpected character: {}", ch)),
            };
            
            tokens.push(token);
            self.advance();
        }
        
        tokens.push(Token::new(TokenType::Eof, String::new()));
        Ok(tokens)
    }
}


// ==================== AST NODE TYPES ====================

#[derive(Debug, Clone)]
pub enum ASTNode {
    Number(f64),
    Variable(String),
    BinaryOp {
        operator: String,
        left: Box<ASTNode>,   // Heap allocation, owned
        right: Box<ASTNode>,  // Heap allocation, owned
    },
    UnaryOp {
        operator: String,
        operand: Box<ASTNode>, // Heap allocation, owned
    },
    FunctionCall {
        name: String,
        arguments: Vec<ASTNode>, // Owned vector of nodes
    },
}


// ==================== PARSER ====================

struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }
    
    fn current_token(&self) -> &Token {
        &self.tokens[self.position]
    }
    
    fn advance(&mut self) {
        if self.position < self.tokens.len() - 1 {
            self.position += 1;
        }
    }
    
    fn expect(&mut self, expected_type: TokenType) -> Result<(), String> {
        if self.current_token().token_type != expected_type {
            return Err(format!(
                "Expected {:?}, found {:?}",
                expected_type,
                self.current_token().token_type
            ));
        }
        self.advance();
        Ok(())
    }
    
    // Main parse function - takes ownership of self and returns AST
    fn parse(mut self) -> Result<ASTNode, String> {
        let ast = self.parse_expression()?;
        
        if self.current_token().token_type != TokenType::Eof {
            return Err("Unexpected token after expression".to_string());
        }
        
        Ok(ast)
    }
    
    // Parse expression (entry point)
    fn parse_expression(&mut self) -> Result<ASTNode, String> {
        self.parse_additive()
    }
    
    // Additive operators: + and -
    fn parse_additive(&mut self) -> Result<ASTNode, String> {
        let mut left = self.parse_multiplicative()?;
        
        while matches!(
            self.current_token().token_type,
            TokenType::Plus | TokenType::Minus
        ) {
            let operator = self.current_token().value.clone();
            self.advance();
            let right = self.parse_multiplicative()?;
            
            // Box::new takes ownership and moves to heap
            left = ASTNode::BinaryOp {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    // Multiplicative operators: * and /
    fn parse_multiplicative(&mut self) -> Result<ASTNode, String> {
        let mut left = self.parse_power()?;
        
        while matches!(
            self.current_token().token_type,
            TokenType::Multiply | TokenType::Divide
        ) {
            let operator = self.current_token().value.clone();
            self.advance();
            let right = self.parse_power()?;
            
            left = ASTNode::BinaryOp {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    // Power operator: ^ (right-associative)
    fn parse_power(&mut self) -> Result<ASTNode, String> {
        let left = self.parse_unary()?;
        
        if self.current_token().token_type == TokenType::Power {
            let operator = self.current_token().value.clone();
            self.advance();
            let right = self.parse_power()?; // Right-associative recursion
            
            return Ok(ASTNode::BinaryOp {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            });
        }
        
        Ok(left)
    }
    
    // Unary operators: + and -
    fn parse_unary(&mut self) -> Result<ASTNode, String> {
        if matches!(
            self.current_token().token_type,
            TokenType::Plus | TokenType::Minus
        ) {
            let operator = self.current_token().value.clone();
            self.advance();
            let operand = self.parse_unary()?; // Allow multiple unary operators
            
            return Ok(ASTNode::UnaryOp {
                operator,
                operand: Box::new(operand),
            });
        }
        
        self.parse_primary()
    }
    
    // Primary expressions: numbers, variables, functions, parentheses
    fn parse_primary(&mut self) -> Result<ASTNode, String> {
        let token = self.current_token();
        
        match &token.token_type {
            TokenType::Number => {
                let value = token.value.parse::<f64>()
                    .map_err(|_| format!("Invalid number: {}", token.value))?;
                self.advance();
                Ok(ASTNode::Number(value))
            }
            
            TokenType::Identifier => {
                let name = token.value.clone(); // Clone the string
                self.advance();
                
                // Check if it's a function call
                if self.current_token().token_type == TokenType::LParen {
                    self.advance(); // consume '('
                    
                    let mut arguments = Vec::new();
                    
                    if self.current_token().token_type != TokenType::RParen {
                        arguments.push(self.parse_expression()?);
                        
                        while self.current_token().token_type == TokenType::Comma {
                            self.advance();
                            arguments.push(self.parse_expression()?);
                        }
                    }
                    
                    self.expect(TokenType::RParen)?;
                    
                    Ok(ASTNode::FunctionCall { name, arguments })
                } else {
                    // It's a variable
                    Ok(ASTNode::Variable(name))
                }
            }
            
            TokenType::LParen => {
                self.advance(); // consume '('
                let expr = self.parse_expression()?;
                self.expect(TokenType::RParen)?;
                Ok(expr)
            }
            
            _ => Err(format!("Unexpected token: {:?}", token.token_type)),
        }
    }
}


pub fn build_ast(equation: &str) -> Result<ASTNode, String> {
    let tokenizer = Tokenizer::new(equation.to_string());
    let tokens = tokenizer.tokenize()?;
    
    let parser = Parser::new(tokens);
    let ast = parser.parse()?;
    
    Ok(ast)
}
