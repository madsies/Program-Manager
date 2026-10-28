use gpui::*;
use crate::components::card::Card;
use crate::components::button::Button; 
use crate::logic::manager::ProgramManager;

pub struct MainView {
    manager: Entity<ProgramManager>,
}

impl MainView {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let manager = cx.new(|_| ProgramManager::new());

        cx.observe(&manager, |this, _manager, cx| {
            cx.notify();
        }).detach();

        Self { manager }
    }
}

impl Render for MainView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view_handle = self.manager.clone(); 
        let view_handle2 = view_handle.clone();

        // 👇 Directly read fields from the manager entity
        let manager_ref = self.manager.read(cx);
        let count = manager_ref.count;
        let status = manager_ref.status.clone();

        Card::new("hello-card")
            .secondary()
            .h(250.0)
            .w(300.0)
            .child(div().child("Hello, World!"))
            .child(div().child(format!("Counter = {}", count)))
            .child(
                Button::new("submit-btn")
                    .label("Increment the number")
                    .secondary()
                    .large()
                    .on_click(move |_event, _window, cx| {
                        view_handle.update(cx, |this, cx| {
                            this.increment(); // Calls your ProgramManager method
                            cx.notify();
                        });
                    })
            )
            .child(
                Button::new("submit-btn2")
                    .label("Double the number")
                    .primary()
                    .small()
                    .on_click(move |_event, _window, cx| {
                        view_handle2.update(cx, |this, cx| {
                            this.count *= 2;
                            this.status = format!("Doubled to {}", this.count);
                            cx.notify();
                        });
                    })
            )
    }
}