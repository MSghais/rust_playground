pub fn count_sheep_bool_simple_loop(sheep: &[bool]) -> u8 {
    let mut count: u8 = 0;
    for s in sheep.iter() {
        if *s == true {
            count = count + 1;
        }
    }
    count
}

pub fn sheep_filter_and_count(sheep: &[bool]) -> u8 {
    sheep // take the sheep array
        .iter() // turn it into an iterable
        .filter(|&&x| x) // filter it by taking the values in the array and returning only the true ones
        .count() as u8 // count all of the elements in the filtered array and return a u8
}

#[cfg(test)]
pub mod count_sheep_bool_tests {
    use super::*;

    #[test]
    pub fn test_3() {
        assert_eq!(count_sheep_bool_simple_loop(&[true, false, true, true, false]), 3);
        assert_eq!(sheep_filter_and_count(&[true, false, true, true, false]), 3);
    }
    #[test]
    pub fn test_6() {
        assert_eq!(
            count_sheep_bool_simple_loop(&[true, false, true, true, false, true, true, true]),
            6
        );
        assert_eq!(
            sheep_filter_and_count(&[true, false, true, true, false, true, true, true]),
            6
        );
    }
}
