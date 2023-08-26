use std::cell::RefCell;

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T> 
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        return LimitTracker {
            messenger, 
            value: 0,
            max,
        };
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urget warning: You have used up over 90% of quota");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You have used up 75% of your quota");
        }
    }
}

struct MockMessenger {
    sent_message: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        return MockMessenger {
            sent_message: RefCell::new(vec![]),
        }
    }
}

impl Messenger for MockMessenger {
    // modifing a field in an immutable struct
    // not allowed use Cell
    // fn send(&self, message: &str) {
    //     self.sent_message.push(String::from(message));
    // }

    // sol
    fn send(&self, message: &str) {
        // borrow_mut on refcell to get mutable ref
        self.sent_message.borrow_mut().push(String::from(message))
    }
}

fn main() {
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_value(80);

    assert_eq!(mock_messenger.sent_message.borrow().len(), 1);
}