use std::{fs::File, io::Read};

fn day1(path : &str)-> (i64,i64){
    let mut f = File::open(path).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    let mut count1 = 0;
    let mut count2: i64 = 0;
    let mut clock: i64 = 50;

    contents.trim().lines().for_each( |line|{
        let (direction, number_str) = line.trim().split_at(1);
        let mut number : i64 = number_str.parse().unwrap();

        if direction == "L" { number = -number }
        let new_clock = clock + number;
       
        // this CANNOT be the best way to do this :/
        count2 += (new_clock/100).abs() + i64::from(new_clock <= 0 && clock != 0);
        clock = new_clock.rem_euclid(100);

        if clock == 0 {
            count1 += 1;
        }
    });
    (count1, count2)
}

// For tests
fn main(){
    println!("{:?}", day1("././inputs/day1/final.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input1(){
        let result = day1("././inputs/day1/1.txt"); 
        assert_eq!(result, (4,7),
            "Result input 1 : {:?}", result
        );
    }
}

