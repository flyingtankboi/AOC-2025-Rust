use std::{cmp::{max, min}, collections::HashSet, fs::read_to_string, iter};

fn day9(input : &str)-> (i64,i64){

    // get coords
    let mut coords : Vec<(i64, i64)> = input.trim().lines().map(|line| {
        let (x_s,y_s) = line.trim().split_once(',').expect("Unable to parse");
        let (x,y) : (i64,i64) = (x_s.parse().expect("Not an int"), y_s.parse().expect("Not an int"));
        (x,y)
    }).collect();
    let mut convex_hull = coords.clone();
    convex_hull.push(coords[0]);
    coords.sort();

    // Complete convex hull sides
    let mut convex_hull_set = HashSet::new();
    for coord_packed in convex_hull.windows(2) {
        if let &[coord1,coord2] = coord_packed {
            for dx in min(coord1.0,coord2.0)..= max(coord1.0,coord2.0) {
                for dy in min(coord1.1,coord2.1)..=max(coord1.1,coord2.1){
                    convex_hull_set.insert((dx,dy));

                    //print_hull(&convex_hull_set);
                }
            }
        }
    }
    //print_hull(&convex_hull_set);

    // result 1 
    let mut result1 = 0;
    let mut result2 = 0;
    for i in 0..coords.len() {
        for j in (i+1)..coords.len() {
            //dbg!(&coords[i], &coords[j]);
            let dx = (coords[j].0 - coords[i].0 + 1).abs();
            let dy = (coords[j].1 - coords[i].1 + 1).abs();
            //dbg!(dx*dy);
            result1 = result1.max(dx*dy);
            //dbg!(coords[i], coords[j]);
            // print_hull(&convex_hull_set);
            if !area_is_out_of_hull(coords[i], coords[j], &convex_hull_set) {
                result2 = result2.max(dx*dy);
            }
        }
    }

    (result1,result2)
}

fn area_is_out_of_hull(coord1: (i64,i64), coord2 : (i64,i64), hull : &HashSet<(i64,i64)>) -> bool {
    // Go through the border and check if in or out
    let x_range = min(coord1.0, coord2.0)..=max(coord1.0, coord2.0);
    let y_range = min(coord1.1, coord2.1)..=max(coord1.1, coord2.1);

    let horizontal = x_range.clone().zip(iter::repeat(coord1.1))
        .chain(x_range.zip(iter::repeat(coord2.1)));

    let vertical = iter::repeat(coord1.0).zip(y_range.clone())
        .chain(iter::repeat(coord2.0).zip(y_range));

    let mut border = horizontal.chain(vertical);
    let temp = border.any(|coord| point_is_out(coord, hull));
    dbg!(coord1, coord2, temp);
    temp
}

fn point_is_out(coord : (i64,i64), hull : &HashSet<(i64,i64)>) -> bool {
    // check if on border or shoot ray left and count
    dbg!(coord);
    if hull.contains(&coord) {
        dbg!(false);
        false
    } else {

        let mut count = 0;
        let mut ray = coord;
        while ray.0 > 0 {
            if hull.contains(&ray) {
                count += 1;
            }
            ray.0 -= 1;
        }
        dbg!(count%2==0);
        count%2 == 0
    }
}

fn print_hull(hull : &HashSet<(i64,i64)>) {
    println!("=={}==", hull.len());
    print!("  ");
    for k in 0..20 {print!("{k} ")}
    println!();
    for i in 0..20 {
        print!("{i} ");
        for j in 0..20 {
            if hull.contains(&(i as i64,j as i64)) {
                print!("# ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
}

// For tests
fn main(){
    let path = "inputs/day9/final.txt";
    let input = read_to_string(path).expect("Couldn't read file");

    println!("{:?}", day9(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input1(){
        let path = "inputs/day9/1.txt";
        let input = read_to_string(path).expect("Couldn't read file");

        let result = day9(&input); 
        assert_eq!(result, (50,24),
            "Result input 1 : {:?}", result
        );
    }
}

