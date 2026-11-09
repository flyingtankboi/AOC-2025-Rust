
use std::fs::read_to_string;

fn day2(input : &str)-> (i64,i64){

    let mut count1 = 0;
    let mut count2 = 0;

    // Split comma
    for id_range in input.trim().split(','){
        // Split -
        let range_vec : Vec<i64> = id_range.split('-').map(|id| id.parse::<i64>().expect("id not an int")).collect();
        if range_vec.len() > 2 {panic!("More than two id in id range")}
        // Check id
        (range_vec[0] ..= range_vec[1]).for_each( |id| {
            if is_valid1(id) {
                count1 += id;
            }
            if is_valid2(id){
                count2 += id;
            }
        });
    }

    (count1,count2)
}

fn is_valid1(id : i64) -> bool {
    let id_str = id.to_string();
    let (left, right) = id_str.split_at(id_str.len()/2);
    left == right
}

fn is_valid2(id : i64) -> bool {
    let id_str = id.to_string();

    for str_length in 1 ..= id_str.len()/2 {
        if id_str.len()%str_length != 0 {continue;}

        let pattern_repeat = id_str.len()/str_length;
        let pattern = &id_str[0..str_length];
        
        if id_str == pattern.repeat(pattern_repeat) {
            return true;
        }
    }
    false
}

// For tests
fn main(){
    let path = "inputs/day2/final.txt";
    let mut input = read_to_string(path).expect("Couldn't read file");

    println!("{:?}", day2(&mut input));
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

