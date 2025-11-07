use bitvec::prelude::*;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use fastrand::usize;
use std::io::BufRead;
use std::{
    collections::VecDeque,
    fs::File,
    io::{self, BufReader},
};

pub type NodeId = usize;

pub struct Graph {
    pub size: usize,
    pub adj: Vec<Vec<NodeId>>,
}

impl Graph {
    pub fn new(size: usize) -> Self {
        let adj: Vec<Vec<NodeId>> = vec![vec![]; size];

        Graph { size, adj }
    }

    fn add_edge(&mut self, to: NodeId, from: NodeId) {
        self.adj[from].push(to);
        self.adj[to].push(from);
    }

    /// Creates a random prufer sequence to create a random spanning tree to ensure baseline full
    /// connectivity for the graph.
    #[inline(always)]
    fn min_tree(&self) -> Vec<(NodeId, NodeId)> {
        // now hopefully this is going to contain tevery single node
        let mut prufer: Vec<NodeId> = (0..self.size - 2).map(|_| usize(0..self.size)).collect();
        // println!("prufer len: {}", prufer.len());
        fastrand::shuffle(&mut prufer);
        let mut degree: Vec<usize> = vec![1; self.size];
        // println!("len degree: {}", degree.len());

        // check how many times the node appears and then increase the corersponding degree
        // prufer is randomly scattered, degree is not
        for &id in &prufer {
            degree[id] += 1
        }

        let mut edges: Vec<(NodeId, NodeId)> = Vec::with_capacity(10000);

        let mut leaf = 0;
        for _ in 0..self.size - 2 {
            while degree[leaf] != 1 {
                leaf += 1
            }
            let node: NodeId = prufer.remove(0);
            edges.push((leaf, node));
            degree[leaf] -= 1;
            degree[node] -= 1;
            leaf = 0;
        }

        // vector of remianing two nodes after prufer sequence processing
        // we need their indices in the degree vec
        let remaining: Vec<NodeId> = degree
            .iter()
            .enumerate()
            .filter(|(_, d)| **d == 1) // filter the two
            // remaining degree 1
            .map(|(i, _)| i) // take their indices
            .collect(); // collect

        edges.push((remaining[0], remaining[1]));
        edges
    }

    #[inline(always)]
    fn node_id_to_indx(&self, edge: (NodeId, NodeId)) -> usize {
        assert_ne!(edge.0, edge.1, "SELF LOOP!");

        let (a, b) = if edge.0 < edge.1 {
            (edge.0, edge.1)
        } else {
            (edge.1, edge.0)
        };

        ((a * (2 * self.size - a - 1)) / 2) + (b - a - 1)
    }

    /// Calculates the remaining edges to be added to reach certain density
    /// and subsequently generates random unique edges
    #[inline(always)]
    fn connect_to_density_(&mut self, density: f32) -> Vec<(NodeId, NodeId)> {
        let mut edges_ = self.min_tree();
        edges_.sort_by(|a, b| a.cmp(b));

        // let current_density: f32 = (2 * edges_.len()) as f32 / (self.size * (self.size - 1)) as f32;
        let edges_to_make =
            (density * 0.5 * (self.size * (self.size - 1)) as f32) as usize - edges_.len();

        // println!("Edges remaining to make to reach density: {edges_to_make}");
        // println!("Current density: {current_density}");

        let mut adjacency: BitVec = bitvec!(0; (self.size * (self.size - 1)) / 2);

        // just set the really created nodes into the adjacency list
        for &edge in &edges_ {
            adjacency.set(self.node_id_to_indx(edge), true);
        }

        // now I can just take a simplified Erdös-Rényi
        let mut extra: usize = 0;
        // randomly take new edges, refer to the adjacency and set accordingly
        while extra < edges_to_make {
            let test_edge = (fastrand::usize(0..self.size), fastrand::usize(0..self.size));

            if test_edge.0 == test_edge.1 {
                continue;
            }

            let idx = self.node_id_to_indx(test_edge);
            if adjacency[idx] {
                continue;
            } else {
                edges_.push(test_edge);
                adjacency.set(idx, true);
                extra += 1;
            }
        }

        edges_
    }

    pub fn complete_graph(&mut self, density: f32) {
        let edges = self.connect_to_density_(density);
        for edge in edges {
            self.add_edge(edge.0, edge.1);
        }
    }

    pub fn read_test_graph(ident: usize) -> io::Result<Graph> {
        let path = format!("/home/aperiax/School/SVK/graph_{}", ident);
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let mut g = Graph::new(ident);
        for line in reader.lines() {
            let line = line?;
            let mut parts = line.split_whitespace();

            let a: usize = parts.next().unwrap().parse().unwrap();
            let b: usize = parts.next().unwrap().parse().unwrap();
            g.add_edge(a, b);
        }
        Ok(g)
    }

    /// Searches for a path to certain node from a specified start node.
    pub fn bfs(&self, start: NodeId, dest: NodeId) -> Option<Vec<NodeId>> {
        let mut explored: BitVec = bitvec!(0; self.size);
        let mut queue: VecDeque<NodeId> = VecDeque::new();
        let mut parent: Vec<Option<usize>> = vec![None; self.size];

        queue.push_back(start);
        explored.set(start, true);

        while let Some(current_) = queue.pop_front() {
            //TODO: make this not retarded later

            if current_ == dest {
                let mut path: Vec<NodeId> = Vec::new();
                let mut curr = Some(current_);
                while let Some(v) = curr {
                    path.push(v);
                    curr = parent[v]
                }

                path.reverse();
                return Some(path);
            };

            for &dest_ in &self.adj[current_] {
                if !explored[dest_] {
                    explored.set(dest_, true);
                    parent[dest_] = Some(current_);
                    queue.push_back(dest_);
                }
            }
        }
        None
    }
}

fn criterion_config() -> Criterion {
    Criterion::default().sample_size(10) // <--- reduce to 10 samples
}

pub fn bfs_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("BFS-bench-var-size");

    for &size in &[100, 1000, 10000, 100000] {
        let g = Graph::read_test_graph(size).expect("Read error");
        group.bench_with_input(BenchmarkId::from_parameter(&size), &g, |b, g| {
            b.iter(|| g.bfs(0, usize(0..size)))
        });
    }
}

criterion_group! {name=benches; config=criterion_config();targets= bfs_bench}
criterion_main!(benches);
