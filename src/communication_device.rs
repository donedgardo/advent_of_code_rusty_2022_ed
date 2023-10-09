use std::collections::HashMap;

pub fn get_first_marker(input: &str) -> Option<usize> {
    get_marker_index(input, 4)
}

pub fn get_message_marker(input: &str) -> Option<usize> {
    get_marker_index(input, 14)
}

fn get_marker_index(input: &str, min: usize) -> Option<usize> {
    for i in 0..input.len()-min {
        if is_unique(&input[i..i+min]) {
            return Some(i+min);
        }
    }
    None
}

pub fn is_unique(input: &str) -> bool {
    let mut characters = HashMap::with_capacity(input.len());
    for c in input.chars() {
        match characters.get(&c) {
            None => {
                characters.insert(c, 1);
            }
            Some(_) => {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod communication_device_test {
    use crate::communication_device::get_message_marker;
    use crate::communication_device::get_first_marker;

    #[test]
    fn it_finds_starting_marker_examples() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let marker = get_first_marker(&input).unwrap();
        assert_eq!(marker, 5);
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let marker = get_first_marker(&input).unwrap();
        assert_eq!(marker, 6);
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let marker = get_first_marker(&input).unwrap();
        assert_eq!(marker, 10);
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let marker = get_first_marker(&input).unwrap();
        assert_eq!(marker, 11);
    }

    #[test]
    fn it_finds_message_marker_examples() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let marker = get_message_marker(&input).unwrap();
        assert_eq!(marker, 19);
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let marker = get_message_marker(&input).unwrap();
        assert_eq!(marker, 23);
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let marker = get_message_marker(&input).unwrap();
        assert_eq!(marker, 23);
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let marker = get_message_marker(&input).unwrap();
        assert_eq!(marker, 29);
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let marker = get_message_marker(&input).unwrap();
        assert_eq!(marker, 26);
    }

    #[test]
    fn it_returns_none_if_no_marker() {
        let input = "aaaaaaaaaa";
        assert!(get_first_marker(&input).is_none());
    }

    mod unique_characters_test {
        use crate::communication_device::is_unique;

        #[test]
        fn it_detects_unique() {
            let input = "abcd";
            assert!(is_unique(&input));
        }

        #[test]
        fn it_detects_non_unique() {
            let input = "aaaa";
            assert!(!is_unique(&input));
        }

    }
}
