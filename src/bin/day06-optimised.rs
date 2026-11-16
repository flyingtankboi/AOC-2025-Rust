use std::{fs::read_to_string};


enum Op {Sum, Prod}

impl Op {
    fn parse(c : char) -> Op {
        match c {
            '+' => Op::Sum,
            '*' => Op::Prod,
            _ => panic!("Not an operator")
        }
    }

    fn init_integer(&self) -> i64 {
        match self {
            Op::Sum => 0,
            Op::Prod => 1,
        }
    } 

    fn apply(&self, acc : i64, n : i64) -> i64 {
        match self {
            Op::Sum  =>  acc + n,
            Op::Prod =>  acc * n,
        }
    }
}


fn day6(input : &str)-> (i64, i64){

    let lines : Vec<&str> = input.lines().collect();
    let (&operator_line, input_lines) = lines.split_last().expect("empty");

    let operators : Vec<(usize,Op)> = operator_line.chars()
                                                   .enumerate()
                                                   .filter(|&(_,c)| !c.is_whitespace())
                                                   .map(|(i,c)| (i,Op::parse(c)))
                                                   .collect();

    let mut running_results1 : Vec<i64> = operators.iter().map(|(_, op)| op.init_integer()).collect();
    let mut running_results2 = running_results1.clone();

    // Line by line for result 1
    for &line in input_lines {
        // Results 1 
        for (i,number_str) in line.split_whitespace().enumerate() {
            let number : i64 = number_str.parse().expect("Not an int");
            running_results1[i] = operators[i].1.apply(running_results1[i], number);
        }
    }

    // Column by column for result 2
    let mut operation_index = 0;
    for i in 0 .. input_lines[0].len() {
        let column_temp : Vec<u8> = input_lines.iter().filter_map(|&line| line.as_bytes().get(i)).copied().collect();
        let column = String::from_utf8(column_temp).expect("Not UTF8");
        if let Ok(number) = column.trim().parse::<i64>() {
            running_results2[operation_index] = operators[operation_index].1.apply(running_results2[operation_index], number);
        } else {
            operation_index += 1;
        }
    }

    let result1 = running_results1.iter().sum();
    let result2  = running_results2.iter().sum();
    
    (result1, result2)
}

// For tests
fn main(){
    let path = "inputs/day6/final.txt";
    let input = read_to_string(path).expect("Couldn't read file");

    println!("{:?}", day6(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input1(){
        let path = "inputs/day6/1.txt";
        let input = read_to_string(path).expect("Couldn't read file");

        let result = day6(&input); 
        assert_eq!(result, (4277556,3263827),
            "Result input 1 : {:?}", result
        );
    }
}

