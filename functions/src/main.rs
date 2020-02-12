fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let list = vec![23, 53, -213, 546364, 14];

    let result = largest(&list);

    println!("The largest number is {}", result);
}