// whenever the crate in which this macro is defined in is brought into scope
// this macro should be made available
#[macro_export]
// vec is name of macro
macro_rules! vec {
    // match expr with one arm
    // $x of type expr
    // * means can repeat with ,
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            // generate this code for every match we get
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}