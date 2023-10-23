fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8];
    let num = 5;
    let index = binary_search(&arr, num);
    println!("The index of {} is {}", num, index);
}

fn binary_search(arr: &[i32], num: i32) -> i32 {
    let mut low = 0;
    let mut high = arr.len() - 1;
    let mut mid;

    while low <= high {
        mid = (low + high) / 2;
        if arr[mid] == num {
            return mid as i32;
        } else if arr[mid] > num {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    return -1;
}
