use std::{collections::HashSet, fs::read_to_string, cmp::max, cmp::min};

fn day5(input : &str)-> (u64, u64){

    let mut set = HashSet::new();
    let mut input_iter = input.trim().lines();
    let mut result1 = 0;

    // Parse ranges
    for line in input_iter.by_ref().take_while(|&x| x != "" ){

        let (low, high):(u64,u64) =  match line.trim().split_once("-") {
            Some((l_s,h_s)) => (l_s.parse().expect("Not an int"), h_s.parse().expect("Not an int")),
            None => panic!("No delimiter -"),
        };

        // Smart insert by combining
        let touching : Vec<(u64, u64)> =  set.iter().filter(|&&(lo,hi)| {
            (lo ..= hi).contains(&low) || (lo ..= hi).contains(&high) || (low < lo && high > hi)
        }).cloned().collect();

        let mut max_high = high;
        let mut min_low = low;
        for (parent_low, parent_high) in touching {
            set.remove(&(parent_low, parent_high));
            max_high = max(max_high, parent_high);
            min_low = min(min_low, parent_low);
        }

        set.insert((min_low, max_high));
    }

    // Parse ingredients
    for line in input_iter {
        let x : u64 = line.trim().parse().expect("Not an int");
        result1 += set.iter().any(|&(lo,hi)| (lo ..= hi).contains(&x)) as u64;
    }
    let result2 = set.iter().map(|&(lo, hi)| hi-lo+1).sum();

    (result1, result2)
}


// For tests
fn main(){
    let path = "inputs/day5/final.txt";
    let input = read_to_string(path).expect("Couldn't read file");

    println!("{:?}", day5(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input1(){
        let path = "inputs/day5/1.txt";
        let input = read_to_string(path).expect("Couldn't read file");

        let result = day5(&input); 
        assert_eq!(result, (3,14),
            "Result input 1 : {:?}", result
        );
    }
}

