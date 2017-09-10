//! Parser for SELinux security attributes, and abstractions new to the CIL framework.

use ast::*;
use name::*;

named!(pub level<&[u8], Expr>,
    do_parse!(
        sensitivity: identifier >>
        tag!(":") >>
        categories: category_range_or_id >>

        (Expr::Level {
            sensitivity,
            categories: Box::new(categories)
        })
    ) 
);

named!(level_or_id<&[u8], Expr>, alt_complete!(level | variable));

named!(pub level_range<&[u8], Expr>,
    ws!(do_parse!(
        range: separated_pair!(level_or_id, tag!("-"), level_or_id) >>

        (Expr::LevelRange(
            Box::new(range.0), Box::new(range.1)
        ))
    )) 
);

named!(category_range_or_id<&[u8], Expr>, alt_complete!(category_range | variable));

named!(pub category_range<&[u8], Expr>,
    ws!(do_parse!(
        range: separated_pair!(identifier, tag!("."), identifier) >>

        (Expr::CategoryRange(
            range.0, range.1
        ))
    )) 
);

named!(pub context<&[u8], Expr>,
  ws!(do_parse!(
      user_id: identifier >>
      char!(':') >>
      role_id: identifier >>
      char!(':') >>
      type_id: identifier >>
      level_range: opt!(complete!(preceded!(char!(':'), level_range))) >>
      
      (Expr::Context {
        user_id, 
        role_id,
        type_id,
        level_range: level_range.map(|v| Box::new(v))
      })
  ))
);

#[cfg(test)]
mod tests {
    use super::*;
    use testing::parse;

    #[test]
    pub fn parse_context_expr() {
        let result = parse::<Expr, _>("user:role:type", context);

        match result {
            Expr::Context {
                user_id,
                role_id,
                type_id,
                level_range,
            } => {
                assert_eq!("user", user_id);
                assert_eq!("role", role_id);
                assert_eq!("type", type_id);
                assert_eq!(true, level_range.is_none());
            }
            _ => panic!("Invalid value parsed"),
        }
    }

    #[test]
    pub fn parse_levelrange_expr() {
        let result = parse::<Expr, _>("s0-s1", level_range);

        if let Expr::LevelRange(low, high) = result {
            assert_eq!(Expr::var("s0"), *low);
            assert_eq!(Expr::var("s1"), *high);
        } else {
            panic!("Invalid value parsed");
        }
    }

    #[test]
    pub fn parse_context_expr_with_levelrange() {
        let result = parse::<Expr, _>("user:role:type:s0 - s1", context);

        match result {
            Expr::Context { level_range, .. } => {
                if let &Expr::LevelRange(ref low, ref high) = level_range.unwrap().as_ref() {
                    assert_eq!(Expr::var("s0"), **low);
                    assert_eq!(Expr::var("s1"), **high);
                } else {
                    panic!("No level range found");
                }
            }
            _ => panic!("Invalid value parsed"),
        }
    }

}