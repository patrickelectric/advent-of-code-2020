use aoc2020::helper;

pub fn calculate_loop_size(public_key: u64, subject_number: u64) -> u64 {
    let mut value = 1;
    let mut count = 0;
    loop {
        value *= subject_number;
        value %= 20201227;
        count += 1;
        if value == public_key {
            break;
        };
    }

    return count;
}

pub fn calculate_encryption_key(loop_size: u64, public_key: u64) -> u64 {
    let mut value = 1;
    let subject_number = public_key;
    for _ in 0..loop_size {
        value *= subject_number;
        value %= 20201227;
    }
    return value;
}

fn main() {
    let card_public_key = 17607508;
    let door_public_key = 15065270;

    let card_loop_size = calculate_loop_size(card_public_key, 7);
    let door_loop_size = calculate_loop_size(door_public_key, 7);
    let encryption_key_1 = calculate_encryption_key(door_loop_size, card_public_key);
    let encryption_key_2 = calculate_encryption_key(card_loop_size, door_public_key);
    assert_eq!(encryption_key_1, encryption_key_2);

    helper::print_answer("25-1", encryption_key_1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let card_loop_size = 8;
        let door_loop_size = 11;
        let card_public_key = 5764801;
        let door_public_key = 17807724;
        assert_eq!(calculate_loop_size(card_public_key, 7), card_loop_size);
        assert_eq!(calculate_loop_size(door_public_key, 7), door_loop_size);

        let encryption_key = 14897079;
        assert_eq!(calculate_encryption_key(door_loop_size, card_public_key), encryption_key);
        assert_eq!(calculate_encryption_key(card_loop_size, door_public_key), encryption_key);
    }
}
