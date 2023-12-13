use debug_cell::{Ref, RefCell, RefMut};

use id_arena::{Arena, Id};

use crate::ast::{Node, NodeInterface};

#[derive(Default)]
pub struct AllArenas {
    nodes: RefCell<Arena<Node>>,
}

impl AllArenas {
    pub fn alloc_node(&self, node: Node) -> Id<Node> {
        let id = self.nodes.borrow_mut().alloc(node);
        self.node_mut(id).set_arena_id(id);
        id
    }

    pub fn node(&self, node: Id<Node>) -> Ref<Node> {
        Ref::map(self.nodes.borrow(), |nodes| &nodes[node])
    }

    pub fn node_mut(&self, node: Id<Node>) -> RefMut<Node> {
        RefMut::map(self.nodes.borrow_mut(), |nodes| &mut nodes[node])
    }
}
