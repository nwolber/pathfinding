//! Algorithms for directed graphs.

pub mod astar;
pub mod bfs;
pub mod dfs;
pub mod dijkstra;
pub mod edmonds_karp;
pub mod fringe;
pub mod idastar;
pub mod iddfs;
pub mod strongly_connected_components;
pub mod topological_sort;

use indexmap::IndexMap;
use itertools;
use std::hash::Hash;

fn reverse_path<N, V>(
    parents: &IndexMap<N, V>,
    mut parent: impl FnMut(&V) -> usize,
    start: usize,
) -> Vec<N>
where
    N: Eq + Hash + Clone,
{
    let path = itertools::unfold(start, |i| {
        parents.get_index(*i).map(|(node, value)| {
            *i = parent(value);
            node
        })
    })
    .collect::<Vec<&N>>();

    path.into_iter().rev().cloned().collect()
}
