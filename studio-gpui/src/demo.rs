use gpui_kit::component::ActiveTheme as _;
use gpui_kit::component::carousel::{
    Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPrevious, CarouselState,
};
use gpui_kit::{
    AnyView, AppContext, Context, Entity, InteractiveElement, IntoElement, ParentElement, Render,
    StatefulInteractiveElement, Styled, Window, div, px, relative,
};

use crate::editor::{Generation, GenerationFailed, GenerationProgress, GenerationSuccess};

// -----------------------------------------------------------------------------
// Gallery data
// -----------------------------------------------------------------------------

#[derive(Clone)]
pub struct GalleryItem {
    title: &'static str,
    subtitle: &'static str,
    view: AnyView,
}

impl GalleryItem {
    pub fn new(title: &'static str, subtitle: &'static str, view: AnyView) -> Self {
        Self {
            title,
            subtitle,
            view,
        }
    }
}

pub struct GallerySection {
    id: &'static str,
    title: &'static str,
    description: &'static str,
    items: Vec<GalleryItem>,
    carousel: Entity<CarouselState>,
}

impl GallerySection {
    pub fn new(
        id: &'static str,
        title: &'static str,
        description: &'static str,
        items: Vec<GalleryItem>,
        cx: &mut Context<DemoApp>,
    ) -> Self {
        let carousel = cx.new(|_| CarouselState::new(items.len()));

        Self {
            id,
            title,
            description,
            items,
            carousel,
        }
    }
}

// -----------------------------------------------------------------------------
// Component gallery
// -----------------------------------------------------------------------------

pub struct ComponentGallery {
    sections: Vec<GallerySection>,
}

impl ComponentGallery {
    pub fn new(sections: Vec<GallerySection>) -> Self {
        Self { sections }
    }

    fn render_section_header(section: &GallerySection, cx: &Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        div()
            .flex()
            .items_end()
            .justify_between()
            .gap_4()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_lg()
                            .text_color(theme.foreground)
                            .child(section.title),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.muted_foreground)
                            .child(section.description),
                    ),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(theme.muted_foreground)
                    .child(format!("{:02}", section.items.len())),
            )
    }

    fn render_preview(item: &GalleryItem, cx: &Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        div()
            .w_full()
            .h_full()
            .min_w_0()
            .min_h_0()
            .p_4()
            .rounded_xl()
            .border_1()
            .border_color(theme.border)
            .bg(theme.background)
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div().flex().items_start().justify_between().gap_3().child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(
                            div()
                                .text_sm()
                                .text_color(theme.foreground)
                                .child(item.title),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.muted_foreground)
                                .child(item.subtitle),
                        ),
                ),
            )
            .child(
                div()
                    .w_full()
                    .flex_1()
                    .min_w_0()
                    .min_h_0()
                    .flex()
                    .flex_col()
                    .justify_center()
                    .items_stretch()
                    .child(item.view.clone()),
            )
    }

    fn render_section(section: &GallerySection, cx: &Context<Self>) -> impl IntoElement {
        let id = section.id;
        let carousel = &section.carousel;

        div()
            .w_full()
            .min_w_0()
            .flex()
            .flex_col()
            .gap_4()
            .child(Self::render_section_header(section, cx))
            .child(
                Carousel::new(id, carousel)
                    .w_full()
                    .min_w_0()
                    .gap_3()
                    .child(
                        CarouselContent::new(carousel)
                            .w_full()
                            .min_w_0()
                            .items_stretch()
                            .track_style(gpui_kit::StyleRefinement::default().ml_neg_3())
                            .children(section.items.iter().enumerate().map(|(index, item)| {
                                CarouselItem::new((id, index), index, carousel)
                                    .flex_basis(relative(0.40))
                                    .max_w(px(760.0))
                                    .h_full()
                                    .min_w_0()
                                    .pl_3()
                                    .flex()
                                    .child(Self::render_preview(item, cx))
                            })),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_end()
                            .gap_2()
                            .child(CarouselPrevious::new(carousel))
                            .child(CarouselNext::new(carousel)),
                    ),
            )
    }
}

impl Render for ComponentGallery {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        div()
            .id("component-gallery")
            .size_full()
            .min_w_0()
            .min_h_0()
            .overflow_y_scroll()
            .bg(theme.secondary)
            .p_5()
            .flex()
            .flex_col()
            .gap_8()
            .children(
                self.sections
                    .iter()
                    .map(|section| Self::render_section(section, cx)),
            )
    }
}

// -----------------------------------------------------------------------------
// Demo application
// -----------------------------------------------------------------------------

pub struct DemoApp {
    gallery: Entity<ComponentGallery>,
}

impl DemoApp {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let generation_items = vec![
            GalleryItem::new(
                "Default",
                "Ready to generate firmware",
                cx.new(|cx| Generation::new(window, cx)).into(),
            ),
            GalleryItem::new(
                "In progress",
                "Live generation progress",
                cx.new(|cx| GenerationProgress::new(window, cx)).into(),
            ),
            GalleryItem::new(
                "Success",
                "Generation completed",
                cx.new(|cx| GenerationSuccess::new(window, cx)).into(),
            ),
            GalleryItem::new(
                "Failure",
                "Error and recovery state",
                cx.new(|cx| GenerationFailed::new(window, cx)).into(),
            ),
        ];

        let sections = vec![GallerySection::new(
            "generation",
            "Generation",
            "From a natural-language prompt to generated firmware.",
            generation_items,
            cx,
        )];

        let gallery = cx.new(|_| ComponentGallery::new(sections));

        Self { gallery }
    }
}

impl Render for DemoApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .min_w_0()
            .min_h_0()
            .child(self.gallery.clone())
    }
}
