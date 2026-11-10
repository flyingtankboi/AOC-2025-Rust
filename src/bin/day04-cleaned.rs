use std::{collections::HashSet, fs::read_to_string};
use itertools::iproduct;

fn day4(input : &str)-> (u64, u64){
    let mut papers = HashSet::new();
    

    for (i ,line) in input.trim().lines().enumerate() {
        line.trim().chars().enumerate().for_each(|(j,c)|{
            if c == '@' {
                papers.insert((i as isize,j as isize));
            }
        });
    }

    let mut result1 = None;
    let mut result2 = 0;
    let directions : Vec<(isize, isize)> = iproduct!(-1..=1, -1..=1).filter(|&x|x!=(0,0)).collect();
    let mut removeable = Vec::new();
    
    loop{
        
        for &paper in &papers {

            let count = directions.iter().filter(|&&direction|{
                papers.contains(&(paper.0 + direction.0, paper.1 + direction.1))
            }).count(); 
            
            if count < 4 {
                removeable.push(paper);
            }
        }

        // Part 1
        result1.get_or_insert(removeable.len() as u64);

        // Part 2
        if removeable.is_empty() {
            break
        } else {
            for x in removeable.drain(..) {
                result2 += 1;
                papers.remove(&x);
            }
        }
    }

    (result1.unwrap_or(0), result2)

}



// For tests
fn main(){
    let path = "inputs/day4/final.txt";
    let input = read_to_string(path).expect("Couldn't read file");

    println!("{:?}", day4(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input1(){
        let path = "inputs/day4/1.txt";
        let input = read_to_string(path).expect("Couldn't read file");

        let result = day4(&input); 
        assert_eq!(result, (13,43),
            "Result input 1 : {:?}", result
        );
    }
}

