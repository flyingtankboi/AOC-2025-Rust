
use std::{fs::{File, read, read_to_string}, io::Read};

fn day2(input : &str)-> (i64,i64){
    (0,0)
}

// For tests
fn main(){
    let path = "inputs/day2/final.txt";
    let input = read_to_string(path).expect("Couldn't read file");

    println!("{:?}", day2(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input1(){
        let path = "inputs/day2/1.txt";
        let input = read_to_string(path).expect("Couldn't read file");

        let result = day2(&input); 
        assert_eq!(result, (4,0),
            "Result input 1 : {:?}", result
        );
    }
}

