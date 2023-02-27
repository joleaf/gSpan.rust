use crate::models::edge::Edge;

pub struct PDFS<'a> {
    pub id: usize,
    pub edge: &'a Edge,
    pub prev: Option<Box<&'a PDFS<'a>>>,
}

impl<'a> PDFS<'a> {
    pub fn new(id: usize, edge: &'a Edge, prev: Option<&'a PDFS<'a>>) -> PDFS<'a> {
        PDFS {
            id,
            edge,
            prev: match prev {
                Some(prev) => Some(Box::new(prev)),
                None => None
            }
        }
    }
}
