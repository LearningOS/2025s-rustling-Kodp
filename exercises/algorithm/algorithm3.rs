/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: Ord + Clone>(arr: &mut [T]){
	if arr.len() <= 1 {
        return;
    }
    let piv = partition(arr);
    let (l, r) = arr.split_at_mut(piv);
    sort(l);
    if r.len() > 1 {
        sort(&mut r[1..]);
    }
}

fn partition<T: Ord + Clone>(arr: &mut [T]) -> usize {
    let pivot_index = arr.len() - 1;
    let pivot = arr[pivot_index].clone();
    let mut i = 0;
    // 遍历除 pivot 外的所有元素, 小于等于 pivot 的交换到左侧
    for j in 0..pivot_index {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    // 将 pivot 放到正确位置
    arr.swap(i, pivot_index);
    i
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
}