use id_arena::Id;
use tracing::trace;
use tree_sitter_lint::{tree_sitter_grep::RopeOrSlice, tree_sitter::{Tree, Node}};

use super::{scope::_Scope, arenas::AllArenas};

pub struct ScopeAnalyzer<'a> {
    file_contents: RopeOrSlice<'a>,
    pub scopes: Vec<Id<_Scope<'a>>>,
    arenas: AllArenas<'a>,
}

impl<'a> ScopeAnalyzer<'a> {
    pub fn new(
        file_contents: impl Into<RopeOrSlice<'a>>,
    ) -> Self {
        let file_contents = file_contents.into();

        Self {
            file_contents,
            scopes: Default::default(),
            arenas: Default::default(),
        }
    }

    pub fn visit(&mut self, node: Node<'a>) {
        trace!(?node, "visiting node");

        match node.kind() {
            _ => self.visit_children(node),
        }
    }

    fn visit_children(&mut self, node: Node<'a>) {
        let mut cursor = node.walk();
        for child in node.named_children(&mut cursor) {
            self.visit(child);
        }
    }
}
