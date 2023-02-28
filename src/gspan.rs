use crate::misc::{get_backward, get_forward_pure, get_forward_rm_path, get_forward_root};
use crate::models::dfs_code::DFSCode;
use crate::models::edge::Edge;
use crate::models::graph::Graph;
use crate::models::history::History;
use crate::models::projected::Projected;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct GSpanConfig {
    out_path: String,
    trans: Vec<Graph>,
    min_sup: usize,
    max_pat_min: usize,
    max_pat_max: usize,
    directed: bool,
    single_nodes: bool,
}

impl GSpanConfig {
    pub fn new(
        graphs: Vec<Graph>,
        min_sup: usize,
        max_pat_min: usize,
        max_pat_max: usize,
        directed: bool,
        single_nodes: bool,
        out_path: String,
    ) -> GSpanConfig {
        GSpanConfig {
            trans: graphs,
            min_sup,
            max_pat_min,
            max_pat_max,
            directed,
            single_nodes,
            out_path,
        }
    }

    pub fn run(&self) -> usize {
        // 0. Define output file
        let mut out: BufWriter<File> = BufWriter::new(File::create(self.out_path.clone()).unwrap());
        let mut next_id: usize = 0;
        // 1. Find single node frequent subgraph, if requested
        let mut single_vertex: BTreeMap<usize, BTreeMap<isize, usize>> = BTreeMap::new();
        let mut single_vertex_label: BTreeMap<isize, usize> = BTreeMap::new();
        if self.max_pat_min <= 1 {
            for graph in &self.trans {
                for vertex in &graph.vertices {
                    let key = vertex.label;
                    let d = single_vertex.entry(graph.id).or_insert(BTreeMap::new());
                    if d.get(&key).is_none() {
                        single_vertex_label
                            .entry(key)
                            .and_modify(|v| *v += 1)
                            .or_insert(1);
                    }
                    d.entry(key).and_modify(|v| *v += 1).or_insert(1);
                }
            }
        }

        //println!("{:?}", single_vertex);
        //println!("{:?}", single_vertex_label);
        // 2. Report the single vertex subgraphs
        for (frequent_label, value) in single_vertex_label.iter() {
            if value < &self.min_sup {
                continue;
            }

            let id = next_id.clone();
            next_id += 1;
            let mut g = Graph::new(id, self.directed);
            let mut v = g.create_vertex();
            v.label = *frequent_label;

            let mut counts: Vec<usize> = Vec::new();
            counts.resize(self.trans.len(), 0);
            for (key, it2) in single_vertex.iter() {
                counts[*key] = *it2.get(&frequent_label).or(Some(&0)).unwrap();
            }

            // Report single-graphs
            self.report_single(&mut out, &mut g, counts.iter().sum());
        }
        // 3. Subgraphs > Verticies
        let mut root: BTreeMap<isize, BTreeMap<usize, BTreeMap<isize, Projected>>> =
            BTreeMap::new();
        let mut edges: Vec<&Edge> = Vec::new();
        for g in &self.trans {
            for from in &g.vertices {
                if get_forward_root(&g, from, &mut edges) {
                    for edge in &edges {
                        let key_1 = from.label;
                        let root_1 = root.entry(key_1).or_insert(BTreeMap::new());
                        let key_2 = edge.e_label;
                        let root_2 = root_1.entry(key_2).or_insert(BTreeMap::new());
                        let key_3 = g.vertices.get(edge.to).unwrap().label;
                        let root_3 = root_2.entry(key_3).or_insert(Projected::new());
                        root_3.push(g.id, edge, None);
                    }
                }
            }
        }

        let mut dfs_code = DFSCode::new();
        for (from_label_key, from_label) in root.iter() {
            for (e_label_key, e_label) in from_label.iter() {
                for (to_label_key, to_label) in e_label.iter() {
                    dfs_code.push(0, 1, *from_label_key, *e_label_key, *to_label_key);
                    self.project(to_label, &mut dfs_code, &mut next_id, &mut out);
                    dfs_code.pop();
                }
            }
        }
        return next_id;
    }

