use std::{collections::HashSet, fs::read_to_string};
use itertools::iproduct;

fn day4(input : &str)-> (u64, u64){
    let mut papers = HashSet::new();
    

    for (i,line) in input.trim().lines().enumerate() {
        line.trim().chars().enumerate().for_each(|(j,c)|{
            if c == '@' {
                papers.insert((i,j));
            }
        });
    }

    let mut result1 = 0;
    let mut result2 = 0;
    let DIRECTIONS : Vec<(isize, isize)> = iproduct!(-1..=1, -1..=1).collect();
    let mut loop_iter = true;
    
    
    loop{
        let mut removeable = Vec::new();
        for &paper in &papers {
            let mut count = 0; 

            for &direction in &DIRECTIONS {
                let neighbour = match (paper.0.checked_add_signed(direction.0), paper.1.checked_add_signed(direction.1)){
                    (Some(ni), Some(nj)) => (ni,nj),
                    _ => continue,
                };
                
                if let Some(_) = papers.get(&neighbour){
                    count += 1;
                }
            }
            
            if count <= 4 {
                result2 += 1;
                if loop_iter {
                    result1 += 1;
                }
                removeable.push(paper);
            }
        }

        loop_iter = false;
        if removeable.len() == 0 {
            break
        } else {
            for x in removeable{papers.remove(&x);}
        }
    }

    (result1, result2)

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

