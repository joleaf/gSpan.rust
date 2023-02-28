use std::{fmt, io};
use std::fs::{File};
use std::io::BufRead;
use std::path::Path;
use crate::models::edge::Edge;
use crate::models::vertex::Vertex;

#[derive(Debug)]
pub struct GraphSetParseError {
    message: String,
}

impl fmt::Display for GraphSetParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}


#[derive(Debug)]
pub struct Graph {
    pub id: usize,
    pub edge_size: usize,
    pub directed: bool,
    pub vertices: Vec<Vertex>,
}

impl Graph {
    pub fn new(id: usize, directed: bool) -> Graph {
        Graph {
            id,
            edge_size: 0,
            directed,
            vertices: Vec::new(),
        }
    }

    pub fn create_vertex(&mut self) -> &mut Vertex {
        let vertex = Vertex::new(self.vertices.len(), None);
        self.vertices.push(vertex);
        self.get_last_vertex()
    }

    pub fn get_last_vertex(&mut self) -> &mut Vertex {
        self.vertices.last_mut().unwrap()
    }

    pub fn resize(&mut self, size: usize) {
        while &self.vertices.len() < &size {
            self.create_vertex();
        }
    }

    pub fn has_vertex_with_id(&self, id: &usize) -> bool {
        return self.vertices.len() > *id;
    }

    pub fn build_edge(&self) {
        //todo!()
    }

    pub fn graphs_set_from_file<P>(path: P, directed: bool) -> Result<Vec<Graph>, GraphSetParseError>
        where P: AsRef<Path>, {
        let mut graph_list = Vec::new();
        let mut current_graph: Graph = Graph::new(usize::MAX, directed);
        let line_reader = read_lines(path);
        match line_reader {
            Ok(lines) => {
                for line in lines {
                    if let Ok(data_line) = line {
                        let mut data = data_line.split(" ");
                        if let Some(data_type) = data.next() {
                            match data_type {
                                "t" => {
                                    let _ = data.next().ok_or(GraphSetParseError {
                                        message: "Missing '#' in graph".to_string()
                                    })?;
                                    let id = data.next().ok_or(GraphSetParseError {
                                        message: "Id for graph is missing".to_string()
                                    })?;
                                    if id == "-1" {
                                        break;
                                    }
                                    if current_graph.id != usize::MAX {
                                        graph_list.push(current_graph);
                                    }
                                    let id = id.parse::<usize>();
                                    match id {
                                        Ok(id) => current_graph = Graph::new(id, directed),
                                        _ => return Err(GraphSetParseError { message: "Id for graph invalid".to_string() })
                                    }
                                }
                                "v" => {
                                    let id = data.next().ok_or(GraphSetParseError {
                                        message: format!("Graph {}, Missing id for a vertex in", current_graph.id.to_string()).to_string()
                                    })?;
                                    let id = id.parse::<usize>();
                                    match id {
                                        Ok(id) => {
                                            let vertex = current_graph.create_vertex();
                                            if vertex.id != id {
                                                return Err(GraphSetParseError {
                                                    message: format!("Graph {}, Vertex ID ({}) in input file does not fit the expected ID {}", current_graph.id.to_string(), id.to_string(), current_graph.get_last_vertex().id.to_string()).to_string()
                                                });
                                            }
                                            let label = data.next().ok_or(GraphSetParseError {
                                                message: format!("Graph {}, Missing label for a vertex", current_graph.id.to_string()).to_string()
                                            })?;
                                            let label = label.parse::<isize>();
                                            if label.is_err() {
                                                return Err(GraphSetParseError {
                                                    message: format!("Graph {}, Vertex {}, Label invalid", current_graph.id.to_string(), id.to_string())
                                                });
                                            }
                                            current_graph.get_last_vertex().label = label.unwrap();
                                        }
                                        _ => return Err(GraphSetParseError {
                                            message: format!("Graph {}, Vertex ID invalid", current_graph.id.to_string()).to_string()
                                        })
                                    }
                                }
                                "e" => {
                                    let from_id = data.next().ok_or(GraphSetParseError {
                                        message: format!("Graph {}, Missing from id for an edge", current_graph.id.to_string()).to_string()
                                    })?;
                                    let from_id: usize = match from_id.parse() {
                                        Ok(value) => value,
                                        _ => return Err(GraphSetParseError {
                                            message: format!("Graph {}, Invalid from id for an edge", current_graph.id.to_string()).to_string()
                                        })
                                    };
                                    let to_id = data.next().ok_or(GraphSetParseError {
                                        message: format!("Graph {}, Missing to id for a edge in", current_graph.id.to_string()).to_string()
                                    })?;
                                    let to_id: usize = match to_id.parse() {
                                        Ok(value) => value,
                                        _ => return Err(GraphSetParseError {
                                            message: format!("Graph {}, Invalid to id for a edge", current_graph.id.to_string()).to_string()
                                        })
                                    };
                                    let e_label = data.next().ok_or(GraphSetParseError {
                                        message: format!("Graph {}, Missing edge label for a edge", current_graph.id.to_string()).to_string()
                                    })?;
                                    let e_label: usize = match e_label.parse() {
                                        Ok(value) => value,
                                        _ => return Err(GraphSetParseError {
                                            message: format!("Graph {}, Invalid e_label for a edge", current_graph.id.to_string()).to_string()
                                        })
                                    };

                                    if !current_graph.has_vertex_with_id(&from_id) || !current_graph.has_vertex_with_id(&to_id) {
                                        return Err(GraphSetParseError {
                                            message: format!("Graph {}, Edge invalid, ids of vertices not found", current_graph.id.to_string()).to_string()
                                        });
                                    }

                                    let from_vertex: Option<&mut Vertex> = current_graph.vertices.get_mut(from_id);
                                    match from_vertex {
                                        Some(from_vertex) => {
                                            from_vertex.push(to_id, e_label);
                                        }
                                        _ => return Err(GraphSetParseError {
                                            message: format!("Graph {}, Edge invalid, ids of vertices not found", current_graph.id.to_string()).to_string()
                                        })
                                    }
                                    if directed {
                                        let from_vertex: Option<&mut Vertex> = current_graph.vertices.get_mut(to_id);
                                        match from_vertex {
                                            Some(from_vertex) => {
                                                from_vertex.push(from_id, e_label);
                                            }
                                            _ => return Err(GraphSetParseError {
                                                message: format!("Graph {}, Edge invalid, ids of vertices not found", current_graph.id.to_string()).to_string()
                                            })
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            Err(_) => return Err(GraphSetParseError { message: "Error reading file".to_string() })
        }
        if current_graph.id != usize::MAX {
            graph_list.push(current_graph);
        }
        Ok(graph_list)
    }

    pub fn to_str_repr(&self, support: Option<usize>) -> String {
        let mut lines: Vec<String> = Vec::new();
        let mut g_rep = format!("t # {}", self.id.to_string());
        if let Some(support) = support {
            g_rep += &*format!(" * {}", support);
        }
        lines.push(g_rep);
        let mut edges: Vec<&Edge> = Vec::new();
        for vertex in &self.vertices {
            lines.push(vertex.to_str_repr());
            edges.extend(vertex.edges.iter());
        }
        for edge in edges {
            lines.push(edge.to_str_repr());
        }
        lines.join("\n")
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
