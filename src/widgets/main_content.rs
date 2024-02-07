use gtk::gdk_pixbuf::Pixbuf;
use gtk::{Button, gdk_pixbuf, Label, Orientation, Overlay, pango, Picture};
use gtk::cairo::{Context, Operator};
use gtk::prelude::{BoxExt, DrawingAreaExtManual, WidgetExt};
use crate::app_state::AppState;

pub fn create_main_content_box(app_state: &AppState) -> gtk::Box {
    let large_content_box = gtk::Box::new(Orientation::Vertical, 0);
    large_content_box.set_css_classes(&["content-container"]);
    large_content_box.set_size_request(715, 232);
    let pixbuf = Pixbuf::from_file("keyboard-large.png")
        .expect("Could not load the image file");
    let scaled_pixbuf = pixbuf.scale_simple(572, 185, gdk_pixbuf::InterpType::Bilinear)
        .expect("Could not scale the image");
    let picture = Picture::for_pixbuf(&scaled_pixbuf);
    picture.set_can_shrink(false);
    picture.set_keep_aspect_ratio(true);
    picture.set_css_classes(&["image-large-keyboard"]);

    let overlay = Overlay::builder().build();
    overlay.add_overlay(&picture);

    let drawing_area = app_state.drawing_area.clone();

    type DrawOperation = Box<dyn Fn(&Context)>;

    let draw_operations: Vec<DrawOperation> = vec![
        Box::new(|cr: &Context| {
            cr.rectangle(78.0, 46.0, 35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(113.0, 46.0,  35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(148.0, 46.0,  35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(183.0, 46.0,  35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(218.0, 46.0,  35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(253.0, 46.0,  35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(288.0, 46.0,  35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(323.0, 46.0,  35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(358.0, 46.0,  35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(393.0, 46.0,  35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(428.0, 46.0,  35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(463.0, 46.0,  35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(498.0, 46.0,  35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(533.0, 46.0,  70.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(603.0, 46.0,  35.0, 35.0);
            cr.stroke().unwrap();
        }),
         Box::new(|cr: &Context| {
            cr.rectangle(78.0, 83.0, 53.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(131.0, 83.0, 35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(166.0, 83.0,  35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(201.0, 83.0,  35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(236.0, 83.0,  35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(271.0, 83.0,  35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(306.0, 83.0,  35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle (341.0, 83.0,  35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(376.0, 83.0,  35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(411.0, 83.0,  35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(446.0, 83.0,  35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(481.0, 83.0,  35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(516.0, 83.0,  35.0, 35.0);
            cr.stroke().unwrap();
        }),
        Box::new(|cr: &Context| {
            cr.move_to(551.0, 83.0);
            cr.line_to(604.0, 83.0);
            cr.stroke().unwrap();

            cr.move_to(604.0, 83.0);
            cr.line_to(604.0, 153.0);
            cr.stroke().unwrap();

            cr.move_to(604.0, 153.0);
            cr.line_to(560.0, 153.0);
            cr.stroke().unwrap();

            cr.move_to(560.0, 153.0);
            cr.line_to(560.0, 118.0);
            cr.stroke().unwrap();

            cr.move_to(560.0, 118.0);
            cr.line_to(550.0, 118.0);
            cr.stroke().unwrap();
        }),
        Box::new(|cr: &Context| {
            cr.rectangle(604.0, 83.0,  35.0, 35.0);
            cr.stroke().unwrap();
        }),
        Box::new(|cr: &Context| {
            cr.rectangle(78.0, 118.0, 62.0, 35.0);
            cr.stroke().unwrap();
              cr.rectangle(140.0, 118.0, 35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(175.0, 118.0, 35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(210.0, 118.0, 35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(245.0, 118.0, 35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(280.0, 118.0, 35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(315.0, 118.0, 35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(350.0, 118.0, 35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(385.0, 118.0, 35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(420.0, 118.0, 35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(455.0, 118.0, 35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(490.0, 118.0, 35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(525.0, 118.0, 35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(604.0, 118.0, 35.0, 35.0);
            cr.stroke().unwrap();
        }),
        Box::new(|cr: &Context| {
            cr.rectangle(78.0, 153.0, 43.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(121.0, 153.0, 35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(156.0, 153.0, 35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(191.0, 153.0, 35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(226.0, 153.0, 35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(261.0, 153.0, 35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(296.0, 153.0, 35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(331.0, 153.0, 35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(366.0, 153.0, 35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(401.0, 153.0, 35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(436.0, 153.0, 35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(471.0, 153.0, 35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(506.0, 153.0, 62.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(568.0, 153.0, 35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(603.0, 153.0, 35.0, 35.0);
            cr.stroke().unwrap();
        }),
        Box::new(|cr: &Context| {
            cr.rectangle(78.0, 188.0, 43.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(121.0, 188.0, 43.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(164.0, 188.0, 43.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(207.0, 188.0, 220.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(427.0, 188.0, 35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(462.0, 188.0, 35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(497.0, 188.0, 35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(532.0, 188.0, 35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(567.0, 188.0, 35.0, 35.0);
            cr.stroke().unwrap();
            cr.rectangle(602.0, 188.0, 35.0, 35.0);
            cr.stroke().unwrap();
        }),
        Box::new(|cr: &Context| {
            cr.rectangle(78.0, 46.0, 562.0, 178.0);
            cr.fill().unwrap();
        }),
    ];

    let color_state = app_state.color_value.clone();
    drawing_area.set_draw_func(move |_, cr, _, _| {
        let color_value = *color_state.borrow();
        let red = ((color_value >> 16) & 0xFF) as f64 / 255.0;
        let green = ((color_value >> 8) & 0xFF) as f64 / 255.0;
        let blue = (color_value & 0xFF) as f64 / 255.0;
        cr.set_operator(Operator::Over);
        cr.set_source_rgba(red, green, blue, 0.5);
        cr.set_line_width(3.0);
        for draw_op in &draw_operations {
            draw_op(cr);
        }
    });



    overlay.set_margin_bottom(250);
    overlay.add_overlay(&drawing_area);
    large_content_box.append(&overlay);

    let label4 = Label::new(Some("MX-LP 2.1 Compact Wireless Mechanical Keyboard"));
    label4.set_css_classes(&["product-title"]);
    label4.set_wrap(true);
    label4.set_wrap_mode(pango::WrapMode::WordChar);
    label4.set_max_width_chars(15);
    label4.set_justify(gtk::Justification::Center);
    large_content_box.append(&label4);

    let large_content = Button::with_label("KB Setting");
    large_content.set_css_classes(&["button-Settings"]);
    large_content_box.append(&large_content);

    large_content_box
}