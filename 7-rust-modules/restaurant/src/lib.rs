// lib crate

fn serve_order() {}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        
        fn service_order() {}
        
        fn take_payment() {}

        fn cook_order() {}
        
        fn fix_incorrect_order() {
            cook_order();
            super::super::serve_order();
        }
    }
}

pub fn eat_at_restaurant() {
    // abs path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();
}