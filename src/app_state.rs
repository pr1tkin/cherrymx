use std::cell::RefCell;
use std::rc::Rc;
use gtk::DrawingArea;


pub struct AppState {
    pub color_value: Rc<RefCell<u32>>,
    pub scale_value: Rc<RefCell<u8>>,
    pub mode_value: Rc<RefCell<u8>>,
    pub drawing_area: DrawingArea,
    pub button_modes: Rc<RefCell<Vec<gtk::Button>>>,
    pub observers: Rc<RefCell<Vec<Box<dyn Fn(i32)>>>>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            scale_value: Rc::new(RefCell::new(0x02)),
            mode_value: Rc::new(RefCell::new(0)),
            color_value: Rc::new(RefCell::new(0)),
            button_modes: Rc::new(RefCell::new(Vec::new())),
            drawing_area: DrawingArea::builder()
            .content_width(715)
            .content_height(232)
            .hexpand(false)
            .vexpand(false)
            .build(),
            observers: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub(crate) fn add_observer(&self, observer: Box<dyn Fn(i32)>) {
        self.observers.borrow_mut().push(observer);
    }
    
    pub fn update_mode(&self, mode: u8) {
        *self.mode_value.borrow_mut() = mode;
        for observer in self.observers.borrow().iter() {
            observer(mode.into());
        }
    }
    pub fn update_color(&self, color: u32) {
        *self.color_value.borrow_mut() = color;
        println!("Color: {}", color);
    }
    pub fn update_speed(&self, speed: u8) {
        *self.scale_value.borrow_mut() = speed;
    }

    pub fn add_button_mode(&self, button: gtk::Button) {
        self.button_modes.borrow_mut().push(button);
    }
}