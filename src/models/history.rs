use crate::models::edge::Edge;
use crate::models::pdfs::PDFS;
use rustc_hash::FxHashSet;

pub struct History<'a> {
    pub histories: Vec<&'a Edge>,
    pub edges: FxHashSet<usize>,
    pub vertices: FxHashSet<usize>,
}

impl<'a> History<'a> {
    pub fn build(e: &'a PDFS<'a>) -> History<'a> {
        let mut history = History {
            histories: Vec::with_capacity(32),
            edges: FxHashSet::default(),
            vertices: FxHashSet::default(),
        };
        let mut e = e;
        loop {
            history.histories.push(e.edge);
            history.edges.insert(e.edge.id);
            history.vertices.insert(e.edge.from);
            history.vertices.insert(e.edge.to);
            if e.prev.is_none() {
                break;
            }
            e = e.prev.as_ref().unwrap()
        }
        history.histories.reverse();
        history
    }

    pub fn has_edge(&self, id: &usize) -> bool {
        self.edges.contains(&id)
    }

    pub fn has_vertex(&self, id: &usize) -> bool {
        self.vertices.contains(&id)
    }
}
