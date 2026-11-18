use std::{cmp::{max, min}, collections::HashSet, fs::read_to_string, iter};

fn day9(input : &str)-> (i64,i64){

    // get coords
    let mut coords : Vec<(i64, i64)> = input.trim().lines().map(|line| {
        let (x_s,y_s) = line.trim().split_once(',').expect("Unable to parse");
        let (x,y) : (i64,i64) = (x_s.parse().expect("Not an int"), y_s.parse().expect("Not an int"));
        (x,y)
    }).collect();
    let mut hull = coords.clone();
    hull.push(coords[0]);
    coords.sort();

    // Complete convex hull sides
    let mut hull_set = HashSet::new();
    for coord_packed in hull.windows(2) {
        if let &[coord1,coord2] = coord_packed {
            for dx in min(coord1.0,coord2.0)..= max(coord1.0,coord2.0) {
                for dy in min(coord1.1,coord2.1)..=max(coord1.1,coord2.1){
                    hull_set.insert((dx,dy));

                }
            }
        }
    }

    // result 1 
    let mut result1 = 0;
    let mut result2 = 0;
    for i in 0..coords.len() {
        //println!("{i}");
        for j in (i+1)..coords.len() {
            //dbg!(j);
            let dx = (coords[j].0 - coords[i].0).abs() + 1;
            let dy = (coords[j].1 - coords[i].1).abs() + 1;
            result1 = result1.max(dx*dy);
            if !area_is_out_of_hull(coords[i], coords[j], &hull) {
                result2 = result2.max(dx*dy);
            }
        }
    }

    (result1,result2)
}
fn area_is_out_of_hull(coord1: (i64, i64),coord2: (i64, i64),hull_ordered: &[(i64, i64)]) -> bool {
    let min_x = min(coord1.0, coord2.0);
    let max_x = max(coord1.0, coord2.0);
    let min_y = min(coord1.1, coord2.1);
    let max_y = max(coord1.1, coord2.1);

    // 1. All 4 corners must be inside or on the border of the hull
    let corners = [
        (min_x, min_y),
        (min_x, max_y),
        (max_x, min_y),
        (max_x, max_y),
    ];
    if corners.iter().any(|&c| point_is_out(c, hull_ordered)) {
        return true;
    }

    // 2. Check that no hull edge cuts through the strict interior of the rectangle
    let n = hull_ordered.len()-1;
    for i in 0..n {
        let (x1, y1) = hull_ordered[i];
        let (x2, y2) = hull_ordered[i + 1];

        let edge_min_x = min(x1, x2);
        let edge_max_x = max(x1, x2);
        let edge_min_y = min(y1, y2);
        let edge_max_y = max(y1, y2);

        if x1 == x2 {
            // Vertical hull edge: cuts through if its X is strictly inside (min_x, max_x)
            // and its Y-span overlaps the rectangle's Y-span (min_y, max_y)
            if x1 > min_x && x1 < max_x && edge_max_y > min_y && edge_min_y < max_y {
                return true;
            }
        } else if y1 == y2 {
            // Horizontal hull edge: cuts through if its Y is strictly inside (min_y, max_y)
            // and its X-span overlaps the rectangle's X-span (min_x, max_x)
            if y1 > min_y && y1 < max_y && edge_max_x > min_x && edge_min_x < max_x {
                return true;
            }
        }
    }

    false
}

fn point_is_out(coord: (i64, i64), vertices: &[(i64, i64)]) -> bool {
    let (px, py) = coord;
    let mut inside = false;
    let n = vertices.len();

    for i in 0..n {
        let (x1, y1) = vertices[i];
        let (x2, y2) = vertices[(i + 1) % n];

        // 1. Check if on an orthogonal border segment
        if x1 == x2 && px == x1 && py >= y1.min(y2) && py <= y1.max(y2) {
            return false;
        }
        if y1 == y2 && py == y1 && px >= x1.min(x2) && px <= x1.max(x2) {
            return false;
        }

        // 2. Half-open ray casting check (shooting ray left)
        if (y1 > py) != (y2 > py) {
            if px > x1 {
                inside = !inside;
            }
        }
    }

    !inside
}

fn print_hull(hull : &HashSet<(i64,i64)>, mark : Vec<(i64,i64)>) {
    println!("=={}==", hull.len());
    print!("  ");
    for k in 0..20 {print!("{k} ")}
    println!();
    for i in 0..20 {
        print!("{i} ");
        for j in 0..20 {
            if mark.contains(&(i as i64, j as i64)) {
                print!("X ");
            }else if hull.contains(&(i as i64,j as i64)) {
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

