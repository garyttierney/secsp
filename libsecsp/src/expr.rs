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
        n: alt!(
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
    logical_not_expr,
    BinaryOp::ConditionalAnd,
    "&&"
);

named!(logical_not_expr<&[u8], Expr>,
    ws!(
        alt!(
            do_parse!(
                tag!("!") >>
                expr: expr >>

                (Expr::Unary(UnaryOp::ConditionalNot, Box::new(expr)))
            ) |
            primary_expr
        )
    )
);

named!(primary_expr<&[u8], Expr>, alt!(variable));