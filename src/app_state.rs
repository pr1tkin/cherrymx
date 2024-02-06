use std::cell::RefCell;
use std::rc::Rc;

pub struct AppState {
    pub scale_value: Rc<RefCell<u8>>,
    pub mode_value: Rc<RefCell<u8>>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            scale_value: Rc::new(RefCell::new(0)),
            mode_value: Rc::new(RefCell::new(0)),
        }
    }

    pub fn update_mode(&self, mode: u8) {
        *self.mode_value.borrow_mut() = mode;
    }
}