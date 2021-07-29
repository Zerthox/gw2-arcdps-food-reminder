use crate::{
    settings::HasSettings,
    ui::{
        align::RightAlign,
        window::{WindowProps, Windowed},
        Component,
    },
};
use arcdps::imgui::{im_str, ChildWindow, ImString, Ui};
use chrono::Local;
use serde::{Deserialize, Serialize};

/// Time format used for debug messages.
const FORMAT: &str = "%b %d %H:%M:%S.%3f";

/// Debug log component.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DebugLog {
    /// Whether the log is active.
    active: bool,

    /// Current contents of the log.
    #[serde(skip)]
    contents: ImString,

    /// Current size of contents string.
    #[serde(skip)]
    size: usize,

    // button widths used for ui rendering
    #[serde(skip)]
    toggle_width: f32,

    #[serde(skip)]
    clear_button_width: f32,

    #[serde(skip)]
    copy_button_width: f32,
}

impl DebugLog {
    /// Creates a new debug log.
    pub fn new() -> Self {
        Self {
            active: true,
            contents: ImString::default(),
            size: 1, // imgui string has an implicit null at the end
            toggle_width: 60.0,
            clear_button_width: 60.0,
            copy_button_width: 60.0,
        }
    }

    /// Appends output to the debug log.
    pub fn log<S>(&mut self, output: S)
    where
        S: AsRef<str>,
    {
        if self.active {
            // generate line
            let now = Local::now();
            let line = format!("{}: {}\n", now.format(FORMAT), output.as_ref());

            // clear on overflow
            if let Some(new) = self.size.checked_add(line.len()) {
                self.size = new;
            } else {
                self.clear();
            }

            // append line
            self.contents.push_str(&line);
        }
    }

    /// Clears the debug log.
    pub fn clear(&mut self) {
        self.size = 1;
        self.contents.clear();
    }
}

impl Default for DebugLog {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for DebugLog {
    fn render(&mut self, ui: &Ui) {
        // time
        ui.align_text_to_frame_padding();
        ui.text(format!("Time: {}", Local::now().format(FORMAT)));

        // buttons from right to left
        let mut align = RightAlign::build();

        // clear button
        align.item(ui, self.clear_button_width, || {
            if ui.button(im_str!("Clear"), [0.0, 0.0]) {
                self.clear();
            }
            self.clear_button_width = ui.item_rect_size()[0];
        });

        // copy button
        align.item(ui, self.copy_button_width, || {
            if ui.button(im_str!("Copy"), [0.0, 0.0]) {
                ui.set_clipboard_text(&self.contents);
            }
            self.copy_button_width = ui.item_rect_size()[0];
        });

        // activity toggle
        align.item_with_margin(ui, 10.0, self.toggle_width, || {
            ui.checkbox(im_str!("Active"), &mut self.active);
            self.toggle_width = ui.item_rect_size()[0];
        });

        ui.separator();

        // log contents
        ChildWindow::new(im_str!("##food-reminder-log-scroller"))
            .scrollable(true)
            .horizontal_scrollbar(true)
            .build(ui, || {
                ui.text(&self.contents);
                ui.set_scroll_here_y_with_ratio(1.0);
            })
    }
}

impl Windowed for DebugLog {
    fn window_props() -> WindowProps {
        WindowProps::new("Food Debug Log")
            .visible(true)
            .width(600.0)
            .height(300.0)
    }
}

impl HasSettings for DebugLog {
    type Settings = ();
    fn settings_name() -> &'static str {
        "log"
    }
    fn get_settings(&self) -> Self::Settings {}
    fn load_settings(&mut self, _: Self::Settings) {}
}
