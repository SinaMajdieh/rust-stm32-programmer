// --------------------------------------------------
// Log entry
// --------------------------------------------------

use gpui_kit::{
    Context, InteractiveElement, IntoElement, ParentElement, Refineable, Render, SharedString,
    StyleRefinement, Styled, Window,
    base::{h_flex, v_flex},
    component::{ActiveTheme, scroll::ScrollableElement},
    div,
};

#[derive(Clone, Debug)]
pub struct LogEntry {
    timestamp: SharedString,
    message: SharedString,
}

impl LogEntry {
    pub fn new(timestamp: impl Into<SharedString>, message: impl Into<SharedString>) -> Self {
        Self {
            timestamp: timestamp.into(),
            message: message.into(),
        }
    }
}

// --------------------------------------------------
// Build log component
// --------------------------------------------------

pub struct Log {
    entries: Vec<LogEntry>,
    style: StyleRefinement,
    scroll_id: SharedString,
}

#[allow(dead_code)]
impl Log {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            style: StyleRefinement::default(),
            scroll_id: "build-log-scroll".into(),
        }
    }

    /// Append a log entry with an explicit timestamp.
    pub fn log(
        &mut self,
        timestamp: impl Into<SharedString>,
        message: impl Into<SharedString>,
        cx: &mut Context<Self>,
    ) {
        self.entries.push(LogEntry::new(timestamp, message));
        cx.notify();
    }

    /// Append a log entry without a timestamp.
    pub fn log_message(&mut self, message: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.log("", message, cx);
    }

    /// Append several entries at once.
    pub fn log_many(
        &mut self,
        entries: impl IntoIterator<Item = LogEntry>,
        cx: &mut Context<Self>,
    ) {
        self.entries.extend(entries);
        cx.notify();
    }

    /// Remove all log entries.
    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.entries.clear();
        cx.notify();
    }

    /// Replace all current entries.
    pub fn set_entries(
        &mut self,
        entries: impl IntoIterator<Item = LogEntry>,
        cx: &mut Context<Self>,
    ) {
        self.entries = entries.into_iter().collect();
        cx.notify();
    }

    /// Get the number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Configure a unique ID for the scroll area.
    pub fn scroll_id(mut self, id: impl Into<SharedString>) -> Self {
        self.scroll_id = id.into();
        self
    }
}

impl Default for Log {
    fn default() -> Self {
        Self::new()
    }
}

// --------------------------------------------------
// GPUI styling support
// --------------------------------------------------

impl Styled for Log {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

// --------------------------------------------------
// Rendering
// --------------------------------------------------

impl Render for Log {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        let mut container = div()
            .border_1()
            .border_color(theme.colors.border)
            .max_h_24()
            .bg(theme.colors.secondary)
            .rounded_lg();

        // Apply user-provided Styled refinements over the defaults.
        container.style().refine(&self.style);

        container.child(
            v_flex()
                .id(self.scroll_id.clone())
                .overflow_y_scrollbar()
                .w_full()
                .gap_2()
                .p_4()
                .children(self.entries.iter().map(|entry| {
                    h_flex()
                        .w_full()
                        .gap_2()
                        .justify_start()
                        .items_center()
                        .child(
                            div()
                                .flex_none()
                                .text_color(theme.colors.muted_foreground)
                                .child(entry.timestamp.clone()),
                        )
                        .child(
                            div()
                                .text_color(theme.colors.muted_foreground)
                                .child(entry.message.clone()),
                        )
                })),
        )
    }
}
