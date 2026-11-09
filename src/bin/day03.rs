use std::{fs::read_to_string};

fn day3(input : &str)-> (u64,u64){
    let mut sum1 = 0;
    let mut sum2 = 0;
    for lines in input.trim().lines() {
        sum1 += find_max_n(lines, 2);
        sum2 += find_max_n(lines, 12);
    }
    (sum1,sum2)
}

fn find_max_n(lines: &str, n: u64) -> u64 {

    let v: Vec<(usize, u64)> = lines.trim().chars().map(|c| Into::<u64>::into(c.to_digit(10).expect("NaN"))).rev().enumerate().collect();

    let mut maxes = Vec::new();
    let n_usize: usize = n.try_into().expect("Can't convert to usize");


    let mut head = v; 
    let mut tail: Vec<(usize, u64)> = head.split_off(n_usize-1);

    for _ in 0..n {

        let &(index, max) = tail.iter().max_by_key(|(_, val)| *val).expect("Empty iterator");
        maxes.push(max);

        if let Some(next_head) = head.pop() {
            tail.insert(0, next_head);
        }
        
        tail.retain(|&(i, _)| i < index);
    }

    let r = maxes.iter().rev().enumerate().map(|(i, val)| val * 10_u64.pow(i.try_into().expect("Can't convert to u64"))).sum();
    r
}

// For tests
fn main(){
    let path = "inputs/day3/final.txt";
    let input = read_to_string(path).expect("Couldn't read file");

    println!("{:?}", day3(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input1(){
        let path = "inputs/day3/1.txt";
        let input = read_to_string(path).expect("Couldn't read file");

        let result = day3(&input); 
        assert_eq!(result, (357,3121910778619),
            "Result input 1 : {:?}", result
        );
    }
}

