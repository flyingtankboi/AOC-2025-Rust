use std::fs::read_to_string;

fn day_(input : &str)-> (){

}


// For tests
fn main(){
    let path = "inputs/day_/final.txt";
    let input = read_to_string(path).expect("Couldn't read file");

    println!("{:?}", day_(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input1(){
        let path = "inputs/day_/1.txt";
        let input = read_to_string(path).expect("Couldn't read file");

        let result = day_(&input); 
        assert_eq!(result, (),
            "Result input 1 : {:?}", result
        );
    }
}

