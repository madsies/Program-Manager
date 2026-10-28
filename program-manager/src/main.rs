use gpui::*;

use crate::components::button::Button;

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
        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .size_full()
            .gap(px(16.0))
            .bg(rgb(0x1e1e2e))
            .text_color(rgb(0xcdd6f4))
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

