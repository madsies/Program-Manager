use gpui::{prelude::FluentBuilder, *};

use crate::components::button::Button;

pub enum Variant {
    Primary,
    Secondary,
}

#[derive(IntoElement)]
pub struct Card {
    base: Stateful<Div>,
    variant: Variant,
    height: Option<f32>,
    width: Option<f32>,
    hovered: Option<StyleRefinement>,
    pressed: Option<StyleRefinement>,
    fill: Option<(Hsla, Hsla)>,
}

impl Card
{
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: div().id(id),
            variant: Variant::Primary,
            height: None,
            width: None,
            hovered: None,
            pressed: None,
            fill: None
        }
    }

    pub fn child(mut self, child: impl IntoElement) -> Self 
    {
        self.base = self.base.child(child);
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.base = self.base.children(children);
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

    pub fn h(mut self, height:f32) -> Self
    {
        self.height = height.into();
        self
    }

    pub fn w(mut self, width:f32) -> Self
    {
        self.width = width.into();
        self
    }


}


impl RenderOnce for Card {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (bg_color, hover_bg, text_color) = match self.variant {
            Variant::Primary => (rgb(0xf1f1f1), rgb(0xf1f1f1), rgb(0x1b1b1b)),
            Variant::Secondary => (rgb(0x1b1b1b), rgb(0x1b1b1b), rgb(0xf1f1f1)),
        };

        let mut base = self.base
            .flex()
            .flex_col()
            .gap(px(12.0))
            .bg(bg_color)
            .text_color(text_color)
            .rounded_xl()
            .p(px(16.0))
            .shadow_sm()
            .hover(move |style| style.bg(hover_bg));

        if let Some(w) = self.width {
            base = base.w(px(w));
        }

        // Conditionally apply height if set
        if let Some(h) = self.height {
            base = base.h(px(h));
        }

        base
    }
}
