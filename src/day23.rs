use std::collections::{BTreeSet, HashMap, HashSet};

use crate::solution::Solution;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Graph<'a> {
    neighbors: HashMap<&'a str, HashSet<&'a str>>,
}

impl<'a> Graph<'a> {
    fn from_edges(edges: &[(&'a str, &'a str)]) -> Self {
        let mut neighbors = HashMap::new();
        edges.iter().for_each(|&(a, b)| {
            neighbors.entry(a).or_insert_with(HashSet::new).insert(b);
            neighbors.entry(b).or_insert_with(HashSet::new).insert(a);
        });
        Self { neighbors }
    }

    fn nodes(&'a self) -> impl Iterator<Item = &'a str> {
        self.neighbors.keys().copied()
    }
}

pub struct Puzzle;

impl Puzzle {
    fn parse_input(input: &str) -> Vec<(&str, &str)> {
        input
            .lines()
            .map(|l| {
                let (l, r) = l.split_once('-').unwrap();
                (l, r)
            })
            .collect::<Vec<_>>()
    }
}

impl Solution for Puzzle {
    fn part1(&self, input: &str) -> String {
        let graph = Graph::from_edges(&Self::parse_input(input));
        let mut triplets = HashSet::new();
        let mut visited = HashSet::new();
        graph.nodes().for_each(|node| {
            // Skip if visited or not starting with 't'
            if visited.contains(node) || !node.starts_with('t') {
                return;
            }
            graph.neighbors[node]
                .iter()
                // Skip if already visited
                .filter(|&neighbor| !visited.contains(neighbor))
                .for_each(|&neighbor| {
                    graph.neighbors[neighbor]
                        .iter()
                        // Skip if already visited
                        .filter(|&nn| {
                            !visited.contains(nn)
                            // Skip if none starting with 't'
                                && (node.starts_with('t')
                                    || neighbor.starts_with('t')
                                    || nn.starts_with('t'))
                        })
                        .for_each(|&nn| {
                            if graph.neighbors[nn].contains(node) {
                                let mut triplet = [node, neighbor, nn];
                                triplet.sort_unstable();
                                triplets.insert(triplet);
                            }
                        });
                });
            visited.insert(node);
        });
        triplets.len().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let graph = Graph::from_edges(&Self::parse_input(input));
        // Every node is in a clique, and every clique is always contained in a maximal clique
        // So we don't need to check visited nodes again
        let mut clique = BTreeSet::new();
        let mut maximum_clique = BTreeSet::new();
        let mut visited = HashSet::new();
        graph.nodes().for_each(|node| {
            // Skip if already visited
            if !visited.contains(node) {
                // Create a new clique
                clique.clear();
                clique.insert(node);
                // Find the clique this node belongs to
                graph.nodes().for_each(|n| {
                    if !visited.contains(node)
                        && clique.iter().all(|&c| graph.neighbors[c].contains(n))
                    {
                        clique.insert(n);
                        visited.insert(n);
                        if clique.len() > maximum_clique.len() {
                            maximum_clique = clique.clone();
                        }
                    }
                });
            }
        });
        maximum_clique.into_iter().collect::<Vec<_>>().join(",")
    }
}
