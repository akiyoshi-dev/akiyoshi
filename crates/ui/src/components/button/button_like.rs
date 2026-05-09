use crate::clickable::Clickable;
use gpui::{
    AnyElement, App, ClickEvent, Div, ElementId, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, StatefulInteractiveElement, Styled, Window, div, px, rgb,
};
use smallvec::SmallVec;
use theme::{Theme, active_theme};

type ClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

#[derive(IntoElement)]
pub struct ButtonLike {
    id: ElementId,
    pub(super) content: Div,
    theme: Theme,
    on_click: Option<ClickHandler>,
    children: SmallVec<[AnyElement; 2]>,
}

impl ButtonLike {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            content: div(),
            theme: active_theme(),
            on_click: None,
            children: SmallVec::new(),
        }
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}

impl RenderOnce for ButtonLike {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = self.theme;
        let mut content = self
            .content
            .id(self.id)
            .bg(rgb(theme.button.background))
            .text_color(rgb(theme.button.text))
            .size(px(theme.button.size))
            .children(self.children);

        if let Some(handler) = self.on_click {
            content = content.on_click(move |event, window, cx| {
                (handler)(event, window, cx);
            });
        }

        content
    }
}

impl Clickable for ButtonLike {
    fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl ParentElement for ButtonLike {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}
