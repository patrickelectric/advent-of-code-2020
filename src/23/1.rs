pub fn solution_part_one(vector: &Vec<u64>) -> Vec<u64> {
    let mut split_iter = vector.split(|&n| n == 1);
    let front = split_iter.next().unwrap().clone();
    let back = split_iter.next().unwrap().clone();
    return [&back[..], &front[..]].concat();
}

pub fn solution_part_two(vector: &Vec<u64>) -> u64 {
    let index = vector.iter().position(|&x| x == 1).unwrap();
    return vector[index + 1] * vector[index + 2];
}

pub fn run_cups(cups: &Vec<u64>, moves: u64) -> Vec<u64> {
    let mut cups = cups.clone();

    for round in 0..moves {
        if round % 100 == 0{
            println!("{}/{}", round, moves);
        }
        let current = cups.remove(0);
        let pick_up: Vec<u64> = vec![cups.remove(0), cups.remove(0), cups.remove(0)].into();

        let mut destination = current as i64;
        loop {
            destination -= 1;
            if destination <= 0 {
                destination = cups.iter().max().unwrap().clone() as i64;
            }
            if cups.contains(&(destination as u64)) {
                break;
            }
        }
        let mut split_iter = cups.split(|&n| n == destination as u64);
        let front = split_iter.next().unwrap().clone();
        let back = split_iter.next().unwrap().clone();
        cups = [
            &front[..],
            &[destination as u64],
            &pick_up[..],
            &back[..],
            &[current],
        ]
        .concat();
    }

    return cups;
}

pub fn solve_part_one(cups: &Vec<u64>, moves: u64) -> Vec<u64> {
    return solution_part_one(&run_cups(cups, moves));
}

pub fn solve_part_two(cups: &Vec<u64>, moves: u64) -> u64 {
    return solution_part_two(&run_cups(cups, moves));
}

fn main() {
    let input = "135468729";
    let vector: Vec<u64> = input.bytes().map(|byte| byte - '0' as u8).map(Into::into).collect();
    println!("Answer for 23-1: {:?}", solve_part_one(&vector, 100));

    let mut second_vector = vector.clone();
    second_vector.append(&mut (*vector.iter().max().unwrap()..=1000000).map(|x| x).collect::<Vec<u64>>());

    println!("Answer for 23-2: {:?}", solve_part_one(&vector, 100));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let content = "389125467";
        let vector: Vec<u64> = content.bytes().map(|byte| byte - '0' as u8).map(Into::into).collect();
        assert_eq!(solve_part_one(&vector, 0), vec![2, 5, 4, 6, 7, 3, 8, 9]);
        assert_eq!(solve_part_one(&vector, 1), vec![5, 4, 6, 7, 3, 2, 8, 9]);
        assert_eq!(solve_part_one(&vector, 2), vec![3, 2, 5, 4, 6, 7, 8, 9]);
        assert_eq!(solve_part_one(&vector, 3), vec![3, 4, 6, 7, 2, 5, 8, 9]);
        assert_eq!(solve_part_one(&vector, 4), vec![3, 2, 5, 8, 4, 6, 7, 9]);
        assert_eq!(solve_part_one(&vector, 10), vec![9, 2, 6, 5, 8, 3, 7, 4]);
        assert_eq!(solve_part_one(&vector, 100), vec![6, 7, 3, 8, 4, 5, 2, 9]);

        let mut second_vector = vector.clone();
        second_vector.append(&mut (*vector.iter().max().unwrap()..=1000000).map(|x| x).collect::<Vec<u64>>());
        assert_eq!(solve_part_two(&second_vector, 10000000), 149245887792);
    }
}
