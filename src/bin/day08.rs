use std::fs::read_to_string;
// use kdtree::KdTree;
// use kdtree::distance::squared_euclidean;
use std::cmp::{Ordering,min};
use std::collections::BinaryHeap;

pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

impl UnionFind {
    /// Creates a new Union-Find with `n` isolated elements (`0..n`).
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            count: n,
        }
    }

    /// Finds the root representative of `x` with path compression.
    pub fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]]; // Path halving
            x = self.parent[x];
        }
        x
    }

    /// Merges the sets containing `x` and `y`. Returns `true` if they were separate.
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let mut root_x = self.find(x);
        let mut root_y = self.find(y);
        if root_x == root_y {
            return false;
        }
        // Attach the smaller tree under the larger tree
        if self.size[root_x] < self.size[root_y] {
            std::mem::swap(&mut root_x, &mut root_y);
        }
        self.parent[root_y] = root_x;
        self.size[root_x] += self.size[root_y];
        self.count -= 1;
        true
    }

    /// Checks if `x` and `y` belong to the same set.
    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    /// Returns the number of disjoint sets.
    pub fn count(&self) -> usize {
        self.count
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Edge {
    point1_class : usize,
    point2_class : usize,
    distance : i64,
}

impl Ord for Edge {
    fn cmp(&self, other : &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Edge {}

fn day8(input : &str, to_connect : usize)-> (i64,i64){
    // let mut ktree = KdTree::new(3);
    

    // Get coords and put them in a ktree
    let coords : Vec<[i64; 3]> = input.trim().lines().map(|line| {
        let [x_str, y_str, z_str] : [&str;3] = line.trim().splitn(3, ',').collect::<Vec<&str>>().try_into().expect("Expected 3 coordinates");
        let coord : [i64;3] = [x_str.parse().expect("Not an int"), y_str.parse().expect("Not an int"), z_str.parse().expect("Not an int")];
        coord
    }).collect();

    let mut uf = UnionFind::new(coords.len());

    // let mut unique_circuit_id : usize = 0;
    // for coord in &coords {
    //     ktree.add(coord, unique_circuit_id).expect("Unable to add to tree");
    //     unique_circuit_id += 1;
    // }

    // Sort all edges
    let mut heap = BinaryHeap::new();
    for i in 0..coords.len() {
        for j in (i+1).. coords.len(){
            let dx = coords[i][0] - coords[j][0];
            let dy = coords[i][1] - coords[j][1];
            let dz = coords[i][2] - coords[j][2];
            let distance = dx * dx + dy * dy + dz * dz;

            heap.push(Edge { point1_class: i, point2_class : j, distance });
        }
    }
    let nearest_neighbor_edges = heap.into_sorted_vec();

    // Union
    let mut result1 = 0;
    let mut result2 = 0;
    let mut last_edge = Edge{point1_class:0, point2_class:0, distance: 0};

    for i in 0..nearest_neighbor_edges.len() {
        // dbg!(&i, nearest_neighbor_edges[i]);
        if i == to_connect{ // end of part 1
            dbg!("Result1");
            let mut sizes_1: Vec<usize> = (0..coords.len())
                                            .filter(|&i| uf.parent[i] == i)
                                            .map(|i| uf.size[i])
                                            .collect();
            sizes_1.sort();
            result1 = sizes_1.iter().rev().take(3).product::<usize>() as i64;
        }

        let Edge{point1_class, point2_class, distance} = nearest_neighbor_edges[i];
        if uf.union(point1_class,point2_class){
            last_edge = Edge{point1_class, point2_class, distance};
        };
        if uf.count() == 1 {
            break
        }
    }

    result2 = coords[last_edge.point1_class][0] * coords[last_edge.point2_class][0];


    (result1,result2)
}


// For tests
fn main(){
    let path = "inputs/day8/final.txt";
    let input = read_to_string(path).expect("Couldn't read file");

    println!("{:?}", day8(&input,1000));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input1(){
        let path = "inputs/day8/1.txt";
        let input = read_to_string(path).expect("Couldn't read file");

        let result = day8(&input,10); 
        assert_eq!(result, (40,25272),
            "Result input 1 : {:?}", result
        );
    }
}

