use gpui::*;

mod example;

fn main() {
    example::example();

    gpui_platform::application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1280.), px(720.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                        title: Some("Program Manager".into()),
                        appears_transparent:true,
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
        let view_handle = cx.entity();

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
                Button {
                    id: "counter-button".into(),
                    label: "Click Me!".into(),
                    width: 140.0,
                    height: 44.0,
                    on_click: Box::new(move |window, cx| {
                        view_handle.update(cx, |this, cx| {
                            this.count += 1;
                            this.text = format!("Clicked {} times!", this.count).into();
                            cx.notify();
                        });
                    }),
                }
            )
            .child(format!("Status: {}", self.text))
    }
}

#[derive(IntoElement)]
struct Button {
    id: ElementId,
    label: SharedString,
    on_click: Box<dyn Fn(&mut Window, &mut App)>,
    width: f32,
    height: f32,
}

impl RenderOnce for Button {
    fn render(mut self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .cursor_pointer()
            .w(px(self.width))
            .h(px(self.height))
            .bg(rgb(0x4a5568))
            .text_color(rgb(0xffffff))
            .rounded_md()
            .on_mouse_down(MouseButton::Left, move |_event, window, cx| {
                println!("Button clicked!"); 
                (self.on_click)(window, cx);
            })
            .child(self.label)
    }
}