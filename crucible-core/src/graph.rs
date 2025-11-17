//! Dependency graph operations

use crate::types::Module;
use petgraph::algo::is_cyclic_directed;
use petgraph::graph::DiGraph;
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
            for dep_name in module.dependencies.keys() {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_module(name: &str, deps: Vec<(&str, &str)>) -> Module {
        let mut dependencies = HashMap::new();
        for (dep_name, version) in deps {
            dependencies.insert(dep_name.to_string(), version.to_string());
        }

        Module {
            module: name.to_string(),
            version: "1.0.0".to_string(),
            layer: None,
            description: None,
            exports: HashMap::new(),
            dependencies,
        }
    }

    #[test]
    fn test_build_empty_graph() {
        let modules: Vec<Module> = vec![];
        let graph = build_dependency_graph(&modules);
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_build_single_module_no_deps() {
        let modules = vec![create_module("a", vec![])];
        let graph = build_dependency_graph(&modules);
        assert_eq!(graph.node_count(), 1);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_build_linear_dependency_chain() {
        // a -> b -> c
        let modules = vec![
            create_module("a", vec![("b", "1.0.0")]),
            create_module("b", vec![("c", "1.0.0")]),
            create_module("c", vec![]),
        ];
        let graph = build_dependency_graph(&modules);
        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 2);
    }

    #[test]
    fn test_build_multiple_dependencies() {
        // a depends on both b and c
        let modules = vec![
            create_module("a", vec![("b", "1.0.0"), ("c", "1.0.0")]),
            create_module("b", vec![]),
            create_module("c", vec![]),
        ];
        let graph = build_dependency_graph(&modules);
        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 2);
    }

    #[test]
    fn test_detect_no_cycles() {
        let modules = vec![
            create_module("a", vec![("b", "1.0.0")]),
            create_module("b", vec![("c", "1.0.0")]),
            create_module("c", vec![]),
        ];
        let graph = build_dependency_graph(&modules);
        assert!(!detect_cycles(&graph));
    }

    #[test]
    fn test_detect_simple_cycle() {
        // a -> b -> a (cycle)
        let modules = vec![
            create_module("a", vec![("b", "1.0.0")]),
            create_module("b", vec![("a", "1.0.0")]),
        ];
        let graph = build_dependency_graph(&modules);
        assert!(detect_cycles(&graph));
    }

    #[test]
    fn test_detect_three_node_cycle() {
        // a -> b -> c -> a (cycle)
        let modules = vec![
            create_module("a", vec![("b", "1.0.0")]),
            create_module("b", vec![("c", "1.0.0")]),
            create_module("c", vec![("a", "1.0.0")]),
        ];
        let graph = build_dependency_graph(&modules);
        assert!(detect_cycles(&graph));
    }

    #[test]
    fn test_self_dependency() {
        // a depends on itself
        let modules = vec![create_module("a", vec![("a", "1.0.0")])];
        let graph = build_dependency_graph(&modules);
        assert!(detect_cycles(&graph));
    }

    #[test]
    fn test_diamond_dependency_no_cycle() {
        // a -> b, a -> c, b -> d, c -> d (no cycle)
        let modules = vec![
            create_module("a", vec![("b", "1.0.0"), ("c", "1.0.0")]),
            create_module("b", vec![("d", "1.0.0")]),
            create_module("c", vec![("d", "1.0.0")]),
            create_module("d", vec![]),
        ];
        let graph = build_dependency_graph(&modules);
        assert!(!detect_cycles(&graph));
        assert_eq!(graph.node_count(), 4);
        assert_eq!(graph.edge_count(), 4);
    }

    #[test]
    fn test_missing_dependency_not_added() {
        // a depends on b, but b is not in the module list
        let modules = vec![create_module("a", vec![("b", "1.0.0")])];
        let graph = build_dependency_graph(&modules);
        assert_eq!(graph.node_count(), 1);
        assert_eq!(graph.edge_count(), 0); // Edge not added because 'b' doesn't exist
    }

    #[test]
    fn test_complex_graph_no_cycles() {
        // Complex dependency graph without cycles
        let modules = vec![
            create_module("api", vec![("service", "1.0.0"), ("types", "1.0.0")]),
            create_module("service", vec![("repository", "1.0.0"), ("types", "1.0.0")]),
            create_module(
                "repository",
                vec![("database", "1.0.0"), ("types", "1.0.0")],
            ),
            create_module("database", vec![("types", "1.0.0")]),
            create_module("types", vec![]),
        ];
        let graph = build_dependency_graph(&modules);
        assert!(!detect_cycles(&graph));
        assert_eq!(graph.node_count(), 5);
    }
}
