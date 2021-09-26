fn insertion_sort<T: std::cmp::PartialOrd>(input: &mut [T]) {
    for i in 1..input.len() {
        let mut j = i;
        while j > 0 && input[j] < input[j - 1] {
            input.swap(j - 1, j);
            j = j - 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_numbers() {
        let mut vector = vec![3, 2, 1];
        insertion_sort(&mut vector);

        assert_eq!(vector, vec!(1, 2, 3));
    }

    #[test]
    fn sorts_chars() {
        let mut vector = vec!['c', 'b', 'a'];
        insertion_sort(&mut vector);

        assert_eq!(vector, vec!('a', 'b', 'c'));
    }
}
