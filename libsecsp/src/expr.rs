use ast::*;
use name::*;
use security_attributes::*;

named!(pub expr<&[u8], Expr>,
  alt_complete!(
    context
    | level_range 
    | category_range
    // | binary_expr
    | unary_expr
    | variable
  )
);

named!(pub binary_operator<&[u8], BinaryOp>,
  alt!(
      value!(BinaryOp::ConditionalAnd, tag!("&&")) |
      value!(BinaryOp::ConditionalOr, tag!("||")) |
      value!(BinaryOp::BitwiseAnd, char!('&')) |
      value!(BinaryOp::BitwiseXor, char!('^')) |
      value!(BinaryOp::BitwiseOr, char!('|'))
  )
);

named!(pub binary_expr<&[u8], Expr>,
  ws!(do_parse!(
    lhs: expr >>
    op: binary_operator >> 
    rhs: expr >>
    
    (Expr::Binary(Box::new(lhs), op, Box::new(rhs)))
  ))
);

named!(pub unary_operator<&[u8], UnaryOp>,
  alt!(
      value!(UnaryOp::ConditionalNot, char!('!')) |
      value!(UnaryOp::BitwiseNot, char!('~'))
  )
);

named!(pub unary_expr<&[u8], Expr>,
  ws!(do_parse!(
    op: unary_operator >> 
    expr: expr >>
    
    (Expr::Unary(op, Box::new(expr)))
  ))
);
