use id_arena::{Arena, Id};

use crate::ast::Node;

pub struct AllArenas<'a> {
    nodes: Arena<Node<'a>>,
}

impl<'a> AllArenas<'a> {
    pub fn alloc_node(&mut self, node: Node<'a>) -> Id<Node<'a>> {
        self.nodes.alloc(node)
    }

    pub fn node(&self, node: Id<Node<'a>>) -> &Node<'a> {
        &self.nodes[node]
    }
}
