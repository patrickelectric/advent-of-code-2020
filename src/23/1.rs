pub fn run_cups(cups: &Vec<u8>, moves: u64) -> Vec<u8> {
    let mut cups = cups.clone();

    for _ in 0..moves {
        let current = cups.remove(0);
        let pick_up: Vec<u8> = vec![cups.remove(0), cups.remove(0), cups.remove(0)].into();

        let mut destination = current as i64;
        loop {
            destination -= 1;
            if destination <= 0 {
                destination = cups.iter().max().unwrap().clone() as i64;
            }
            if cups.contains(&(destination as u8)) {
                break;
            }
        }
        let mut split_iter = cups.split(|&n| n == destination as u8);
        let front = split_iter.next().unwrap().clone();
        let back = split_iter.next().unwrap().clone();
        cups = [
            &front[..],
            &[destination as u8],
            &pick_up[..],
            &back[..],
            &[current],
        ]
        .concat();
    }

    let cups: Vec<u8> = cups.into();
    let mut split_iter = cups.split(|&n| n == 1);
    let front = split_iter.next().unwrap().clone();
    let back = split_iter.next().unwrap().clone();
    return [&back[..], &front[..]].concat();
}

fn main() {
    let input = "135468729";
    let vector: Vec<u8> = input.bytes().map(|byte| byte - '0' as u8).collect();
    println!("Answer for 23-1: {:?}", run_cups(&vector, 100));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let content = "389125467";
        let vector: Vec<u8> = content.bytes().map(|byte| byte - '0' as u8).collect();
        assert_eq!(run_cups(&vector, 0), vec![2, 5, 4, 6, 7, 3, 8, 9]);
        assert_eq!(run_cups(&vector, 1), vec![5, 4, 6, 7, 3, 2, 8, 9]);
        assert_eq!(run_cups(&vector, 2), vec![3, 2, 5, 4, 6, 7, 8, 9]);
        assert_eq!(run_cups(&vector, 3), vec![3, 4, 6, 7, 2, 5, 8, 9]);
        assert_eq!(run_cups(&vector, 4), vec![3, 2, 5, 8, 4, 6, 7, 9]);
        assert_eq!(run_cups(&vector, 10), vec![9, 2, 6, 5, 8, 3, 7, 4]);
        assert_eq!(run_cups(&vector, 100), vec![6, 7, 3, 8, 4, 5, 2, 9]);
    }
}
