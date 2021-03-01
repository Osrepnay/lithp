pub fn parse(prog: String) -> Result<Vec<Expr>, String> {
    let tokens = tokenize(prog)?;

    fn func_call(tokens: &Vec<Token>, idx: &mut usize) -> Result<Expr, String> {
        let name = match tokens.first() {
            Some(Token::Identifier(name)) => name,
            Some(_) => return Err("Not an identifier".to_owned()),
            None => return Err("Empty tokens".to_owned()),
        };
        if tokens[1] != Token::GroupLeft {
            return Err("Missing opening parenthesis".to_owned());
        }
        if *tokens.last().unwrap() != Token::GroupRight {
            return Err("Missing closing parenthesis".to_owned());
        }
        let mut args = Vec::new();
        while tokens[*idx] != Token::GroupRight {
            args.push(match &tokens[*idx] {
                Token::Identifier(name) => {
                    if tokens[*idx + 1] == Token::GroupLeft {
                        let mut start = 2;
                        let result = func_call(&tokens[*idx..].to_vec(), &mut start)?;
                        *idx += start;
                        result
                    } else {
                        Expr::Identifier(name.clone())
                    }
                }
                Token::StringLiteral(string) => Expr::StringLiteral(string.clone()),
                Token::IntLiteral(int) => Expr::IntLiteral(*int),
                Token::LongLiteral(long) => Expr::LongLiteral(*long),
                Token::DoubleLiteral(double) => Expr::DoubleLiteral(*double),
                Token::BooleanLiteral(boolean) => Expr::BooleanLiteral(*boolean),
                _ => return Err("Unexpected token, expected value".to_owned()),
            });
            *idx += 1;
        }
        Ok(Expr::FuncCall(name.clone(), args))
    }

    let mut exprs = Vec::new();
    let mut total_offset = 0;
    while total_offset < tokens.len() {
        let mut start = 2;
        exprs.push(func_call(&tokens[total_offset..].to_vec(), &mut start)?);
        total_offset += start + 1;
    }
    Ok(exprs)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Identifier(String),
    StringLiteral(String),
    IntLiteral(i32),
    LongLiteral(i64),
    DoubleLiteral(f64),
    BooleanLiteral(bool),
    FuncCall(String, Vec<Expr>),
}

pub fn tokenize(prog: String) -> Result<Vec<Token>, String> {
    let bytes = prog.as_bytes();
    let mut tokens = Vec::new();
    let mut value = "".to_owned();
    let mut in_str = false;
    for byte in bytes {
        let character = *byte as char;
        let mut push_val = || {
            let value = value.clone();
            if value.parse::<i32>().is_ok() {
                tokens.push(Token::IntLiteral(value.parse().unwrap()));
            } else if value.parse::<i64>().is_ok() {
                tokens.push(Token::LongLiteral(value.parse().unwrap()));
            } else if value.parse::<f64>().is_ok() {
                tokens.push(Token::DoubleLiteral(value.parse().unwrap()));
            } else if value.len() >= 2
                && *value.as_bytes().first().unwrap() as char == '"'
                && *value.as_bytes().last().unwrap() as char == '"'
            {
                let length = value.as_bytes().len();
                tokens.push(Token::StringLiteral(value[1..length - 1].to_owned()));
            } else {
                if value == "true".to_owned() {
                    tokens.push(Token::BooleanLiteral(true));
                } else if value == "false".to_owned() {
                    tokens.push(Token::BooleanLiteral(false));
                } else {
                    tokens.push(Token::Identifier(value));
                }
            }
        };
        let token = match character {
            '(' => {
                if in_str {
                    value.push(character);
                    continue;
                }
                if !value.is_empty() {
                    push_val();
                }
                value = "".to_owned();
                Some(Token::GroupLeft)
            }
            ')' => {
                if in_str {
                    value.push(character);
                    continue;
                }
                if !value.is_empty() {
                    push_val();
                }
                value = "".to_owned();
                Some(Token::GroupRight)
            }
            '"' => {
                in_str = !in_str;
                value.push(character);
                None
            }
            ' ' => {
                if in_str {
                    value.push(character);
                    continue;
                }
                if !value.is_empty() {
                    push_val();
                }
                value = "".to_owned();
                None
            }
            '\n' => {
                if in_str {
                    value.push(character);
                    continue;
                }
                if !value.is_empty() {
                    push_val();
                }
                value = "".to_owned();
                None
            }
            _ => {
                value.push(character);
                None
            }
        };
        if let Some(token) = token {
            tokens.push(token);
        }
    }
    let mut push_val = || {
        let value = value.clone();
        if value.parse::<i32>().is_ok() {
            tokens.push(Token::IntLiteral(value.parse().unwrap()));
        } else if value.parse::<i64>().is_ok() {
            tokens.push(Token::LongLiteral(value.parse().unwrap()));
        } else if value.parse::<f64>().is_ok() {
            tokens.push(Token::DoubleLiteral(value.parse().unwrap()));
        } else if value.len() >= 2
            && *value.as_bytes().first().unwrap() as char == '"'
            && *value.as_bytes().last().unwrap() as char == '"'
        {
            let length = value.as_bytes().len();
            tokens.push(Token::StringLiteral(value[1..length - 1].to_owned()));
        } else {
            if value == "true".to_owned() {
                tokens.push(Token::BooleanLiteral(true));
            } else if value == "false".to_owned() {
                tokens.push(Token::BooleanLiteral(false));
            } else {
                tokens.push(Token::Identifier(value));
            }
        }
    };
    if !value.is_empty() {
        push_val();
    }
    Ok(tokens)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    GroupLeft,
    GroupRight,
    Identifier(String),
    StringLiteral(String),
    IntLiteral(i32),
    LongLiteral(i64),
    DoubleLiteral(f64),
    BooleanLiteral(bool),
}
