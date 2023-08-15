fn main() {
    let number_list = vec![34, 5, 25, 100, 65];

    let largest = get_largest(number_list);

    println!("The largest number is {}", largest);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let largest = get_largest(char_list);

    println!("The largest char is {}", largest);

}

// multiple generic types like <T, U>
// generic Type T should implement a trait to be orderd and copied
fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];

    for number in number_list {
        if (number > largest) {
            largest = number;
        }
    }

    return largest;
}