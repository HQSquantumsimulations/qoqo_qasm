use qoqo_calculator::CalculatorError;
use std::collections::HashSet;
use std::str::FromStr;
static ATOL: f64 = f64::EPSILON;

/// Match name of function to number of arguments.
/// Returns result with CalculatorError when function name is not known.
fn function_argument_numbers(input: &str) -> Result<usize, CalculatorError> {
    match input {
        "sin" => Ok(1),
        "cos" => Ok(1),
        "abs" => Err(CalculatorError::ParsingError {
            msg: "Function abs is not supported in OpenQASM 3.0.",
        }),
        "tan" => Ok(1),
        "acos" => Ok(1),
        "asin" => Ok(1),
        "atan" => Ok(1),
        "cosh" => Err(CalculatorError::ParsingError {
            msg: "Function cosh is not supported in OpenQASM 3.0.",
        }),
        "sinh" => Err(CalculatorError::ParsingError {
            msg: "Function sinh is not supported in OpenQASM 3.0.",
        }),
        "tanh" => Err(CalculatorError::ParsingError {
            msg: "Function tanh is not supported in OpenQASM 3.0.",
        }),
        "acosh" => Err(CalculatorError::ParsingError {
            msg: "Function acosh is not supported in OpenQASM 3.0.",
        }),
        "asinh" => Err(CalculatorError::ParsingError {
            msg: "Function asinh is not supported in OpenQASM 3.0.",
        }),
        "atanh" => Err(CalculatorError::ParsingError {
            msg: "Function atanh is not supported in OpenQASM 3.0.",
        }),
        "arcosh" => Err(CalculatorError::ParsingError {
            msg: "Function arcosh is not supported in OpenQASM 3.0.",
        }),
        "arsinh" => Err(CalculatorError::ParsingError {
            msg: "Function arsinh is not supported in OpenQASM 3.0.",
        }),
        "artanh" => Err(CalculatorError::ParsingError {
            msg: "Function artanh is not supported in OpenQASM 3.0.",
        }),
        "exp" => Ok(1),
        "exp2" => Err(CalculatorError::ParsingError {
            msg: "Function exp2 is not supported in OpenQASM 3.0.",
        }),
        "expm1" => Err(CalculatorError::ParsingError {
            msg: "Function expm1 is not supported in OpenQASM 3.0.",
        }), //< exponential minus Ok(1)
        "log" => Ok(1),
        "log10" => Err(CalculatorError::ParsingError {
            msg: "Function log10 is not supported in OpenQASM 3.0.",
        }),
        "sqrt" => Ok(1),
        "cbrt" => Err(CalculatorError::ParsingError {
            msg: "Function cbrt is not supported in OpenQASM 3.0.",
        }), //< cubic root
        "ceil" => Ok(1),
        "floor" => Ok(1),
        "fract" => Err(CalculatorError::ParsingError {
            msg: "Function fract is not supported in OpenQASM 3.0.",
        }),
        "round" => Err(CalculatorError::ParsingError {
            msg: "Function round is not supported in OpenQASM 3.0.",
        }),
        "erf" => Err(CalculatorError::ParsingError {
            msg: "Function erf is not supported in OpenQASM 3.0.",
        }),
        "tgamma" => Err(CalculatorError::ParsingError {
            msg: "Function tgamma is not supported in OpenQASM 3.0.",
        }),
        "lgamma" => Err(CalculatorError::ParsingError {
            msg: "Function lgamma is not supported in OpenQASM 3.0.",
        }),
        "sign" => Ok(1),
        "delta" => Err(CalculatorError::ParsingError {
            msg: "Function delta is not supported in OpenQASM 3.0.",
        }),
        "theta" => Err(CalculatorError::ParsingError {
            msg: "Function theta is not supported in OpenQASM 3.0.",
        }),
        "parity" => Err(CalculatorError::ParsingError {
            msg: "Function parity is not supported in OpenQASM 3.0.",
        }),
        "atan2" => Err(CalculatorError::ParsingError {
            msg: "Function atan2 is not supported in OpenQASM 3.0.",
        }),
        "hypot" => Err(CalculatorError::ParsingError {
            msg: "Function hypot is not supported in OpenQASM 3.0.",
        }),
        "pow" => Ok(2),
        "max" => Err(CalculatorError::ParsingError {
            msg: "Function max is not supported in OpenQASM 3.0.",
        }),
        "min" => Err(CalculatorError::ParsingError {
            msg: "Function min is not supported in OpenQASM 3.0.",
        }),
        _ => Err(CalculatorError::FunctionNotFound {
            fct: input.to_string(),
        }),
    }
}

