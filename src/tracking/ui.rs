use super::{settings::Color, BuffState, Entry, Sorting, Tracker};
use crate::{
    buff_ui,
    defs::{BuffDef, Definitions, DIMINISHED, MALNOURISHED},
};
use arc_util::{
    api::CoreColor,
    exports,
    ui::{render, Component, Windowable},
};
use arcdps::imgui::{
    TabBar, TabItem, TableColumnFlags, TableColumnSetup, TableFlags, TableSortDirection, Ui,
};

impl Tracker {
    /// Renders a player entry in a table.
    fn render_table_entry(
        &self,
        ui: &Ui,
        defs: &Definitions,
        entry_id: usize,
        entry: &Entry,
        colors: &exports::Colors,
        sub: bool,
    ) {
        let player = &entry.player;
        let sub_color = colors.sub_base(player.subgroup);
        let prof_color = colors.prof_base(player.profession);
        let red = colors
            .core(CoreColor::LightRed)
            .unwrap_or([1.0, 0.0, 0.0, 1.0]);
        let green = colors
            .core(CoreColor::LightGreen)
            .unwrap_or([0.0, 1.0, 0.0, 1.0]);
        let yellow = colors
            .core(CoreColor::LightYellow)
            .unwrap_or([1.0, 1.0, 0.0, 1.0]);

        // new row for each player
        ui.table_next_row();

        // render subgroup cell
        if sub {
            ui.table_next_column();
            let sub = format!("{:>2}", player.subgroup);
            match (self.settings.color_sub, sub_color, prof_color) {
                (Color::Sub, Some(color), _) => ui.text_colored(color, sub),
                (Color::Prof, _, Some(color)) => ui.text_colored(color, sub),
                _ => ui.text(sub),
            }
        }

        // render name cell
        ui.table_next_column();
        match (self.settings.color_name, sub_color, prof_color) {
            (Color::Sub, Some(color), _) => ui.text_colored(color, &player.character),
            (Color::Prof, _, Some(color)) => ui.text_colored(color, &player.character),
            _ => ui.text(&player.character),
        }
        if ui.is_item_hovered() {
            ui.tooltip_text(&player.account);
        }

        // render food cell
        ui.table_next_column();
        match entry.food.state {
            BuffState::Unknown => {
                ui.text("???");
                if ui.is_item_hovered() {
                    ui.tooltip_text("Uncertain");
                }
            }
            BuffState::None => {
                ui.text_colored(red, "NONE");
                if ui.is_item_hovered() {
                    ui.tooltip_text("No Food");
                }
            }
            BuffState::Some(id) => {
                if let Some(BuffDef::Food(food)) = defs.get_buff(id) {
                    let color = match food.id {
                        MALNOURISHED => red,
                        _ => green,
                    };
                    ui.text_colored(color, &food.display);
                    buff_ui::render_buff_tooltip(ui, food);
                    buff_ui::render_food_context_menu(ui, entry_id, food.id, Some(&food.name));
                } else {
                    ui.text_colored(yellow, "SOME");
                    if ui.is_item_hovered() {
                        ui.tooltip_text("Unknown Food");
                    }
                    buff_ui::render_food_context_menu(ui, entry_id, id, None);
                }
            }
        }

        // render util cell
        ui.table_next_column();
        match entry.util.state {
            BuffState::Unknown => {
                ui.text("???");
                if ui.is_item_hovered() {
                    ui.tooltip_text("Uncertain");
                }
            }
            BuffState::None => {
                ui.text_colored(red, "NONE");
                if ui.is_item_hovered() {
                    ui.tooltip_text("No Utility");
                }
            }
            BuffState::Some(id) => {
                if let Some(BuffDef::Util(util)) = defs.get_buff(id) {
                    let color = match util.id {
                        DIMINISHED => red,
                        _ => green,
                    };
                    ui.text_colored(color, &util.display);
                    buff_ui::render_buff_tooltip(ui, util);
                    buff_ui::render_util_context_menu(ui, entry_id, util.id, Some(&util.name));
                } else {
                    ui.text_colored(yellow, "SOME");
                    if ui.is_item_hovered() {
                        ui.tooltip_text("Unknown Utility");
                    }
                    buff_ui::render_util_context_menu(ui, entry_id, id, None);
                }
            }
        }
    }

