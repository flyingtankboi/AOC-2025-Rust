use std::{fs::read_to_string};

fn day2(input : &str)-> (i64,i64){

    let mut count1 = 0;
    let mut count2 = 0;

    // Split comma
    for id_range in input.trim().split(','){
        // Split -
        let (range_low_str, range_high_str) = id_range.split_once('-').expect("No - delimiter found");
        let range_low : i64 = range_low_str.parse().expect("ID not an int");
        let range_high : i64 = range_high_str.parse().expect("ID not an int");

        // count1 += sum1(range_low, range_high);
        // count2 += sum2(range_low, range_high);
    }

    (count1,count2)
}

// TODO 
fn sum1(low : i64, high : i64) -> () {
    // Can be done with an exact formula
}

// TODO 
fn sum2(low : i64, high : i64) -> () {
    // Can be done with an exact formula
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
        assert_eq!(result, (1227775554,4174379265),
            "Result input 1 : {:?}", result
        );
    }
}