/// Match name of function with one argument to Rust function and return Result.
fn function_1_argument(input: &str, arg0: f64) -> Result<f64, CalculatorError> {
    match input {
        "sin" => Ok(arg0.sin()),
        "cos" => Ok(arg0.cos()),
        "abs" => Ok(arg0.abs()),
        "tan" => Ok(arg0.tan()),
        "acos" => Ok(arg0.acos()),
        "asin" => Ok(arg0.asin()),
        "atan" => Ok(arg0.atan()),
        "cosh" => Ok(arg0.cosh()),
        "sinh" => Ok(arg0.sinh()),
        "tanh" => Ok(arg0.tanh()),
        "acosh" => Ok(arg0.acosh()),
        "asinh" => Ok(arg0.asinh()),
        "atanh" => Ok(arg0.atanh()),
        "arcosh" => Ok(arg0.acosh()),
        "arsinh" => Ok(arg0.asinh()),
        "artanh" => Ok(arg0.atanh()),
        "exp" => Ok(arg0.exp()),
        "exp2" => Ok(arg0.exp2()),
        "expm1" => Ok(arg0.exp_m1()), //< exponential minus 1
        "log" => Ok(arg0.ln()),
        "log10" => Ok(arg0.log10()),
        "sqrt" => Ok(arg0.sqrt()),
        "cbrt" => Ok(arg0.cbrt()), //< cubic root
        "ceil" => Ok(arg0.ceil()),
        "floor" => Ok(arg0.floor()),
        "fract" => Ok(arg0.fract()),
        "round" => Ok(arg0.round()),
        "sign" => Ok(arg0.signum()),
        "delta" => {
            if (arg0 - 0.0).abs() < ATOL {
                Ok(1.0)
            } else {
                Ok(0.0)
            }
        }
        "theta" => {
            if (arg0 - 0.0).abs() < ATOL {
                Ok(0.5)
            } else if arg0 < 0.0 {
                Ok(0.0)
            } else {
                Ok(1.0)
            }
        }
        //"parity" => {let m = i64::from((arg0+0.5).floor());
        //     if m.overflowing_rem(2) {Ok(-1.0)} else {Ok(1.0)}},
        _ => Err(CalculatorError::FunctionNotFound {
            fct: input.to_string(),
        }),
    }
}

/// Match name of function with two arguments to Rust function and return Result.
fn function_2_arguments(input: &str, arg0: f64, arg1: f64) -> Result<f64, CalculatorError> {
    match input {
        "atan2" => Ok(arg0.atan2(arg1)),
        "hypot" => Ok(arg0.hypot(arg1)),
        "pow" => Ok(arg0.powf(arg1)),
        "max" => Ok(arg0.max(arg1)),
        "min" => Ok(arg0.min(arg1)),
        _ => Err(CalculatorError::FunctionNotFound {
            fct: input.to_string(),
        }),
    }
}

/// Struct to keep track of variables present in input Circuit.
#[derive(Debug, Clone)]
pub struct VariableGatherer {
    ///  HashSet of variables in current Circuit
    pub variables: HashSet<String>,
}

impl Default for VariableGatherer {
    fn default() -> Self {
        Self::new()
    }
}

impl VariableGatherer {
    /// Create a new CircuitParser instance.
    pub fn new() -> Self {
        VariableGatherer {
            variables: HashSet::new(),
        }
    }

    /// Register variable for CircuitParser.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the variable
    ///
    pub fn register_variable(&mut self, name: &str) {
        self.variables.insert(name.to_string());
    }

    ///  Parse a string expression allowing variable assignments.
    ///
    ///
    ///
    /// # Arguments
    ///
    /// * `expression` - Expression that is parsed
    ///
    pub fn parse(&mut self, expression: &str) -> Result<(), CalculatorError> {
        let mut parser = MutableCircuitParser::new_mutable(expression, self);
        let end_value = parser.evaluate_all_tokens()?;
        match end_value {
            None => Err(CalculatorError::NoValueReturnedParsing),
            Some(_) => Ok(()),
        }
    }
}

/// Enum combining different types of Tokens in an Expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// A float or integer
    Number(f64),
    /// A variable
    Variable(String),
    /// A  known function
    Function(String),
    /// Plus
    Plus,
    /// Minus
    Minus,
    /// Multiply
    Multiply,
    /// Divice
    Divide,
    /// Poser
    Power,
    /// Factorial
    Factorial,
    /// DoubleFactorial
    DoubleFactorial,
    /// A bracket opening
    BracketOpen,
    /// A bracket closing
    BracketClose,
    /// Assign operator
    Assign,
    /// Assignment of a variable
    VariableAssign(String),
    /// Comma
    Comma,
    /// End of Expression
    EndOfExpression,
    /// End of parsed string
    EndOfString,
    /// No Token has been recognized in string
    Unrecognized,
}

