pub mod messenger;
pub use crate::messenger::*;
pub use std::cell::RefCell;
pub use std::collections::HashMap;
pub use std::rc::Rc;

pub struct Worker {
    pub track_value: Rc<usize>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
    pub all_messages: RefCell<Vec<String>>,
}

impl Worker {
    pub fn new(value: usize) -> Self {
        Self {
            track_value: Rc::new(value),
            mapped_messages: RefCell::new(HashMap::new()),
            all_messages: RefCell::new(Vec::new()),
        }
    }
}

impl messenger::Logger for Worker {
    fn warning(&self, msg: &str) {
        self.all_messages.borrow_mut().push(String::from(msg));
        self.mapped_messages.borrow_mut().insert(
            String::from("Warning"),
            String::from(msg.strip_prefix("Warning: ").unwrap()),
        );
    }

    fn error(&self, msg: &str) {
        self.all_messages.borrow_mut().push(String::from(msg));
        self.mapped_messages.borrow_mut().insert(
            String::from("Error"),
            String::from(msg.strip_prefix("Error: ").unwrap()),
        );
    }

    fn info(&self, msg: &str) {
        self.all_messages.borrow_mut().push(String::from(msg));
        self.mapped_messages.borrow_mut().insert(
            String::from("Info"),
            String::from(msg.strip_prefix("Info: ").unwrap()),
        );
    }
}
