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

fn find_max_n(lines: &str, n: usize) -> u64 {

    let v  = lines.trim().as_bytes();

    let mut result:u64 = 0;

    // let mut head = v; 
    let mut shift = n-1;
    let mut start = 0;

    for _ in 0..n {
        let head = &v[start .. (v.len() - shift)];

        let &max = head.iter().max().expect("Empty iterator");
        let index = head.iter().position(|&x| x == max).expect("Max not found");

        result = result*10 + u64::from(max - b'0');

        if shift != 0 {
            shift = shift-1;
        }
        start = start+index+1;
    }
    result
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

