use gpui::{prelude::FluentBuilder, *};

pub enum Variant {
    Primary,
    Secondary,
}

pub enum Size {
    Small,
    Medium,
    Large,
}

type Click = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

#[derive(IntoElement)]
pub struct Button {
    base: Stateful<Div>,
    label: Option<SharedString>,
    on_click: Option<Click>,
    disabled: bool,
    selected: bool,
    variant: Variant,
    size: Size,
    hovered: Option<StyleRefinement>,
    pressed: Option<StyleRefinement>,
    fill: Option<(Hsla, Hsla)>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: div().id(id),
            label: None,
            disabled: false,
            selected: false,
            variant: Variant::Primary,
            size: Size::Medium,
            hovered: None,
            pressed: None,
            fill: None,
            on_click: None,
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn primary(mut self) -> Self {
        self.variant = Variant::Primary;
        self
    }

    pub fn secondary(mut self) -> Self {
        self.variant = Variant::Secondary;
        self
    }

    pub fn small(mut self) -> Self {
        self.size = Size::Small;
        self
    }

    pub fn medium(mut self) -> Self {
        self.size = Size::Medium;
        self
    }

    pub fn large(mut self) -> Self {
        self.size = Size::Large;
        self
    }

    pub fn fill(mut self, background: Hsla, hover: Hsla) -> Self {
        self.fill = Some((background, hover));
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (height, px_x, text_size) = match self.size {
            Size::Small => (px(28.0), px(12.0), px(12.0)),
            Size::Medium => (px(36.0), px(16.0), px(14.0)),
            Size::Large => (px(44.0), px(20.0), px(16.0)),
        };

        let (bg_color, hover_bg, text_color) = match self.variant {
            Variant::Primary => (rgb(0x3b82f6), rgb(0x2563eb), rgb(0xffffff)),
            Variant::Secondary => (rgb(0xe5e7eb), rgb(0xd1d5db), rgb(0x1f2937)),
        };

        let (bg, hover) = if let Some((custom_bg, custom_hover)) = self.fill {
            (custom_bg, custom_hover)
        } else {
            (bg_color.into(), hover_bg.into())
        };

        let mut base = self.base
            .flex()
            .items_center()
            .justify_center()
            .h(height)
            .px(px_x)
            .rounded_md()
            .text_size(text_size)
            .text_color(text_color)
            .bg(bg)
            .when(!self.disabled, |div| {
                div.cursor_pointer()
                   .hover(move |style| style.bg(hover))
            })
            .when(self.disabled, |div| {
                div.opacity(0.5)
                   .cursor_not_allowed()
            });

        if !self.disabled {
            if let Some(handler) = self.on_click {
                base = base.on_click(move |event, window, cx| {
                    (handler)(event, window, cx);
                });
            }
        }

        base.when_some(self.label, |div, label| div.child(label))
    }
}