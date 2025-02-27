mod limit;
use limit::*;
use std::cell::RefCell;

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> Self {
            Self {
                messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn over_75_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut tracker = LimitTracker::new(&mock_messenger, 10);
        tracker.set_value(8);
        assert_eq!(mock_messenger.messages.borrow().len(), 1);
    }
}
