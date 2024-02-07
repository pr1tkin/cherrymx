use std::cell::RefCell;
use std::rc::Rc;
use gtk::DrawingArea;


pub struct AppState {
    pub color_value: Rc<RefCell<u32>>,
    pub scale_value: Rc<RefCell<u8>>,
    pub mode_value: Rc<RefCell<u8>>,
    pub drawing_area: DrawingArea
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            scale_value: Rc::new(RefCell::new(0)),
            mode_value: Rc::new(RefCell::new(0)),
            color_value: Rc::new(RefCell::new(0)),
            drawing_area: DrawingArea::builder()
            .content_width(715)
            .content_height(232)
            .hexpand(true)
            .vexpand(true)
            .build()
        }
    }

    pub fn update_mode(&self, mode: u8) {
        *self.mode_value.borrow_mut() = mode;
    }
    pub fn update_color(&self, color: u32) {
        *self.color_value.borrow_mut() = color;
        println!("Color: {}", color);
    }
    pub fn update_speed(&self, mode: u8) {
        *self.scale_value.borrow_mut() = mode;
    }
}