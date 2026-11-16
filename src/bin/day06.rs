use std::{fs::read_to_string, iter};

fn day6(input : &str)-> (i64, i64){

    let mut input_it = input.lines().rev();
    let operators : Vec<(usize,char)> = input_it.next()
                                                .expect("Empty input")
                                                .chars()
                                                .enumerate()
                                                .filter(|&(_,c)| !c.is_whitespace())
                                                .collect();
    let mut running_results1 = init_results(&operators);

    let mut running_numbers : Vec<Vec<i64>> = vec![Vec::new();operators.len()];

    for line in input_it.rev() {
        // Results 2
        let mut current_batch_index = 0;
        let mut current_op_index = operators[current_batch_index].0;
        
        for (index, c) in line.chars().enumerate() {
            if current_batch_index + 1 < operators.len() && index == operators[current_batch_index+1].0 { // next batch
                current_batch_index += 1;
                current_op_index = operators[current_batch_index].0;
            }
            if c.is_whitespace() {continue}


            // Use relative index to create the number
            let relative_index = index - current_op_index;
            // dbg!("----");
            // dbg!(&relative_index, &current_op_index, &current_batch_index, &c);
            if relative_index >= running_numbers[current_batch_index].len() {
                let diff = relative_index - running_numbers[current_batch_index].len() + 1;
                running_numbers[current_batch_index].extend(iter::repeat_n(0, diff ));
            }

            running_numbers[current_batch_index][relative_index] *= 10;
            running_numbers[current_batch_index][relative_index] += c.to_digit(10).expect("Not a digit") as i64;
        }


        // Results 1 
        for (i,number_str) in line.split_whitespace().enumerate() {
            let number : i64 = number_str.parse().expect("Not an int");
            match operators[i].1 {
                '+' =>  running_results1[i] += number,
                '*' =>  running_results1[i] *= number,
                _ => panic!("Not an operator"),
            }
        }
    }

    let result1 = running_results1.iter().sum();
    let result2: i64 = running_numbers
                        .iter()
                        .enumerate()
                        .map(|(i,v)| -> i64{
                            match operators[i].1 {
                                '+' =>  v.iter().sum(),
                                '*' =>  v.iter().product(),
                                _ => panic!("Not an operator"),
                            }
                        })
                        .sum();
    
    (result1, result2)
}

fn init_results(operators : &Vec<(usize,char)>) -> Vec<i64> {
    let mut running_results = Vec::new();
    for (_,ops) in operators {
        match ops {
            '+' =>  running_results.push(0),
            '*' =>  running_results.push(1),
            _ => panic!("Not an operator"),
        };
    }
    running_results
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

