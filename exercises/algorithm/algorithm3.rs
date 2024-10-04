/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: Ord>(array: &mut [T]) {
    // 插入排序算法
    let len = array.len();
    for i in 1..len {
        let mut j = i;
        // 将 array[i] 插入到已排序的子序列 array[0..i] 中
        while j > 0 && array[j - 1] > array[j] {
            array.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }

    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }

    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }

    #[test]
    fn test_sort_empty() {
        let mut vec: Vec<i32> = Vec::new();
        sort(&mut vec);
        assert_eq!(vec, vec![]);
    }

    #[test]
    fn test_sort_already_sorted() {
        let mut vec = vec![1, 2, 3, 4, 5];
        sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_sort_with_duplicates() {
        let mut vec = vec![5, 3, 8, 3, 9, 1, 5];
        sort(&mut vec);
        assert_eq!(vec, vec![1, 3, 3, 5, 5, 8, 9]);
    }
}
