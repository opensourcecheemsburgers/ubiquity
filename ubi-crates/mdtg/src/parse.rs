use tokenize::Token;

/// Represents an AST type
#[derive(Debug, Clone, PartialEq)]
pub enum AST {
    Column(usize),
    Row(usize),
    Position(char),
    Cross,
    Table,
}

/// Represents an AST node, containing an AST item and a list of children nodes
#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub children: Vec<Node>,
    pub item: AST,
}

impl Node {
    /// Create a new `Node` using a given item
    pub fn new(item: AST) -> Node {
        Node {
            children: Vec::new(),
            item,
        }
    }

    /// Given a slice of nodes, add them all to the nodes children vec
    pub fn add_children(&mut self, children: &[Node]) {
        self.children.extend_from_slice(children);
    }
}

/// Parse a list of tokens into an AST.
///
/// Returns a node representing the markdown table specification AST,
/// or a string representing an error that has occurred during parsing.
///
/// For example, an AST returned could look like
///
/// ```
/// Node {
///   children: [
///     Node {
///       children: [
///         Node { children: [], item: Position('c') },
///         Node { children: [], item: Position('r') }
///       ],
///       item: Columns(3)
///     },
///     Node { children: [], item: Cross },
///     Node { children: [], item: Rows(5) }
///   ],
///   item: Table
/// }
/// ```
pub fn parse(mut tokens: Vec<Token>) -> Result<Node, String> {
    tokens.reverse();
    parse_table(&mut tokens)
}

fn parse_table(tokens: &mut Vec<Token>) -> Result<Node, String> {
    let columns_node = parse_columns(tokens)?;
    let cross_node = parse_cross(tokens)?;
    let rows_node = parse_rows(tokens)?;

    let mut table_node = Node::new(AST::Table);

    table_node.add_children(&[columns_node, cross_node, rows_node]);

    Ok(table_node)
}

fn parse_columns(tokens: &mut Vec<Token>) -> Result<Node, String> {
    let tok = tokens.pop();

    fn is_position(tok: &Token) -> bool {
        if let Token::Position(_) = tok {
            true
        } else {
            false
        }
    }

    if let Some(Token::Num(n)) = tok {
        let mut columns_node = Node::new(AST::Column(n));

        while tokens.last().map_or(false, is_position) {
            if let Some(Token::Position(p)) = tokens.pop() {
                let position_node = Node::new(AST::Position(p));
                columns_node.add_children(&[position_node]);
            }
        }

        if columns_node.children.len() > n as usize {
            Err("Number of positions exceed number of columns".to_string())
        } else {
            Ok(columns_node)
        }
    } else {
        Err("Expected a column number".to_string())
    }
}

fn parse_cross(tokens: &mut Vec<Token>) -> Result<Node, String> {
    let tok = tokens.pop();

    if let Some(Token::Cross) = tok {
        Ok(Node::new(AST::Cross))
    } else {
        Err("Expected 'x'".to_string())
    }
}

fn parse_rows(tokens: &mut Vec<Token>) -> Result<Node, String> {
    let tok = tokens.pop();

    if let Some(Token::Num(n)) = tok {
        Ok(Node::new(AST::Row(n)))
    } else {
        Err("Expected a row number".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_table() {
        let tokens = vec![Token::Num(3), Token::Cross, Token::Num(5)];

        let mut table_node = Node::new(AST::Table);
        let column_node = Node::new(AST::Column(3));
        let cross_node = Node::new(AST::Cross);
        let row_node = Node::new(AST::Row(5));

        table_node.add_children(&[column_node, cross_node, row_node]);

        assert_eq!(parse(tokens), Ok(table_node));
    }

    #[test]
    fn test_complex_table() {
        let tokens = vec![
            Token::Num(6),
            Token::Position('l'),
            Token::Position('c'),
            Token::Position('r'),
            Token::Cross,
            Token::Num(2),
        ];

        let mut table_node = Node::new(AST::Table);
        let mut column_node = Node::new(AST::Column(6));
        let cross_node = Node::new(AST::Cross);
        let row_node = Node::new(AST::Row(2));

        let left_position_node = Node::new(AST::Position('l'));
        let center_position_node = Node::new(AST::Position('c'));
        let right_position_node = Node::new(AST::Position('r'));

        column_node.add_children(&[
            left_position_node,
            center_position_node,
            right_position_node,
        ]);
        table_node.add_children(&[column_node, cross_node, row_node]);

        assert_eq!(parse(tokens), Ok(table_node));
    }

    #[test]
    fn test_missing_x_table() {
        let tokens = vec![Token::Num(3), Token::Num(5)];
        assert_eq!(parse(tokens), Err("Expected 'x'".to_string()));
    }

    #[test]
    fn test_missing_column_table() {
        let tokens = vec![Token::Cross, Token::Num(5)];
        assert_eq!(parse(tokens), Err("Expected a column number".to_string()));
    }

    #[test]
    fn test_missing_row_table() {
        let tokens = vec![Token::Num(3), Token::Cross];
        assert_eq!(parse(tokens), Err("Expected a row number".to_string()));
    }

    #[test]
    fn test_excess_positions_table() {
        let tokens = vec![
            Token::Num(1),
            Token::Position('c'),
            Token::Position('c'),
            Token::Cross,
        ];
        assert_eq!(
            parse(tokens),
            Err("Number of positions exceed number of columns".to_string())
        );
    }
}
