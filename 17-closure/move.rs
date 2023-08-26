fn main() {
    let x = vec![1, 2, 3];

    // move to force take ownership of x
    let equal_to_x = move | z | {
        return z = x;
    };

    //println!("cannot access x here, cause closure took ownership of x: {:?}", x);
}