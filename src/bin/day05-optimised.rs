use std::{fs::read_to_string};


fn day5(input: &str) -> (u64, u64) {
    let mut set = Vec::new();
    let mut input_iter = input.trim().lines();

    // Parse ranges
    for line in input_iter.by_ref().take_while(|&x| !x.trim().is_empty()) {
        let (low, high): (u64, u64) = match line.trim().split_once('-') {
            Some((l_s, h_s)) => (
                l_s.trim().parse().expect("Not an int"),
                h_s.trim().parse().expect("Not an int"),
            ),
            None => panic!("No delimiter -"),
        };

        set.push((low, high));
    }
    set.sort();

    let mut set_merged: Vec<(u64, u64)> = Vec::new();
    let mut curr_range: Option<(u64, u64)> = None;

    for (low, high) in set {
        match curr_range {
            Some((c_low, c_high)) => {
                if low <= c_high.saturating_add(1) {
                    curr_range = Some((c_low, c_high.max(high)));
                } else {
                    set_merged.push((c_low, c_high));
                    curr_range = Some((low, high));
                }
            }
            None => {
                curr_range = Some((low, high));
            }
        }
    }

    // Push the final range remaining in memory
    if let Some(last_range) = curr_range {
        set_merged.push(last_range);
    }

    // Parse ingredients
    let result1 = input_iter
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().parse::<u64>().expect("Not an int"))
        .filter(|&x| {
            let Some(index) = set_merged.partition_point(|&(low, _)| low <= x).checked_sub(1) else {
                return false;
            };
            let (_, hi) = set_merged[index];
            x <= hi
        })
        .count() as u64;

    let result2 = set_merged.iter().map(|&(lo, hi)| hi - lo + 1).sum();
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

