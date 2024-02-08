use std::cell::RefCell;
use gtk::glib::{ControlFlow, SourceId};
use std::rc::Rc;
use std::time::{Duration, Instant};
use gtk::cairo::{Context, Operator};
use gtk::DrawingArea;
use gtk::prelude::*;
use crate::effect::effect::Effect;

pub struct Animator {
    effect: RefCell<Rc<dyn Effect>>,
    start_time: Instant,
    timeout_source_id: Option<SourceId>,
}

impl Animator {

    pub fn new(effect: Rc<dyn Effect>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            effect: RefCell::new(effect),
            start_time: Instant::now(),
            timeout_source_id: None,
        }))
    }

    pub fn set_effect(&self, effect: Rc<dyn Effect>) {
        *self.effect.borrow_mut() = effect;
    }

    pub fn start(&mut self, drawing_area: Rc<RefCell<DrawingArea>>) {
        self.start_time = Instant::now();
        if self.timeout_source_id.is_none() {
            let area_clone = drawing_area.clone();
            let source_id = gtk::glib::timeout_add_local(Duration::from_millis(16), move || {
                area_clone.borrow().queue_draw();
                ControlFlow::Continue
            });
            self.timeout_source_id = Some(source_id);
        }
    }

    pub fn stop(&mut self) {
        if let Some(source_id) = self.timeout_source_id.take() {
            source_id.remove();
        }
    }

    pub fn draw(&self, cr: &Context, color: u32) {
        let time_elapsed = self.start_time.elapsed().as_secs_f64();
        let opacity = self.effect.borrow().update_opacity(time_elapsed);

        let red = ((color >> 16) & 0xFF) as f64 / 255.0;
        let green = ((color >> 8) & 0xFF) as f64 / 255.0;
        let blue = (color & 0xFF) as f64 / 255.0;

        cr.set_operator(Operator::Over);
        cr.set_source_rgba(red, green, blue, opacity);
        cr.set_line_width(3.0);
    }

}
