use crate::models::edge::Edge;
use crate::models::graph::Graph;
use crate::models::history::History;
use crate::models::vertex::Vertex;

pub fn get_forward_root<'a>(g: &Graph, v: &'a Vertex, result: &mut Vec<&'a Edge>) -> bool {
    result.clear();
    for edge in &v.edges {
        if v.label <= g.vertices.get(edge.to).unwrap().label {
            result.push(edge);
        }
    }
    !result.is_empty()
}

pub fn get_backward<'a, 'b>(g: &'a Graph, e1: &'a Edge, e2: &'a Edge, history: &'b History) -> Option<&'a Edge> {
    if e1 == e2 {
        return None;
    }
    for edge in &g.vertices.get(e2.to).unwrap().edges {
        if history.has_edge(&edge.id) {
            continue;
        }
        if (edge.to == e1.from) && ((e1.e_label == edge.e_label)
            || (e1.e_label == edge.e_label) && (g.vertices.get(e1.to).unwrap().label <= g.vertices.get(e2.to).unwrap().label)) {
            return Some(&edge);
        }
    }
    return None;
}

pub fn get_forward_pure<'a, 'b>(g: &'a Graph, e: &'a Edge, min_label: isize, history: &'b History, result: &mut Vec<&'a Edge>) -> bool {
    result.clear();
    for edge in &g.vertices.get(e.to).unwrap().edges {
        if min_label > g.vertices.get(edge.to).unwrap().label || history.has_vertex(&edge.to) {
            continue;
        }
        result.push(&edge);
    }
    !result.is_empty()
}

pub fn get_forward_rm_path<'a, 'b>(g: &'a Graph, e: &'a Edge, min_label: isize, history: &'b History, result: &mut Vec<&'a Edge>) -> bool {
    result.clear();
    let to_label = g.vertices.get(e.to).unwrap().label;
    for edge in &g.vertices.get(e.from).unwrap().edges {
        let to_label_2 = g.vertices.get(edge.to).unwrap().label;
        if e.to == edge.to || min_label > to_label_2 || history.has_vertex(&edge.to) {
            continue;
        }
        if e.e_label < edge.e_label || (e.e_label == edge.e_label && to_label <= to_label_2) {
            result.push(&edge);
        }
    }
    !result.is_empty()
}