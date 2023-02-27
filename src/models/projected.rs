use crate::models::edge::Edge;
use crate::models::pdfs::PDFS;

pub struct Projected<'a> {
    pub projections: Vec<Box<PDFS<'a>>>,
}

impl<'a> Projected<'a> {
    pub fn new() -> Projected<'a> {
        Projected {
            projections: Vec::new()
        }
    }
    pub fn push(&mut self, id: usize, edge: &'a Edge, prev: Option<&'a PDFS<'a>>) {
        let new_pdfs = PDFS::new(id, edge, prev);
        self.projections.push(Box::new(new_pdfs));
    }
}