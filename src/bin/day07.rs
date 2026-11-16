use std::fs::read_to_string;
use std::collections::{HashSet, HashMap};

#[derive(Eq, PartialEq, Hash)]
enum Block{
    Spawner(i64,i64),
    Splitter(i64,i64),
}


impl Block {
    fn parse(b : &u8, i:i64, j:i64) -> Option<Block> {
        match b {
            b'S' => Some(Block::Spawner(i,j)),
            b'^' => Some(Block::Splitter(i,j)),
            _    => None,
        }
    }
}

fn day7(input : &str)-> (i64,i64){
    let mut map = HashSet::new();
    let mut rays = HashMap::new();

    let lines = input.lines().enumerate();

    let mut floor_i = 0;

    let mut result1 = 0;
    let mut result2 = 0;
    
    // Create map
    for (i,line) in lines {
        floor_i = i as i64;
        line.trim()
            .as_bytes()
            .iter()
            .enumerate()
            .filter_map(|(j, b)| Block::parse(b, i as i64, j as i64))// need cleaner conversion
            .all(|block| {
                if let Block::Spawner(i,j) = block {
                    rays.insert((i+1, j), 1);
                }
                map.insert(block)
            }); 
    }

    // Run simulation
    loop {
        let mut to_add = Vec::new(); 
        for ((i,j), n) in rays.drain() { // clears the rays so we keep only the current one
            if let Some(_) = map.get(&Block::Splitter(i+1,j)) { // splitter beneeth
                result1 += 1;
                to_add.push(((i+1, j-1),n)); //left
                to_add.push(((i+1, j+1),n)); //right
            } else if i+1 == floor_i  { // floor beneeth
                result2 += n; // Count number of timelines
                continue
            } else { // spread
                to_add.push(((i+1,j),n));
            }
        }

        if to_add.is_empty() {
            break
        } else {
            for (new_ray, new_timeline) in to_add.drain(..) {
                rays.entry(new_ray).and_modify(|n| *n += new_timeline).or_insert(new_timeline);
            }
        }
    }
    (result1, result2)
}


// For tests
fn main(){
    let path = "inputs/day7/final.txt";
    let input = read_to_string(path).expect("Couldn't read file");

    println!("{:?}", day7(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input1(){
        let path = "inputs/day7/1.txt";
        let input = read_to_string(path).expect("Couldn't read file");

        let result = day7(&input); 
        assert_eq!(result, (21,40),
            "Result input 1 : {:?}", result
        );
    }
}

