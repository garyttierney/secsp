use ast::*;
use name::*;
use security_attributes::*;


named!(pub expr<&[u8], Expr>,
  alt_complete!(
    context
    | level_range 
    | category_range
    | logical_or_expr
  )
);

macro_rules! binexp {
  ($name:ident, $next: ident, $op:path, $tag:expr) => {
    named!(pub $name<&[u8], Expr>,
      ws!(do_parse!(
        a: $next >>
        n: alt_complete!(
          ws!(do_parse!(
            tag!($tag) >>
            b: $name >>
            (Expr::Binary(Box::new(a.clone()), $op, Box::new(b)))
          )) |
          value!(a)
        ) >>
        (n)
      ))
    );
  }
}

binexp!(
    logical_or_expr,
    logical_xor_expr,
    BinaryOp::ConditionalOr,
    "||"
);

binexp!(
    logical_xor_expr,
    logical_and_expr,
    BinaryOp::ConditionalXor,
    "^^"
);

binexp!(
    logical_and_expr,
    bitwise_or_expr,
    BinaryOp::ConditionalAnd,
    "&&"
);

binexp!(bitwise_or_expr, bitwise_xor_expr, BinaryOp::BitwiseOr, "|");

binexp!(
    bitwise_xor_expr,
    bitwise_and_expr,
    BinaryOp::BitwiseXor,
    "^"
);

binexp!(bitwise_and_expr, unary_expr, BinaryOp::BitwiseAnd, "&");

named!(unary_expr<&[u8], Expr>,
    ws!(
        alt!(
            do_parse!(
                op: alt!(
                    map!(tag!("!"),|_|  UnaryOp::ConditionalNot) |
                    map!(tag!("~"), |_| UnaryOp::BitwiseNot)
                ) >>
                expr: primary_expr >>

                (Expr::Unary(op, Box::new(expr)))
            ) |
            primary_expr
        )
    )
);

named!(pub primary_expr<&[u8], Expr>,
    alt_complete!(
        variable |
        delimited!(char!('('), expr, char!(')')) |
        delimited!(char!('('), variable_list, char!(')'))
    )
);

named!(pub variable_list<&[u8], Expr>,
    map!(ws!(many1!(identifier)), Expr::VariableList)
);

#[cfg(test)]
mod tests {

    use super::*;
    use testing::parse;

    #[test]
    pub fn parse_unary_with_binary_expr() {
        let expected = Expr::Binary(
            Box::from(Expr::Unary(
                UnaryOp::ConditionalNot,
                Box::from(Expr::var("a")),
            )),
            BinaryOp::ConditionalOr,
            Box::from(Expr::var("b")),
        );

        let result = parse::<Expr, _>("!a || b", expr);
        assert_eq!(expected, result);
    }

    #[test]
    pub fn parse_unary_expr() {
        let expected = Expr::Unary(UnaryOp::ConditionalNot, Box::from(Expr::var("a")));
        let result = parse::<Expr, _>("!a", expr);

        assert_eq!(expected, result);
    }

    #[test]
    pub fn parse_logical_xor_expr() {
        let expected = Expr::Binary(
            Box::from(Expr::var("a")),
            BinaryOp::ConditionalXor,
            Box::from(Expr::var("b")),
        );

        let result = parse::<Expr, _>("a ^^ b", expr);
        assert_eq!(expected, result);
    }

    #[test]
    pub fn parse_logical_or_expr() {
        let expected = Expr::Binary(
            Box::from(Expr::var("a")),
            BinaryOp::ConditionalOr,
            Box::from(Expr::var("b")),
        );

        let result = parse::<Expr, _>("a || b", expr);
        assert_eq!(expected, result);
    }

    #[test]
    pub fn parse_logical_and_expr() {
        let expected = Expr::Binary(
            Box::from(Expr::var("a")),
            BinaryOp::ConditionalAnd,
            Box::from(Expr::var("b")),
        );

        let result = parse::<Expr, _>("a && b", expr);
        assert_eq!(expected, result);
    }

    #[test]
    pub fn parse_variable_list_binary_expr() {
        let expected = Expr::Binary(
            Box::from(Expr::VariableList(vec!["a".to_string(), "b".to_string()])),
            BinaryOp::BitwiseAnd,
            Box::from(Expr::VariableList(vec!["a".to_string(), "d".to_string()])),
        );

        let actual = parse::<Expr, _>("(a b) & (a d)", expr);
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn parse_variable_list_unary_and_binary_expr() {
        let expected = Expr::Binary(
            Box::from(Expr::Unary(
                UnaryOp::BitwiseNot,
                Box::from(
                    Expr::VariableList(vec!["a".to_string(), "b".to_string()]),
                ),
            )),
            BinaryOp::BitwiseXor,
            Box::from(Expr::var("c")),
        );

        let actual = parse::<Expr, _>("~(a b) ^ c", expr);
        assert_eq!(expected, actual);
    }
}
