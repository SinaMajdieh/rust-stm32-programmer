use gpui_kit::{
    AnyView, AppContext, Context, Entity, IntoElement, ParentElement, Pixels, Render, Styled,
    Window, div, px, rgb,
};

use crate::editor::{Generation, GenerationFailed, GenerationProgress, GenerationSuccess};

// --------------------------------------------------
// Responsive grid entity
// --------------------------------------------------

struct ResponsiveGrid {
    cells: Vec<AnyView>,
    breakpoint: Pixels,
}

#[allow(dead_code)]
impl ResponsiveGrid {
    fn new(cells: Vec<AnyView>) -> Self {
        Self {
            cells,
            breakpoint: px(600.0),
        }
    }

    fn set_cells(&mut self, cells: Vec<AnyView>, cx: &mut Context<Self>) {
        self.cells = cells;
        cx.notify();
    }

    fn set_breakpoint(&mut self, breakpoint: Pixels) {
        self.breakpoint = breakpoint;
    }
}

impl Render for ResponsiveGrid {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let width = window.bounds().size.width;
        let columns = if width < self.breakpoint { 1 } else { 3 };

        let rows: Vec<Vec<AnyView>> = self
            .cells
            .chunks(columns)
            .map(|cells| cells.to_vec())
            .collect();

        div()
            .size_full()
            .min_w_0()
            .min_h_0()
            .p_2()
            .flex()
            .flex_col()
            .items_stretch()
            .gap_2()
            .children(rows.into_iter().map(|row_cells| {
                div()
                    .w_full()
                    .flex_1()
                    .min_w_0()
                    .min_h_0()
                    .flex()
                    .items_stretch()
                    .gap_2()
                    .children(
                        row_cells
                            .into_iter()
                            .map(|cell| div().flex_1().min_w_0().min_h_0().h_full().child(cell)),
                    )
            }))
    }
}

// --------------------------------------------------
// Example child view — replace with your own views
// --------------------------------------------------

struct DemoCell {
    index: usize,
    color: u32,
}

impl Render for DemoCell {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .min_w_0()
            .min_h_0()
            .rounded_md()
            .bg(rgb(self.color))
            .flex()
            .items_center()
            .justify_center()
            .text_lg()
            .child(format!("Cell {}", self.index))
    }
}

// --------------------------------------------------
// Example application containing the grid
// --------------------------------------------------

pub struct DemoApp {
    grid: Entity<ResponsiveGrid>,
}

impl DemoApp {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let colors = [
            0xE06C75, 0xE5A86B, 0xE5D18C, 0x98C379, 0x56B6C2, 0x61AFEF, 0xC678DD, 0xABB2BF,
            0x7F848E,
        ];

        let mut cells: Vec<AnyView> = colors
            .into_iter()
            .enumerate()
            .map(|(index, color)| {
                cx.new(|_| DemoCell {
                    index: index + 1,
                    color,
                })
                .into()
            })
            .collect();

        cells[0] = cx.new(|cx| Generation::new(window, cx)).into();
        cells[1] = cx.new(|cx| GenerationProgress::new(window, cx)).into();
        cells[2] = cx.new(|cx| GenerationSuccess::new(window, cx)).into();
        cells[3] = cx.new(|cx| GenerationFailed::new(window, cx)).into();

        let grid = cx.new(|_| ResponsiveGrid::new(cells));

        Self { grid }
    }
}

impl Render for DemoApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .min_w_0()
            .min_h_0()
            .child(self.grid.clone())
    }
}
