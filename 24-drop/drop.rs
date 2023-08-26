

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}", self.data);
    }
}

fn main() {
    // called deref in rev order
    let c = CustomSmartPointer {
        data: String::from("my stuff")
    };

    // manually call drop function
    drop(c);
    
    let d = CustomSmartPointer {
        data: String::from("other stuff")
    };
}