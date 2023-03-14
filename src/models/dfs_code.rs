use crate::models::dfs::DFS;
use crate::models::graph::Graph;
use std::cmp;
use std::cmp::max;

pub struct DFSCode {
    pub dfs_vec: Vec<DFS>,
}

impl DFSCode {
    pub fn new() -> DFSCode {
        DFSCode {
            dfs_vec: Vec::with_capacity(32),
        }
    }

    pub fn push(
        &mut self,
        from: usize,
        to: usize,
        from_label: isize,
        e_label: usize,
        to_label: isize,
    ) {
        self.dfs_vec
            .push(DFS::from(from, to, from_label, e_label, to_label))
    }

    pub fn pop(&mut self) -> Option<DFS> {
        return self.dfs_vec.pop();
    }

    pub fn to_graph(&self, g: &mut Graph, single_nodes: bool) {
        // Version 1: Multiple nodes for nodes with same label
        if !single_nodes {
            for it in &self.dfs_vec {
                g.resize(cmp::max(it.from, it.to) + 1);

                if it.from_label != -1 {
                    g.vertices.get_mut(it.from).unwrap().label = it.from_label;
                }
                if it.to_label != -1 {
                    g.vertices.get_mut(it.to).unwrap().label = it.to_label;
                }
                g.vertices.get_mut(it.from).unwrap().push(it.to, it.e_label);
                if !g.directed {
                    g.vertices.get_mut(it.to).unwrap().push(it.from, it.e_label);
                }
            }
        } else {
            // Version 2: One node for nodes with the same label
            // Create label mapping
            todo!()
        }
        g.build_edge();
    }

    pub fn build_rm_path(&self) -> Vec<usize> {
        let mut rm_path: Vec<usize> = Vec::new();
        let mut old_from = usize::MAX;
        for i in (0..self.dfs_vec.len()).rev() {
            let dfs = self.dfs_vec.get(i).unwrap();
            if dfs.from < dfs.to && (rm_path.is_empty() || old_from == dfs.to) {
                rm_path.push(i);
                old_from = dfs.from;
            }
        }
        rm_path
    }

    pub fn count_node(&self) -> usize {
        let mut count = 0;
        for dfs in &self.dfs_vec {
            count = max(count, max(dfs.from, dfs.to) + 1);
        }
        count
    }
}