    fn report_single(
        &self,
        out: &mut BufWriter<File>,
        g: &mut Graph,
        sup: usize,
    ) {
        if self.max_pat_max >= self.max_pat_min && g.vertices.len() > self.max_pat_max {
            return;
        }
        if self.max_pat_min > 0 && g.vertices.len() < self.max_pat_min {
            return;
        }
        out.write(&*g.to_str_repr(Some(sup)).into_bytes());
        out.write(b"\n");
    }

    fn report(
        &self,
        sup: usize,
        dfs_code: &DFSCode,
        next_id: &mut usize,
        out: &mut BufWriter<File>,
    ) {
        if self.max_pat_max >= self.max_pat_min && dfs_code.count_node() > self.max_pat_max {
            return;
        }
        if self.max_pat_min > 0 && dfs_code.count_node() < self.max_pat_min {
            return;
        }
        let id = next_id.clone();
        *next_id += 1;
        let mut g = Graph::new(id, self.directed);
        dfs_code.to_graph(&mut g, self.single_nodes);
        out.write(&*g.to_str_repr(Some(sup)).into_bytes());
        out.write(b"\n");
    }

    fn project(
        &self,
        projected: &Projected,
        dfs_code: &mut DFSCode,
        next_id: &mut usize,
        out: &mut BufWriter<File>,
    ) {
        // Check if the pattern is frequent enough
        let sup: usize = self.support(projected);
        if sup < self.min_sup {
            return;
        }
        // Check if the pattern is not min
        if !self.is_min(dfs_code) {
            return;
        }

        // Output the frequent substructures
        self.report(sup, dfs_code, next_id, out);

        /*
         * In case we have a valid upper bound and our graph already exceeds it,
         * return. Note: we do not check for equality as the DFS exploration may
         * still add edges within an existing sub-graph, without increasing the
         * number of nodes.
         */
        if self.max_pat_max >= self.max_pat_min && dfs_code.count_node() > self.max_pat_max {
            return;
        }

        /*
         * We just outputted a frequent sub-graph. As it is frequent enough, so
         * might be its (n+1)-extension-graphs, hence we enumerate them all.
         */

        let rm_path = dfs_code.build_rm_path();
        let min_label = dfs_code.dfs_vec.get(0).unwrap().from_label;
        let max_toc = dfs_code.dfs_vec.get(*rm_path.get(0).unwrap()).unwrap().to;

        let mut new_fwd_root: BTreeMap<usize, BTreeMap<usize, BTreeMap<isize, Projected>>> =
            BTreeMap::new();
        let mut new_bck_root: BTreeMap<usize, BTreeMap<usize, Projected>> = BTreeMap::new();
        let mut edges: Vec<&Edge> = Vec::new();

        // Enumerate all possible one edge extensions of the current substructure.
        for a_projected in projected.projections.iter() {
            let id = a_projected.id;
            let history = History::build(a_projected);

            // backward
            for i in (0..rm_path.len()).rev() {
                let e = get_backward(
                    self.trans.get(id).unwrap(),
                    history.histories.get(*rm_path.get(0).unwrap()).unwrap(),
                    history.histories.get(*rm_path.get(i).unwrap()).unwrap(),
                    &history,
                );
                if let Some(e) = e {
                    let key_1 = dfs_code.dfs_vec.get(*rm_path.get(i).unwrap()).unwrap().from;
                    let root_1 = new_bck_root.entry(key_1).or_insert(BTreeMap::new());
                    let key_2 = e.e_label;
                    let root_2 = root_1.entry(key_2).or_insert(Projected::new());
                    root_2.push(id, e, Some(&a_projected));
                }
            }
            // pure forward
            if get_forward_pure(
                self.trans.get(id).unwrap(),
                history.histories.get(*rm_path.get(0).unwrap()).unwrap(),
                min_label,
                &history,
                &mut edges,
            ) {
                for it in &edges {
                    let root_1 = new_fwd_root.entry(max_toc).or_insert(BTreeMap::new());
                    let key_2 = it.e_label;
                    let root_2 = root_1.entry(key_2).or_insert(BTreeMap::new());
                    let key_3 = self
                        .trans
                        .get(id)
                        .unwrap()
                        .vertices
                        .get(it.to)
                        .unwrap()
                        .label;
                    let root_3 = root_2.entry(key_3).or_insert(Projected::new());
                    root_3.push(id, it, Some(&a_projected));
                }
            }
            // backtracked forward
            for a_rm_path in &rm_path {
                if get_forward_rm_path(
                    self.trans.get(id).unwrap(),
                    history.histories.get(*a_rm_path).unwrap(),
                    min_label,
                    &history,
                    &mut edges,
                ) {
                    for it in &edges {
                        let key_1 = dfs_code.dfs_vec.get(*a_rm_path).unwrap().from;
                        let root_1 = new_fwd_root.entry(key_1).or_insert(BTreeMap::new());
                        let key_2 = it.e_label;
                        let root_2 = root_1.entry(key_2).or_insert(BTreeMap::new());
                        let key_3 = self
                            .trans
                            .get(id)
                            .unwrap()
                            .vertices
                            .get(it.to)
                            .unwrap()
                            .label;
                        let root_3 = root_2.entry(key_3).or_insert(Projected::new());
                        root_3.push(id, it, Some(a_projected));
                    }
                }
            }
        }
        // Test all extended substructures..
        // .. backward
        for (to_key, to) in new_bck_root.iter() {
            for (e_label_key, e_label) in to.iter() {
                dfs_code.push(max_toc, *to_key, -1, *e_label_key, -1);
                self.project(e_label, dfs_code, next_id, out);
                dfs_code.pop();
            }
        }
        // .. forward
        for (from_key, from) in new_fwd_root.iter().rev() {
            for (e_label_key, e_label) in from.iter() {
                for (to_label_key, to_label) in e_label.iter() {
                    dfs_code.push(*from_key, max_toc + 1, -1, *e_label_key, *to_label_key);
                    self.project(to_label, dfs_code, next_id, out);
                    dfs_code.pop();
                }
            }
        }
    }

