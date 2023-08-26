// by default generics implement Sized trait
// that is there size is known at compile time
fn generic<T: Sized>(t: T) {

}
// to use generics which works with run time changeable sizes
fn generic<T: ?Sized>(t: T) {
    
}