use std::fs::read_to_string;
use std::collections::{HashSet, VecDeque, HashMap};

fn day10(input : &str)-> (i64, i64){

    let mut result1 = 0;
    let mut result2 = 0;

    for line in input.trim().lines() {
        // Parse all information
        let (_,temp_right) = line.split_once("[").expect("Not parsable");
        let (lamps_str, temp_right) = temp_right.split_once("]").expect("Not parsable");

        let lamps : Vec<bool> = lamps_str.bytes()
                                             .map(|b| b == b'#' )
                                             .collect();

        let (temp_middle, temp_right) = temp_right.split_once('{').expect("Not parsable");
        let (voltage_str,_) = temp_right.split_once('}').expect("Not parsable");

        let voltage : Vec<i64> = voltage_str.split(',')
                .map(|to_parse| to_parse.parse::<i64>().expect("Not an int"))
                .collect();

        let buttons : Vec<Vec<usize>> = temp_middle
                   .split_whitespace()
                   .map(|button_str| {
                        let button_str_trimmed = &button_str[1..button_str.len() - 1];
                        button_str_trimmed.split(',')
                                          .map(|button_light| button_light.parse::<usize>().expect("Not an int"))
                                          .collect()
                   })
                   .collect();
        
            
        let by_pattern = subsets_by_pattern(&buttons);
        result1 += min_lights(&lamps, &by_pattern);

        let mut memo = HashMap::new();
        result2 += min_presses(&voltage, &buttons, &by_pattern, &mut memo)
            .expect("No solution");

    }
    (result1, result2)
}


fn subsets_by_pattern(buttons: &[Vec<usize>]) -> HashMap<u32, Vec<u32>> {
    let button_masks: Vec<u32> = buttons
        .iter()
        .map(|button| button.iter().fold(0, |acc, &light| acc | (1 << light)))
        .collect();

    let mut by_pattern = HashMap::new();
    for subset in 0..(1u32 << buttons.len()) {
        let mut pattern = 0;
        for (i, &mask) in button_masks.iter().enumerate() {
            if subset & (1 << i) != 0 {
                pattern ^= mask;
            }
        }
        by_pattern.entry(pattern).or_insert_with(Vec::new).push(subset);
    }
    by_pattern
}

fn min_lights(lights: &[bool], by_pattern: &HashMap<u32, Vec<u32>>) -> i64 {
    let target = lights
        .iter()
        .enumerate()
        .filter(|&(_, &on)| on)
        .fold(0, |acc, (i, _)| acc | (1 << i));

    by_pattern[&target]
        .iter()
        .map(|subset| subset.count_ones() as i64)
        .min()
        .expect("no subset makes these lights")
}

fn press_subset(target: &[i64], buttons: &[Vec<usize>], subset: u32) -> Option<Vec<i64>> {
    let mut remaining = target.to_vec();
    for (i, button) in buttons.iter().enumerate() {
        if subset & (1 << i) != 0 {
            for &counter in button {
                remaining[counter] -= 1;
            }
        }
    }

    if remaining.iter().any(|&v| v < 0) {
        None
    } else {
        Some(remaining)
    }
}

fn min_presses(
    target: &[i64],
    buttons: &[Vec<usize>],
    by_pattern: &HashMap<u32, Vec<u32>>,
    memo: &mut HashMap<Vec<i64>, Option<i64>>,
) -> Option<i64> {
    if target.iter().all(|&v| v == 0) {
        return Some(0);
    }
    if let Some(&known) = memo.get(target) {
        return known;
    }

    let odd_pattern: u32 = target
        .iter()
        .enumerate()
        .filter(|&(_, &v)| v % 2 != 0)
        .fold(0, |acc, (i, _)| acc | (1 << i));

    let Some(subsets) = by_pattern.get(&odd_pattern) else {
        return None;
    };

    let best = subsets
        .iter()
        .filter_map(|&subset| {
            let remaining = press_subset(target, buttons, subset)?;
            let halved: Vec<i64> = remaining.iter().map(|v| v / 2).collect();
            let rest = min_presses(&halved, buttons, by_pattern, memo)?;
            Some(subset.count_ones() as i64 + 2 * rest)
        })
        .min();

    memo.insert(target.to_vec(), best);
    best
}



// For tests
fn main(){
    let path = "inputs/day10/final.txt";
    let input = read_to_string(path).expect("Couldn't read file");

    println!("{:?}", day10(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input1(){
        let path = "inputs/day10/1.txt";
        let input = read_to_string(path).expect("Couldn't read file");

        let result = day10(&input); 
        assert_eq!(result, (7,33),
            "Result input 1 : {:?}", result
        );
    }
}

