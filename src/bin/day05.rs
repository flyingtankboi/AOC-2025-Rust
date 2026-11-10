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

        // set.insert((low, high)); part 1
        // Smart insert by combining
        let touching : Vec<(u64, u64)> =  set.iter().filter(|&&(lo,hi)| {
            (lo ..= hi).contains(&low) || (lo ..= hi).contains(&high) || (low < lo && high > hi)
        }).cloned().collect();

        let mut max_high = high;
        let mut min_low = low;
        // dbg!(&set,(low,high), &touching);
        for (parent_low, parent_high) in touching {
            set.remove(&(parent_low, parent_high));
            max_high = max(max_high, parent_high);
            min_low = min(min_low, parent_low);
        }

        set.insert((min_low, max_high));
        // dbg!(&set);

        // let _ = match ((parent_low ..= parent_high).contains(&low), (parent_low ..= parent_high).contains(&high)){
        //     (true, false) => set.insert((parent_low,high)),
        //     (false, true) => set.insert((low,parent_high)),
        //     (false, false) => set.insert((low,high)),
        //     (true, true) => set.insert((parent_low, parent_high)),
        // };



    }

    // Here I thought i would need an input_iter.next() to throw away the "" but id didnt work and apparently I didnt need it

    // Parse ingredients
    for line in input_iter {
        let x : u64 = line.trim().parse().expect("Not an int");
        result1 += set.iter().any(|&(lo,hi)| (lo ..= hi).contains(&x)) as u64;
    }
    // dbg!(&set);
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

