use crate::ast::types::{Block, Definition};
use crate::ast::{AstChildren, AstNode};

pub trait ItemOwner: AstNode {
    fn items(&self) -> AstChildren<Definition> {
        self.child::<Block>().children()
    }

    fn items_of<T: AstNode>(&self) -> AstChildren<T> {
        self.child::<Block>().children()
    }
}

pub trait NameOwner: AstNode {}

#[cfg(test)]
mod tests {
    use crate::ast::SourceFile;

    use super::*;

    fn parse_item<T: AstNode>(text: &str) -> T {
        SourceFile::parse(text)
            .tree()
            .items_of::<T>()
            .nth(0)
            .unwrap()
    }

    //    #[test]
    //    fn variable_as_name_owner() {
    //        test_name_owner::<VariableDef>("type t;", "t");
    //    }
    //
    //    #[test]
    //    fn variable_with_initializer_as_name_owner() {
    //        test_name_owner::<VariableDef>("type_attribute t = v;", "t");
    //    }
    //
    //    #[test]
    //    fn macro_as_name_owner() {
    //        test_name_owner::<MacroDef>("macro abc() {}", "abc");
    //    }
    //
    //    #[test]
    //    fn block_as_name_owner() {
    //        test_name_owner::<ContainerDef>("block abc {}", "abc");
    //    }
}
