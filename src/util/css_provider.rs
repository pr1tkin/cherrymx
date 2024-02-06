use gtk::gdk;

pub fn setup_css_provider() {
    let button_colors = [
        "#FF5555", "#55FFFF", "#FFFF55", "#FF55FF",
        "#5555FF", "#55FF55", "#FFFFFF", "#FFAAAA",
        "#AAFFAA", "#AAAFFF", "#AAAAAA",
    ];

    let css_color_provider = gtk::CssProvider::new();
    css_color_provider.load_from_data(format!(
        "
          .dialog {{
                border: 1px solid #000;
                box-shadow: 0 2px 4px rgba(0, 0, 0, 0.5);
                background: #fff;
                color: #000;
                padding: 32px 16px;
            }}
            .title-label {{
                font-weight: bold;
                font-size: 32px;
                margin: 32px 16px;
            }}
            .message-label {{
                font-size: 24px;
                margin: 24px 16px;
                color: #a5a3a3;
            }}
            .imagebox{{
                background: #a5a3a3;
                padding: 24px 16px;
            }}
            .destructive-action {{
                background: #e94b78;
                color: #fff;
                padding: 24px 16px;
                border-radius: 16px;
            }}
            .button-apply {{
                margin: 32px 100px;
            }}
            .image-separator {{
                color: #dd0d0d;
                padding-bottom: 10px;
                margin-left: 16px;
                margin-right: 16px;
            }}
            .grid-layer {{
                background: #fff;
                color: #333;
            }}
            .image-container {{
                background: #e4eaf0;
                color: #000;
                border: 1px solid #a5a3a3;
            }}
            .image-small-keyboard {{
                padding: 16px;
                background: #fff;
                margin-top: 32px;
                margin-left: 16px;
                margin-right: 16px;
            }}
            .image-box-label{{
                color: #000;
                padding: 16px;
                background: #fff;
                margin-left: 16px;
                margin-right: 16px;
             }}
            .button-container {{
                background: #f4f6fa;
                padding: 32px;
            }}
            .image-large-keyboard{{
                margin: 16px;
                margin-top: 40px;
                background: #fff;
            }}
            .content-container {{
                background: #fff;
            }}
            .label-box {{
                color: #a5a3a3;
                background: #f4f6fa;
            }}
            .lighting-title {{
                color: #e94b78;
                background: #f4f6fa;
                font-weight: bold;
                margin-top: 32px;
                font-size: 24px;
            }}
            .lighting-subtitle {{
                color: #e94b78;
                background: #f4f6fa;
                margin-top: 8px;
            }}
            .product-title {{
                font-size: 32px;
                font-weight: 600;
                padding: 32px 100px;
            }}
            .button-Settings {{
                margin: 0 250px;
            }}
            .button-speed {{
                font-weight: 600;
            }}
            .lighting-colorpicker-title {{
                font-weight: 600;
            }}
            {}
            ",
        button_colors.iter().enumerate().map(|(i, color)| format!(
            ".color-button-{} {{ background: {}; color: black; min-height: 40px; min-width: 120px; margin: 0 16px;}}", i, color
        )).collect::<Vec<_>>().join("")
    ).as_str());


    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Error initializing GTK CSS provider."),
        &css_color_provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}