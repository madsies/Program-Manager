use gpui::*;

use crate::components::{button::Button, card::Card};

mod example;
mod components;

fn main() {
    example::example();
    

    gpui_platform::application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1280.), px(720.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                        title: Some("Program Manager".into()),
                        appears_transparent:false,
                        traffic_light_position: None,
                    }),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| HelloWorld {
                    text: "Not clicked yet".into(),
                    count: 0,
                })
            },
        )
        .unwrap();
    });
}

struct HelloWorld {
    text: SharedString,
    count: i32,
}

impl Render for HelloWorld {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Card::new("hello-card")
            .secondary()
            .h(250.0)
            .w(300.0)
            .child(
                Button::new("submit-btn")
                    .label("Button 1")
                    .secondary()
                    .large()
                    .on_click(|event, window, cx| {
                        println!("Button 1 clicked successfully!");
                    })
            )
            .child(
                Button::new("submit-btn2")
                    .label("Button 2")
                    .primary()
                    .small()
                    .on_click(|event, window, cx| {
                        println!("Button 2 clicked successfully!");
                    })
            )
    }
}

