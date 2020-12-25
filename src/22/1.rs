use aoc2020::helper;

pub fn solve(game: &str) -> u64 {
    let mut players: Vec<Vec<u64>> = vec![];
    for line in game.lines().filter(|line| !line.is_empty()) {
        if line.contains("Player") {
            players.push(vec![]);
            continue;
        }

        players
            .last_mut()
            .unwrap()
            .push(line.parse::<u64>().unwrap());
    }

    let mut players_iter = players.iter_mut();
    let first_player = players_iter.next().unwrap();
    let second_player = players_iter.next().unwrap();

    loop {
        if first_player.is_empty() || second_player.is_empty() {
            break;
        }

        let first_card = first_player.remove(0);
        let second_card = second_player.remove(0);
        if first_card > second_card {
            first_player.append(&mut vec![first_card, second_card]);
        } else {
            second_player.append(&mut vec![second_card, first_card]);
        }
    }

    let final_vector = [&first_player[..], &second_player[..]].concat();
    let result: u64 = final_vector
        .iter()
        .enumerate()
        .map(|(index, value)| value * (final_vector.len() - index) as u64)
        .sum();
    return result;
}

fn main() {
    let content = helper::get_input_file("22-input.txt");
    helper::print_answer("22-1", solve(&content));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let content = r#"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"#;

        let result = solve(content);
        assert_eq!(result, 306);
    }
}