    /// Renders the tracker tab for the squad.
    fn render_squad_tab(&mut self, ui: &Ui, defs: &Definitions) {
        if self.players.is_empty() {
            ui.text("No players in range");
        } else {
            let col_sub = TableColumnSetup {
                name: "Sub",
                user_id: 0.into(),
                flags: TableColumnFlags::PREFER_SORT_DESCENDING | TableColumnFlags::DEFAULT_SORT,
                init_width_or_weight: 0.0,
            };

            let col_player = TableColumnSetup {
                name: "Player",
                user_id: 1.into(),
                flags: TableColumnFlags::PREFER_SORT_DESCENDING,
                init_width_or_weight: 0.0,
            };

            let col_food = TableColumnSetup {
                name: "Food",
                user_id: 2.into(),
                flags: TableColumnFlags::PREFER_SORT_DESCENDING,
                init_width_or_weight: 0.0,
            };

            let col_util = TableColumnSetup {
                name: "Util",
                user_id: 3.into(),
                flags: TableColumnFlags::PREFER_SORT_DESCENDING,
                init_width_or_weight: 0.0,
            };

            const TABLE_ID: &str = "##squad-table";
            let table_flags =
                TableFlags::SIZING_STRETCH_PROP | TableFlags::PAD_OUTER_X | TableFlags::SORTABLE;

            if let Some(_table) = if self.settings.show_sub {
                ui.begin_table_header_with_flags(
                    TABLE_ID,
                    [col_sub, col_player, col_food, col_util],
                    table_flags,
                )
            } else {
                ui.begin_table_header_with_flags(
                    TABLE_ID,
                    [col_player, col_food, col_util],
                    table_flags,
                )
            } {
                // update sorting if necessary
                if let Some(sort_specs) = ui.table_sort_specs_mut() {
                    sort_specs.conditional_sort(|column_specs| {
                        if let Some(sorted_column) = column_specs
                            .iter()
                            .find(|column| column.sort_direction().is_some())
                        {
                            // update sorting state
                            match sorted_column.column_user_id() {
                                0 => self.sorting = Sorting::Sub,
                                1 => self.sorting = Sorting::Name,
                                2 => self.sorting = Sorting::Food,
                                3 => self.sorting = Sorting::Util,
                                _ => {}
                            }

                            // ascending is reverse order for us
                            self.reverse = sorted_column.sort_direction().unwrap()
                                == TableSortDirection::Ascending;

                            // refresh sorting
                            self.refresh_sort();
                        }
                    });
                }

                // render table content
                let colors = exports::colors();
                for entry in &self.players {
                    self.render_table_entry(
                        ui,
                        defs,
                        entry.player.id,
                        entry,
                        &colors,
                        self.settings.show_sub,
                    );
                }
            }
        }
    }

    /// Renders the tracker tab for own characters.
    fn render_self_tab(&mut self, ui: &Ui, defs: &Definitions) {
        let current = self.get_self();
        if current.is_none() && self.chars_cache.is_empty() {
            ui.text("No characters found");
        } else if let Some(_table) = ui.begin_table_header_with_flags(
            "##self-table",
            [
                TableColumnSetup::new("Player"),
                TableColumnSetup::new("Food"),
                TableColumnSetup::new("Util"),
            ],
            TableFlags::SIZING_STRETCH_PROP | TableFlags::PAD_OUTER_X,
        ) {
            // render table content
            let colors = exports::colors();
            if let Some(entry) = current {
                self.render_table_entry(ui, defs, usize::MAX, entry, &colors, false);
            }
            for (i, entry) in self.chars_cache.iter().enumerate() {
                self.render_table_entry(ui, defs, i, entry, &colors, false);
            }
        }
    }
}

impl Component for Tracker {
    type Props = Definitions;

    fn render(&mut self, ui: &Ui, defs: &Self::Props) {
        TabBar::new("##tabs").build(ui, || {
            TabItem::new("Squad").build(ui, || {
                self.render_squad_tab(ui, defs);
            });
            TabItem::new("Characters").build(ui, || {
                self.render_self_tab(ui, defs);
            });
        });
    }
}

impl Windowable for Tracker {
    const CONTEXT_MENU: bool = true;

    fn render_menu(&mut self, ui: &Ui, _defs: &Self::Props) {
        let colors = exports::colors();
        let grey = colors
            .core(CoreColor::MediumGrey)
            .unwrap_or([0.5, 0.5, 0.5, 1.0]);

        // hotkey
        render::input_key(ui, "##hotkey", "Hotkey", &mut self.settings.hotkey);

        ui.spacing();

        // display options
        ui.menu("Display", || {
            ui.text_colored(grey, "Display");

            ui.checkbox("Show subgroup", &mut self.settings.show_sub);

            const COLORS: &[Color] = &[Color::None, Color::Sub, Color::Prof];
            let input_width = render::ch_width(ui, 16);

            let mut sub_index = COLORS
                .iter()
                .position(|entry| *entry == self.settings.color_sub)
                .unwrap();

            ui.set_next_item_width(input_width);
            if ui.combo("Subgroup color", &mut sub_index, COLORS, |entry| {
                entry.to_string().into()
            }) {
                self.settings.color_sub = COLORS[sub_index];
            }

            let mut name_index = COLORS
                .iter()
                .position(|entry| *entry == self.settings.color_name)
                .unwrap();

            ui.set_next_item_width(input_width);
            if ui.combo("Name color", &mut name_index, COLORS, |entry| {
                entry.to_string().into()
            }) {
                self.settings.color_name = COLORS[name_index];
            }
        });
    }
}
