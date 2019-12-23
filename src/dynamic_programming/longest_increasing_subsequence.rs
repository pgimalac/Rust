pub fn longest_increasing_subsequence<T: PartialOrd + Copy>(list: &[T]) -> Vec<T> {
    longest_increasing_subsequence_by(list, |a, b| a <= b)
}

pub fn longest_strictly_increasing_subsequence<T: PartialOrd + Copy>(list: &[T]) -> Vec<T> {
    longest_increasing_subsequence_by(list, |a, b| a < b)
}

pub fn longest_increasing_subsequence_by<T: Copy, F>(list: &[T], comp: F) -> Vec<T>
where
    F: Fn(&T, &T) -> bool,
{
    if list.is_empty() {
        return Vec::new();
    }
    let mut subseq: Vec<T> = Vec::new();
    subseq.push(*list.first().unwrap());
    for i in list[1..].iter() {
        if comp(i, subseq.last().unwrap()) {
            let mut lower = 0usize;
            let mut upper = subseq.len();
            while lower != upper {
                let middle = (lower + upper) >> 1;
                if comp(&&subseq[middle], i) {
                    lower = middle + 1;
                } else {
                    upper = middle;
                }
            }
            subseq[lower] = *i;
        } else {
            subseq.push(*i);
        }
    }
    subseq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_vec() {
        let index = longest_increasing_subsequence::<usize>(&vec![]);
        assert_eq!(index, vec![]);
    }

    #[test]
    fn single_element() {
        let index = longest_increasing_subsequence(&vec![0]);
        assert_eq!(index, vec![0]);
    }

    #[test]
    fn all_increasing() {
        let list = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let index = longest_increasing_subsequence(&list);
        assert_eq!(index, list);
    }

    #[test]
    fn all_decreasing() {
        let index = longest_increasing_subsequence(&vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
        assert_eq!(index.len(), 1);
    }

    #[test]
    // multiple answers
    fn random_list() {
        let index = longest_increasing_subsequence(&vec![
            0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15,
        ]);
        assert_eq!(index.len(), 6);
    }

    #[test]
    fn random_list2() {
        let index = longest_increasing_subsequence(&vec![0, 2, 3, 5, 7, 4, 5, 6]);
        assert_eq!(index, vec![0, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn longest_decreasing_subsequence() {
        let list = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let index = longest_increasing_subsequence_by(&list, |a, b| a > b);
        assert_eq!(index, list);
    }
}
