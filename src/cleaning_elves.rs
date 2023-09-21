pub fn fully_contains_each_other(range1: (i32, i32), range2: (i32, i32)) -> bool {
    range1.0 <= range2.0 && range1.1 >= range2.1 ||
        range2.0 <= range1.0 && range2.1 >= range1.1
}

pub fn does_partial_overlap(range1: (i32, i32), range2: (i32, i32)) -> bool {
    range1.0 <= range2.1 && range2.0 <= range1.1
}

pub fn filter_pairs(input: &str, operation: fn((i32, i32), (i32, i32)) -> bool ) -> Vec<((i32, i32), (i32, i32))>  {
    let pairs_who_overlap: Vec<((i32, i32), (i32, i32))> =
        process_input_pairs(input)
            .into_iter()
            .filter(|pairs| {
                operation((pairs.0.0, pairs.0.1), (pairs.1.0, pairs.1.1))
            }).collect();
    pairs_who_overlap
}

fn process_input_pairs(input: &str) -> Vec<((i32, i32), (i32, i32))> {
    let pairs: Vec<((i32, i32), (i32, i32))> = input.split("\n")
        .map(|pairs_input| {
            let ranges: Vec<(i32, i32)> = pairs_input.split(",").map(|range_input| {
                let limits: Vec<i32> = range_input.split("-").map(|i| {
                    i.parse::<i32>().unwrap_or(0)
                }).collect();
                assert_eq!(limits.len(), 2);
                (limits[0], limits[1])
            }).collect();
            assert_eq!(ranges.len(), 2);
            (ranges[0], ranges[1])
        }).collect();
    pairs
}


#[cfg(test)]
mod day_4_test {
    use std::fs;
    use crate::cleaning_elves::{does_partial_overlap, filter_pairs, fully_contains_each_other, process_input_pairs};

    #[test]
    fn contains_range() {
        assert!(fully_contains_each_other((0, 0), (0, 0)));
        assert!(!fully_contains_each_other((2, 3), (6, 8)));
        assert!(!fully_contains_each_other((2, 3), (4, 5)));
        assert!(!fully_contains_each_other((5, 7), (7, 9)));
        assert!(fully_contains_each_other((2, 8), (3, 7)));
        assert!(fully_contains_each_other((6, 6), (4, 6)));
    }


    #[test]
    fn process_one_input_pairs_correctly() {
        let input = "2-4,3-3";
        let section_range_pairs = process_input_pairs(input);
        assert_eq!(section_range_pairs.len(), 1);
        assert_eq!(section_range_pairs[0].0, (2, 4));
        assert_eq!(section_range_pairs[0].1, (3, 3));
    }

    #[test]
    fn process_multiple_input_pairs_correctly() {
        let input = "2-4,3-3\n3-3,5-5";
        let section_range_pairs = process_input_pairs(input);
        assert_eq!(section_range_pairs.len(), 2);
        assert_eq!(section_range_pairs[1].0, (3, 3));
        assert_eq!(section_range_pairs[1].1, (5, 5));
    }

    #[test]
    fn solves_sample() -> std::io::Result<()> {
        let input = fs::read_to_string("src/bin/04/sample.txt")?;
        let pairs_who_overlap = filter_pairs(input.as_str(), fully_contains_each_other);
        assert_eq!(pairs_who_overlap.len(), 2);
        Ok(())
    }

    #[test]
    fn partial_overlapping_spec() {
        assert!(does_partial_overlap((0, 0), (0, 0)));
        assert!(does_partial_overlap((0, 0), (0, 0)));
        assert!(!does_partial_overlap((2, 4), (6, 8)));
        assert!(!does_partial_overlap((2, 3), (4, 5)));
        assert!(does_partial_overlap((5, 7), (7, 9)));
        assert!(does_partial_overlap((2, 8), (3, 7)));
        assert!(does_partial_overlap((6, 6), (4, 6)));
        assert!(does_partial_overlap((2, 6), (4, 8)));
    }

    #[test]
    fn solves_sample_2() -> std::io::Result<()> {
        let input = fs::read_to_string("src/bin/04/sample.txt")?;
        let pairs_who_overlap = filter_pairs(input.as_str(), does_partial_overlap);
        assert_eq!(pairs_who_overlap.len(), 4);
        Ok(())
    }
}
