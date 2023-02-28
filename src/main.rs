use crate::gspan::GSpanConfig;
use crate::models::graph::Graph;
use std::time::Instant;

mod gspan;
mod misc;
pub mod models;

use clap::Parser;

/// Fast Rust implementation for gSpan
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file with the graph database
    #[arg(short, long)]
    input: String,

    /// Output file for the resulting subgraphs
    #[arg(short, long, default_value = "out.txt")]
    output: String,

    /// Min support
    #[arg(short, long, default_value_t = 2)]
    support: usize,

    /// Minimum number of vertices
    #[arg(long, default_value_t = 1)]
    min_vertices: usize,

    /// Maximum number of vertices
    #[arg(long, default_value_t = 10)]
    max_vertices: usize,

    /// The graphs are directed
    #[arg(short, long, default_value_t = false)]
    directed: bool,
}

fn main() {
    let args = Args::parse();

    println!("gSpan Subgraph Mining");
    println!("---------------------");
    println!("Using arguments:");
    println!("{:?}", args);
    let now = Instant::now();
    let graphs = Graph::graphs_set_from_file(args.input, args.directed);
    match graphs {
        Ok(ref graphs) => {
            println!("All good parsing input file, found {} graphs", graphs.len());
        }
        Err(err) => panic!("{}", err.to_string()),
    }
    let graphs = graphs.unwrap();
    println!("Mining subgraphs..");
    let gspan = GSpanConfig::new(
        graphs,
        args.support,
        args.min_vertices,
        args.max_vertices,
        args.directed,
        false,
        args.output,
    );
    let subgraphs = gspan.run();
    let delta = now.elapsed().as_millis();
    println!("Finished.");
    println!("Found {} subgraphs", subgraphs);
    println!("Took {}ms", delta);
}
