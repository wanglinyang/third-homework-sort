fn main() {
    let mut arr = vec!["ab","cd","abc","ac"];
    bubble_sort(arr.as_mut_slice(), Some(|a:&&str, b:&&str| a.partial_cmp(b)));
    println!("{:?}", arr);
}

fn bubble_sort<T, F>(arr: &mut [T], compare: Option<F>)
where
    T: PartialOrd,
    F: Fn(&T, &T) -> Option<std::cmp::Ordering>,
{
    let len = arr.len();

    for i in 0..len {
        for j in 0..len - i - 1 {
            if let Some(ref cmp) = compare {
                if let Some(ordering) = cmp(&arr[j], &arr[j + 1]) {
                    if ordering == std::cmp::Ordering::Greater {
                        arr.swap(j, j + 1);
                    }
                }
            } else {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut arr = vec!["ab", "cd", "abc", "ac"];
        bubble_sort(arr.as_mut_slice(), Some(|a: &&str, b: &&str| a.partial_cmp(b)));
        assert_eq!(arr, vec!["ab", "abc", "ac", "cd"]);

        let mut arr2 = vec![4, 2, 9, 1, 5];
        bubble_sort(arr2.as_mut_slice(), Some(|a: &i32, b: &i32| a.partial_cmp(b)));
        assert_eq!(arr2, vec![1, 2, 4, 5, 9]);

        let mut arr3 = vec![true, false, true, false];
        bubble_sort(arr3.as_mut_slice(), Some(|a: &bool, b: &bool| a.partial_cmp(b)));
        assert_eq!(arr3, vec![false, false, true, true]);
    }
}