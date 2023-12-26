// use petgraph::dot::{Config, Dot};
use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::petgraph::graph::{NodeIndex, UnGraph};
use rustworkx_core::Result;
use std::collections::HashMap;
// use std::io::Write;
use std::time::Instant;
use utils::timed_execution::TimedExecution;

fn main() {
    let mut graph: UnGraph<&str, u32> = UnGraph::new_undirected();
    let mut nodes: HashMap<&str, NodeIndex> = HashMap::new();
    Instant::timed(|| {
        include_str!("input.txt")
            .lines()
            .map(|ln| ln.split_at(3))
            .for_each(|(left, r)| {
                let right: Vec<&str> = r.split_ascii_whitespace().skip(1).collect();
                for n in right.iter() {
                    let l_node = *nodes.entry(left).or_insert_with(|| graph.add_node(left));
                    let r_node = *nodes.entry(n).or_insert_with(|| graph.add_node(n));
                    graph.add_edge(l_node, r_node, 1);
                    graph.add_edge(r_node, l_node, 1);
                }
            });
        let min_cut: rustworkx_core::Result<Option<(usize, Vec<_>)>> =
            stoer_wagner_min_cut(&graph, |_| Result::Ok(1));

        let (_cut_size, nodes_in_partition) = min_cut.unwrap().unwrap();
        let total_nodes = nodes.len();

        println!(
            "{}",
            (total_nodes - nodes_in_partition.len()) * nodes_in_partition.len()
        );
    });
}

//Optionaly create grphviz UI dot -Tsvg graph.dot > out.svg
// let dot_txt = format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
// let mut file = std::fs::File::create("graph.dot").expect("Failed to create file.");
// let _ = file.write_all(dot_txt.as_bytes());
