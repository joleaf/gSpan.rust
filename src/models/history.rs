use crate::models::edge::Edge;
use crate::models::pdfs::PDFS;
use std::borrow::Borrow;
use std::collections::HashSet;

pub struct History<'a> {
    pub histories: Vec<&'a Edge>,
    pub edges: HashSet<usize>,
    pub vertices: HashSet<usize>,
}

impl<'a> History<'a> {
    pub fn build(e: &'a PDFS<'a>) -> History<'a> {
        let mut history = History {
            histories: Vec::new(),
            edges: HashSet::new(),
            vertices: HashSet::new(),
        };
        let mut e = e.borrow();
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
