use gpui::*;

use crate::{components::{button::Button, card::Card}, views::main_view::MainView};

mod example;
mod components;
mod views;
mod logic;

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
                cx.new(|cx| MainView::new(cx))
            },
        )
        .unwrap();
    });
}