    fn support(&self, projected: &Projected) -> usize {
        let mut oid = usize::MAX;
        let mut size = 0;

        for cur in projected.projections.iter() {
            if oid != cur.id {
                size += 1;
            }
            oid = cur.id;
        }
        size
    }

    fn is_min(&self, dfs_code: &mut DFSCode) -> bool {
        if dfs_code.dfs_vec.len() == 1 {
            return true;
        }

        let mut graph_is_min = Graph::new(0, self.directed);
        dfs_code.to_graph(&mut graph_is_min, self.single_nodes);

        let mut dfs_code_is_min = DFSCode::new();

        let mut root: BTreeMap<isize, BTreeMap<usize, BTreeMap<isize, Projected>>> =
            BTreeMap::new();
        let mut edges: Vec<&Edge> = Vec::new();

        for from in &graph_is_min.vertices {
            if get_forward_root(&graph_is_min, from, &mut edges) {
                for it in &edges {
                    let key_1 = from.label;
                    let root_1 = root.entry(key_1).or_insert(BTreeMap::new());
                    let key_2 = it.e_label;
                    let root_2 = root_1.entry(key_2).or_insert(BTreeMap::new());
                    let key_3 = graph_is_min.vertices.get(it.to).unwrap().label;
                    let root_3 = root_2.entry(key_3).or_insert(Projected::new());
                    root_3.push(graph_is_min.id, it, None);
                }
            }
        }

        let from_label_binding = root.first_key_value().unwrap();
        let from_label = from_label_binding.1;
        let e_label_binding = from_label.first_key_value().unwrap();
        let e_label = e_label_binding.1;
        let to_label_binding = e_label.first_key_value().unwrap();
        let to_label = to_label_binding.1;
        dfs_code_is_min.push(
            0,
            1,
            *from_label_binding.0,
            *e_label_binding.0,
            *to_label_binding.0,
        );
        self.is_min_project(to_label, dfs_code, &mut dfs_code_is_min, &graph_is_min)
    }

