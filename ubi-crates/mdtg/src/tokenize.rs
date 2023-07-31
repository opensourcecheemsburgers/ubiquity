use std::iter::Peekable;

/// Represents an input token
#[derive(Debug, PartialEq)]
pub enum Token {
    Cross,
    Num(usize),
    Position(char),
}

/// Given a string, return a list of tokens representing that string,
/// or an error if one has occurred
pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::<Token>::new();

    let mut it = input.chars().peekable();

    while let Some(&c) = it.peek() {
        match c {
            'x' => {
                tokens.push(Token::Cross);
                it.next();
            }
            'l' | 'c' | 'r' => {
                tokens.push(Token::Position(c));
                it.next();
            }
            '1'..='9' => {
                it.next();
                let n = get_num(c, &mut it);
                tokens.push(Token::Num(n));
            }
            ' ' => {
                it.next();
            }
            '0' => return Err("Column/row number must be > 0".to_string()),
            _ => return Err(format!("Unexpected input '{}'", c)),
        }
    }

    Ok(tokens)
}

fn get_num<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> usize {
    let mut n = c.to_digit(10).unwrap() as usize;

    while let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<usize>()) {
        n = n * 10 + digit;
        iter.next();
    }

    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_table() {
        let input = "3 x 5";
        assert_eq!(
            tokenize(&input),
            Ok(vec![Token::Num(3), Token::Cross, Token::Num(5)])
        );
    }

    #[test]
    fn test_complex_table() {
        let input = "6lcr x 2";
        assert_eq!(
            tokenize(&input),
            Ok(vec![
                Token::Num(6),
                Token::Position('l'),
                Token::Position('c'),
                Token::Position('r'),
                Token::Cross,
                Token::Num(2)
            ])
        );
    }

    #[test]
    fn test_invalid_table() {
        let input = "3a x 5";
        assert!(tokenize(&input).is_err());
    }
}
