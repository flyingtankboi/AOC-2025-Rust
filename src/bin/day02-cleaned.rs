use std::fs::read_to_string;

fn day2(input : &str)-> (i64,i64){

    let mut count1 = 0;
    let mut count2 = 0;

    // Split comma
    for id_range in input.trim().split(','){
        // Split -
        let (range_low_str, range_high_str) = id_range.split_once('-').expect("No - delimiter found");
        let range_low : i64 = range_low_str.parse().expect("ID not an int");
        let range_high : i64 = range_high_str.parse().expect("ID not an int");

        // Sum id
        count1 += (range_low ..= range_high).filter( |&id| is_invalid1(id)).sum::<i64>();
        count2 += (range_low ..= range_high).filter( |&id| is_invalid2(id)).sum::<i64>();
    }

    (count1,count2)
}

fn is_invalid1(id : i64) -> bool {
    let id_str = id.to_string();
    if id_str.len()%2 != 0 {return false}
    let (left, right) = id_str.split_at(id_str.len()/2);
    left == right
}

fn is_invalid2(id : i64) -> bool {

    let id_str = id.to_string();
    let id_bytes = id_str.as_bytes();
    // For diffrent pattern lengths
    (1 ..= id_bytes.len()/2).any( |pattern_len| {
        // prune
        if id_bytes.len()%pattern_len != 0 {return false}
        // check pattern with chunks
        let pattern = &id_bytes[0..pattern_len];
        id_bytes.chunks(pattern_len).all(|b| b == pattern)
    })
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