    fn is_min_project(
        &self,
        projected: &Projected,
        dfs_code: &DFSCode,
        dfs_code_is_min: &mut DFSCode,
        graph_is_min: &Graph,
    ) -> bool {
        let rm_path = dfs_code_is_min.build_rm_path();
        let min_label = dfs_code_is_min.dfs_vec.get(0).unwrap().from_label;
        let max_toc: usize = dfs_code_is_min
            .dfs_vec
            .get(*rm_path.get(0).unwrap())
            .unwrap()
            .to;

        {
            let mut root: BTreeMap<usize, Projected> = BTreeMap::new();
            let mut new_to: usize = 0;
            let mut flg = false;
            for i in (1..rm_path.len()).rev() {
                for cur in projected.projections.iter() {
                    let cur = &**cur;
                    let history: History = History::build(cur);
                    let e = get_backward(
                        graph_is_min,
                        history.histories.get(*rm_path.get(i).unwrap()).unwrap(),
                        history.histories.get(*rm_path.get(0).unwrap()).unwrap(),
                        &history,
                    );
                    if let Some(e) = e {
                        let key_1 = e.e_label;
                        let root_1: &mut Projected = root.entry(key_1).or_insert(Projected::new());
                        new_to = dfs_code_is_min
                            .dfs_vec
                            .get(*rm_path.get(i).unwrap())
                            .unwrap()
                            .from;
                        root_1.push(key_1, e, Some(cur));
                        flg = true;
                    }
                }
                if flg {
                    break;
                }
            }
            if flg {
                let e_label = root.first_entry().unwrap();
                dfs_code_is_min.push(max_toc, new_to, -1, *e_label.key(), -1);
                if dfs_code
                    .dfs_vec
                    .get(dfs_code_is_min.dfs_vec.len() - 1)
                    .unwrap()
                    .ne(dfs_code_is_min
                        .dfs_vec
                        .get(dfs_code_is_min.dfs_vec.len() - 1)
                        .unwrap())
                {
                    return false;
                }
                return self.is_min_project(e_label.get(), dfs_code, dfs_code_is_min, graph_is_min);
            }
        }

        {
            let mut flg = false;
            let mut new_from = 0;
            let mut root: BTreeMap<usize, BTreeMap<isize, Projected>> = BTreeMap::new();
            let mut edges: Vec<&Edge> = Vec::new();

            for cur in projected.projections.iter() {
                let history: History = History::build(cur);
                if get_forward_pure(
                    graph_is_min,
                    history.histories.get(*rm_path.get(0).unwrap()).unwrap(),
                    min_label,
                    &history,
                    &mut edges,
                ) {
                    flg = true;
                    new_from = max_toc;
                    for it in &edges {
                        let key_1 = it.e_label;
                        let root_1 = root.entry(key_1).or_insert(BTreeMap::new());
                        let key_2 = graph_is_min.vertices.get(it.to).unwrap().label;
                        let root_2 = root_1.entry(key_2).or_insert(Projected::new());
                        root_2.push(0, it, Some(cur));
                    }
                }
            }
            if !flg {
                for i in 0..rm_path.len() {
                    for cur in projected.projections.iter() {
                        let history: History = History::build(cur);
                        if get_forward_rm_path(
                            graph_is_min,
                            history.histories.get(*rm_path.get(i).unwrap()).unwrap(),
                            min_label,
                            &history,
                            &mut edges,
                        ) {
                            flg = true;
                            new_from = dfs_code_is_min
                                .dfs_vec
                                .get(*rm_path.get(i).unwrap())
                                .unwrap()
                                .from;
                            for it in &edges {
                                let key_1 = it.e_label;
                                let root_1 = root.entry(key_1).or_insert(BTreeMap::new());
                                let key_2 = graph_is_min.vertices.get(it.to).unwrap().label;
                                let root_2 = root_1.entry(key_2).or_insert(Projected::new());
                                root_2.push(0, it, Some(cur));
                            }
                        }
                    }
                    if flg {
                        break;
                    }
                }
            }
            if flg {
                let e_label_binding = root.first_key_value().unwrap();
                let e_label = e_label_binding.1;
                let to_label_binding = e_label.first_key_value().unwrap();
                let to_label = to_label_binding.1;
                dfs_code_is_min.push(
                    new_from,
                    max_toc + 1,
                    -1,
                    *e_label_binding.0,
                    *to_label_binding.0,
                );
                if dfs_code
                    .dfs_vec
                    .get(dfs_code_is_min.dfs_vec.len() - 1)
                    .unwrap()
                    .ne(dfs_code_is_min.dfs_vec.last().unwrap())
                {
                    return false;
                }
                return self.is_min_project(to_label, dfs_code, dfs_code_is_min, graph_is_min);
            }
        }
        true
    }
}
