use crate::traits::Search;
use tree_sitter::Node;

impl<'a> Search<'a> for Node<'a> {
    fn first_occurence(&self, pred: fn(u16) -> bool) -> Option<Node<'a>> {
        let mut cursor = self.walk();
        let mut stack = Vec::new();
        let mut children = Vec::new();

        stack.push(*self);

        while let Some(node) = stack.pop() {
            if pred(node.kind_id()) {
                return Some(node);
            }
            cursor.reset(node);
            if cursor.goto_first_child() {
                loop {
                    children.push(cursor.node());
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
                for child in children.drain(..).rev() {
                    stack.push(child);
                }
            }
        }

        None
    }
}