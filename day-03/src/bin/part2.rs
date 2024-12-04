use logos::Logos;
use std::io;
use std::num::ParseIntError;
use std::result::Result;
use thiserror::Error;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    // match mul(a,b) style expression
    #[regex(r"mul\(\d+,\d+\)")]
    Mul,
    // match do() instruction
    #[regex(r"do\(\)")]
    Do,
    // match don't() instruction
    #[regex(r"don't\(\)")]
    Dont,
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("Parse Error")]
    InvalidExpression,
    #[error("Parse Int Error: {0}")]
    InvalidNumber(#[from] ParseIntError),
}

fn parse_mul_expression(expression: &str) -> Result<(i64, i64), ParseError> {
    // Check for proper prefix and suffix
    if !expression.starts_with("mul(") || !expression.ends_with(")") {
        return Err(ParseError::InvalidExpression);
    }

    // Remove "mul(" and ")" from the expression
    let params = expression.trim_start_matches("mul(").trim_end_matches(")");

    // Split the parameters
    let mut split = params.split(",");

    // Parse the first and second numbers
    let a = split
        .next()
        .ok_or(ParseError::InvalidExpression)?
        .trim()
        .parse::<i64>()
        .map_err(ParseError::InvalidNumber)?;
    let b = split
        .next()
        .ok_or(ParseError::InvalidExpression)?
        .trim()
        .parse::<i64>()
        .map_err(ParseError::InvalidNumber)?;

    // Check for any unexpected extra parts
    if split.next().is_some() {
        return Err(ParseError::InvalidExpression);
    }

    Ok((a, b))
}

fn scan_forward_and_multiply(input: &str) -> Result<i64, ParseError> {
    let mut current_sum = 0;
    let mut is_product_enabled = true;
    let mut lexer = Token::lexer(input);
    while let Some(token) = lexer.next() {
        match token {
            Ok(Token::Mul) => {
                if is_product_enabled {
                    let expression = lexer.slice();
                    let (a, b) = parse_mul_expression(expression)?;
                    current_sum += a * b;
                }
            }
            Ok(Token::Do) => {
                is_product_enabled = true;
            }
            Ok(Token::Dont) => {
                is_product_enabled = false;
            }
            _ => { /* Ignore other tokens */ }
        }
    }

    Ok(current_sum)
}

fn main() {
    let input = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .collect::<String>();
    let result = scan_forward_and_multiply(&input);
    match result {
        Ok(x) => println!("{}", x),
        Err(e) => eprintln!("{}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = scan_forward_and_multiply(sample);
        let expect = 161;
        assert!(result.is_ok(), "Error: {:?}", result.err());
        assert_eq!(result.unwrap(), expect);
    }

    #[test]
    fn test_sample_2() {
        let sample = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = scan_forward_and_multiply(sample);
        let expect = 48;
        assert!(result.is_ok(), "Error: {:?}", result.err());
        assert_eq!(result.unwrap(), expect);
    }

    #[test]
    fn test_simple() {
        let sample = "xmul(2,4)";
        let result = scan_forward_and_multiply(sample);
        let expect = 8;
        assert!(result.is_ok(), "Error: {:?}", result.err());
        assert_eq!(result.unwrap(), expect);
    }

    #[test]
    fn test_multi() {
        let sample = "xmul(2,4)%&mul[3,7]";
        let result = scan_forward_and_multiply(sample);
        let expect = 8;
        assert!(result.is_ok(), "Error: {:?}", result.err());
        assert_eq!(result.unwrap(), expect);
    }

    #[test]
    fn test_bad_nesting() {
        let sample = "xmul(2,4)mul%&mul[3,7])";
        let result = scan_forward_and_multiply(sample);
        let expect = 8;
        assert!(result.is_ok(), "Error: {:?}", result.err());
        assert_eq!(result.unwrap(), expect);
    }
}
