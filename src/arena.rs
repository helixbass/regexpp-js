use id_arena::{Arena, Id};

use crate::ast::{Node, NodeInterface};

#[derive(Default)]
pub struct AllArenas<'a> {
    nodes: Arena<Node<'a>>,
}

impl<'a> AllArenas<'a> {
    pub fn alloc_node(&mut self, node: Node<'a>) -> Id<Node<'a>> {
        let id = self.nodes.alloc(node);
        self.node_mut(id).set_arena_id(id);
        id
    }

    pub fn node(&self, node: Id<Node<'a>>) -> &Node<'a> {
        &self.nodes[node]
    }

    pub fn node_mut(&mut self, node: Id<Node<'a>>) -> &mut Node<'a> {
        &mut self.nodes[node]
    }
}
