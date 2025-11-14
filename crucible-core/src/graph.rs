//! Dependency graph operations

use crate::types::Module;
use petgraph::graph::DiGraph;
use petgraph::algo::is_cyclic_directed;
use std::collections::HashMap;

/// Build a dependency graph from modules
pub fn build_dependency_graph(modules: &[Module]) -> DiGraph<String, ()> {
    let mut graph = DiGraph::new();
    let mut node_map = HashMap::new();

    // Add nodes
    for module in modules {
        let node = graph.add_node(module.module.clone());
        node_map.insert(module.module.clone(), node);
    }

    // Add edges
    for module in modules {
        if let Some(from_node) = node_map.get(&module.module) {
            for (dep_name, _) in &module.dependencies {
                if let Some(to_node) = node_map.get(dep_name) {
                    graph.add_edge(*from_node, *to_node, ());
                }
            }
        }
    }

    graph
}

/// Detect if there are cycles in the dependency graph
pub fn detect_cycles(graph: &DiGraph<String, ()>) -> bool {
    is_cyclic_directed(graph)
}
