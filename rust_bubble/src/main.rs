fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn bubble_sort_t<T>(arr: &mut [T])
    where
        T: PartialOrd,
{
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    // 基础冒泡排序
    let mut arr = [64, 34, 25, 12, 22, 11, 90];
    bubble_sort(&mut arr);
    println!("基础冒泡排序: {:?}", arr);

    // 泛型PartialOrd
    let mut arr = [64, 34, 25, 12, 22, 11, 90];
    bubble_sort_t(&mut arr);
    println!("泛型PartialOrd: {:?}", arr);

    // 泛型PartialOrd
    let mut arr_float = [64.0, 34.0, 25.0, 12.0, 22.0, 11.0, 90.0];
    bubble_sort_t(&mut arr_float);
    println!("泛型PartialOrd: {:?}", arr_float);
}
