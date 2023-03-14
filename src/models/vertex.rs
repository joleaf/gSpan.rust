use crate::models::edge::Edge;

#[derive(Debug)]
pub struct Vertex {
    pub id: usize,
    pub label: isize,
    pub edges: Vec<Edge>,
}

impl Vertex {
    pub fn new(id: usize, label: Option<isize>) -> Vertex {
        Vertex {
            id,
            label: match label {
                None => 0,
                Some(label) => label,
            },
            edges: Vec::with_capacity(8),
        }
    }

    pub fn push(&mut self, to: usize, e_label: usize) {
        self.edges.push(Edge::new(self.id, to, e_label));
    }

    pub fn to_str_repr(&self) -> String {
        vec!["v".to_string(), self.id.to_string(), self.label.to_string()].join(" ")
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vertex() {
        let v1 = Vertex::new(1, None);
        assert_eq!(v1.id, 1);
    }

    #[test]
    fn test_add_edge() {
        let mut v1 = Vertex::new(1, Some(2));
        assert_eq!(v1.edges.len(), 0);
        assert_eq!(v1.label, 2);
        v1.push(2, 2);
        assert_eq!(v1.edges.len(), 1);
        let e = v1.edges.pop().unwrap();
        assert_eq!(v1.edges.len(), 0);
        assert_eq!(e.to, 2);
        assert_eq!(e.from, 1);
        assert_eq!(e.e_label, 2);
    }
}