/// Struct implementing Iterator trait to lex string
/// to computational Tokens.
#[derive(Debug)]
pub struct TokenIterator<'a> {
    // Save current expression as a slice of a string so we do not
    // need to copy but only modify (shorten) the slice.
    //
    /// Current str expression being lexed
    pub current_expression: &'a str,
}

// Implement the Iterator Trait for TokenIterator so it can be used as standard rust iterator.
impl<'a, 'b> Iterator for TokenIterator<'a>
where
    'a: 'b,
{
    type Item = Token;

    // Define next method for Token iterator
    fn next(&mut self) -> Option<Token> {
        if self.current_expression.is_empty() {
            None
        } else {
            // Loop to remove whitespace and comments
            loop {
                if self.current_expression.starts_with(' ') {
                    let end = self
                        .current_expression
                        .char_indices()
                        .find_map(|(ind, c)| if c.is_whitespace() { None } else { Some(ind) })
                        .unwrap_or(self.current_expression.len());
                    self.cut_current_expression(end);
                    if self.current_expression.is_empty() {
                        return Some(Token::EndOfString);
                    }
                    continue;
                } else if self.current_expression.starts_with('#') {
                    let end = self
                        .current_expression
                        .char_indices()
                        .find_map(|(ind, c)| if c != '\u{000A}' { None } else { Some(ind + 1) })
                        .unwrap_or(self.current_expression.len());
                    self.cut_current_expression(end);
                    if self.current_expression.is_empty() {
                        return Some(Token::EndOfString);
                    }
                    continue;
                }
                break;
            }
            // Test if head of current_expression is a letter char
            if self
                .current_expression
                .chars()
                .next()
                .unwrap()
                .is_alphabetic()
            {
                // Find end of symbolic expression (not alphanumeric or '_')
                let end = self
                    .current_expression
                    .char_indices()
                    .find_map(|(ind, c)| {
                        if c.is_alphanumeric() || c == '_' {
                            None
                        } else {
                            Some(ind)
                        }
                    })
                    .unwrap_or(self.current_expression.len());
                // Get next token from TokenIterator with shortened expression
                let next_token = if end >= self.current_expression.len() {
                    TokenIterator {
                        current_expression: "",
                    }
                    .next()
                } else {
                    TokenIterator {
                        current_expression: &self.current_expression[end..],
                    }
                    .next()
                };
                // Depending on next token currently lexed string current_expression[..end] creates different tokens
                // Token contains current_expression[..end] for later processing
                return Some(match next_token {
                    Some(Token::Assign) => {
                        let vs = self.current_expression[..end].to_owned();
                        self.cut_current_expression(end + 1);
                        Token::VariableAssign(vs)
                    }
                    Some(Token::BracketOpen) => {
                        let vs = self.current_expression[..end].to_owned();
                        self.cut_current_expression(end + 1);
                        Token::Function(vs)
                    }
                    _ => {
                        let vs = self.current_expression[..end].to_owned();
                        self.cut_current_expression(end);
                        Token::Variable(vs)
                    }
                });
            }
            // Lex string that contains a number.
            // Test if current expression starts with ascii number
            if self
                .current_expression
                .chars()
                .next()
                .unwrap()
                .is_ascii_digit()
                || self.current_expression.starts_with('.')
            {
                // find end of number expression
                let (end, next_char) = self
                    .current_expression
                    .char_indices()
                    .find(|(_, c)| !c.is_ascii_digit() && c != &'.')
                    .unwrap_or((self.current_expression.len(), ' '));
                let mut end_offset = 0;
                let mut start: usize = 0;
                // Handle scientific notation.
                // Starts with e or E for scientific notation
                if next_char == 'e' || next_char == 'E' {
                    // offset for just 'e' or 'E'
                    start = 1;
                    if self
                        .current_expression
                        .chars()
                        .nth(end + start)
                        .unwrap_or(' ')
                        == '+'
                        || self
                            .current_expression
                            .chars()
                            .nth(end + start)
                            .unwrap_or(' ')
                            == '-'
                    {
                        // offset if exponent has sign
                        start = 2;
                    }
                    // Find end of exponent
                    end_offset = self.current_expression[end + start..]
                        .char_indices()
                        .find_map(|(ind, c)| if c.is_ascii_digit() { None } else { Some(ind) })
                        .unwrap_or(self.current_expression.len() - (end + start));
                }
                let end_total = end + start + end_offset;
                let number_expression = &self.current_expression[..end_total];
                // Use inbuilt rust string -> number conversion to get number and handle errors
                self.cut_current_expression(end_total);
                return Some(match f64::from_str(number_expression) {
                    Err(_) => Token::Unrecognized,
                    Ok(f) => Token::Number(f.to_owned()),
                });
            };
            // Create symbol tokens
            let symbol = self.current_expression.chars().next().unwrap();
            self.current_expression = &self.current_expression[1..];
            return Some(match symbol {
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => match self.current_expression.chars().next().unwrap_or(' ') {
                    '*' => {
                        self.current_expression = &self.current_expression[1..];
                        Token::Power
                    }
                    _ => Token::Multiply,
                },
                '/' => Token::Divide,
                '^' => Token::Power,
                '(' => Token::BracketOpen,
                ')' => Token::BracketClose,
                '=' => Token::Assign,
                ',' => Token::Comma,
                ';' => Token::EndOfExpression,
                '!' => match self.current_expression.chars().next().unwrap_or(' ') {
                    '!' => {
                        self.current_expression = &self.current_expression[1..];
                        Token::DoubleFactorial
                    }
                    _ => Token::Factorial,
                },
                _ => Token::Unrecognized,
            });
        }
    }
}

// Helper methods not in standard iterator trait.
impl<'a> TokenIterator<'a> {
    // Return the next token and the current token (in string form).
    fn next_token_and_str(&mut self) -> (Option<Token>, &'a str) {
        let next_token = self.next();
        let next_str = self.current_expression;
        (next_token, next_str)
    }

    // Modify the current expression to be a slice of the current expression.
    fn cut_current_expression(&mut self, end: usize) {
        if end == self.current_expression.len() {
            self.current_expression = "";
        } else {
            self.current_expression = &self.current_expression[end..];
        }
    }
}

/// Parser from &str to f64 using TokenIterator lexer.
struct MutableCircuitParser<'a> {
    /// Expression that has not been parsed yet
    remaining_expression: &'a str,
    /// Token that is currently parsed
    current_token: Token,
    /// CircuitParser that contains set variables
    circuit_parser: &'a mut VariableGatherer,
}

impl<'a, 'b> MutableCircuitParser<'a>
where
    'b: 'a,
{
    pub fn register_variable(&mut self, name: &str) {
        self.circuit_parser.register_variable(name);
    }

    fn new_mutable(expression: &'a str, circuit_parser: &'b mut VariableGatherer) -> Self {
        let (next_token, next_str) = (TokenIterator {
            current_expression: expression,
        })
        .next_token_and_str();
        MutableCircuitParser {
            remaining_expression: next_str,
            current_token: next_token.unwrap(),
            circuit_parser,
        }
    }

    fn remaining_expression(&mut self) -> &'a str {
        self.remaining_expression
    }

    fn current_token(&self) -> &Token {
        &self.current_token
    }

    /// Get next token via TokenIterator.
    fn next_token(&mut self) {
        let (next_token, next_str) = (TokenIterator {
            current_expression: self.remaining_expression(),
        })
        .next_token_and_str();
        match next_token {
            None => {
                self.current_token = Token::EndOfString;
                self.remaining_expression = "";
            }
            Some(t) => {
                self.current_token = t;
                self.remaining_expression = next_str;
            }
        }
    }

    /// Evaluate all Tokens to real value, None (for not returning expressions)
    /// or return error.
    fn evaluate_all_tokens(&mut self) -> Result<Option<f64>, CalculatorError> {
        let mut current_value: Option<f64> = None;
        while self.current_token() != &Token::EndOfString {
            current_value = self.evaluate_init()?;
            while self.current_token() == &Token::EndOfExpression {
                self.next_token();
            }
        }
        Ok(current_value)
    }

    /// Initialize the evaluation of an expression.
    fn evaluate_init(&mut self) -> Result<Option<f64>, CalculatorError> {
        if self.current_token() == &Token::EndOfExpression
            || self.current_token() == &Token::EndOfString
        {
            Err(CalculatorError::UnexpectedEndOfExpression)
        } else {
            Ok(Some(self.evaluate_binary_1()?))
        }
    }

    /// Evaluate least preference binary expression (+, -).
    fn evaluate_binary_1(&mut self) -> Result<f64, CalculatorError> {
        let mut res = self.evaluate_binary_2()?;
        while self.current_token() == &Token::Plus || self.current_token() == &Token::Minus {
            let bsum: bool = self.current_token() == &Token::Plus;
            self.next_token();
            let val = self.evaluate_binary_2()?;
            if bsum {
                res += val;
            } else {
                res -= val;
            }
        }
        Ok(res)
    }

    /// Evaluate middle preference binary expression (*, /).
    fn evaluate_binary_2(&mut self) -> Result<f64, CalculatorError> {
        let mut res = self.evaluate_binary_3()?;
        while self.current_token() == &Token::Multiply || self.current_token() == &Token::Divide {
            let bmul: bool = self.current_token() == &Token::Multiply;
            self.next_token();
            let val = self.evaluate_binary_3()?;
            if bmul {
                res *= val;
            } else {
                if val == 0.0 {
                    return Err(CalculatorError::DivisionByZero);
                }
                res /= val;
            }
        }
        Ok(res)
    }

    /// Evaluate least preference binary expression (^, !).
    fn evaluate_binary_3(&mut self) -> Result<f64, CalculatorError> {
        let mut res = self.evaluate_unary()?;
        match self.current_token() {
            Token::DoubleFactorial => {
                return Err(CalculatorError::NotImplementedError {
                    fct: "DoubleFactorial",
                })
            }
            Token::Factorial => {
                return Err(CalculatorError::NotImplementedError { fct: "Factorial" })
            }
            Token::Power => {
                self.next_token();
                res = res.powf(self.evaluate_unary()?);
            }
            _ => (),
        }
        Ok(res)
    }

    /// Handle any unary + or - signs.
    fn evaluate_unary(&mut self) -> Result<f64, CalculatorError> {
        let mut prefactor: f64 = 1.0;
        match self.current_token() {
            Token::Minus => {
                self.next_token();
                prefactor = -1.0;
            }
            Token::Plus => {
                self.next_token();
            }
            _ => (),
        }
        Ok(prefactor * self.evaluate()?)
    }

    /// Handle numbers, variables, functions and parentheses.
    fn evaluate(&mut self) -> Result<f64, CalculatorError> {
        match self.current_token().clone() {
            Token::BracketOpen => {
                self.next_token();
                let res_init = self.evaluate_init()?.ok_or(CalculatorError::ParsingError {
                    msg: "Unexpected None return",
                })?;
                //self.next_token()?;
                if self.current_token() != &Token::BracketClose {
                    Err(CalculatorError::ParsingError {
                        msg: "Expected Braket close",
                    })
                } else {
                    self.next_token();
                    Ok(res_init)
                }
            }
            Token::Number(vf) => {
                self.next_token();
                Ok(vf)
            }
            Token::Variable(ref vs) => {
                let vsnew = vs.to_owned();
                self.next_token();
                self.register_variable(&vsnew);
                Ok(0.0)
            }
            Token::Function(ref vs) => {
                let vsnew = vs.to_owned();
                self.next_token();
                let mut heap = Vec::new();
                let number_arguments = function_argument_numbers(&vsnew)?;
                for argument_number in 0..number_arguments {
                    heap.push(
                        self.evaluate_init()?
                            .ok_or(CalculatorError::NoValueReturnedParsing)?,
                    );
                    // Swallow commas in function arguments
                    if argument_number < number_arguments - 1 {
                        if self.current_token() != &Token::Comma {
                            return Err(CalculatorError::ParsingError {
                                msg: "expected comma in function arguments",
                            });
                        } else {
                            self.next_token();
                        }
                    }
                    //self.next_token()?;
                }
                if self.current_token() != &Token::BracketClose {
                    return Err(CalculatorError::ParsingError {
                        msg: "Expected braket close.",
                    });
                }
                self.next_token();
                match number_arguments {
                    1 => function_1_argument(
                        &vsnew,
                        *(heap
                            .first()
                            .ok_or(CalculatorError::NotEnoughFunctionArguments)?),
                    ),
                    2 => function_2_arguments(
                        &vsnew,
                        *(heap
                            .first()
                            .ok_or(CalculatorError::NotEnoughFunctionArguments)?),
                        *(heap
                            .get(1)
                            .ok_or(CalculatorError::NotEnoughFunctionArguments)?),
                    ),
                    _ => Err(CalculatorError::ParsingError {
                        msg: "Unsupported number of arguments.",
                    }),
                }
            }
            _ => Err(CalculatorError::ParsingError {
                msg: "Bad_Position",
            }),
        }
    }
}
