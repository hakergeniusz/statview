// StatView's brain: pure standard library, no game types, so it compiles and
// tests wherever. This is the view model of a rebuilt Statistics screen —
// a search box that actually filters, columns that sort, and the formatting
// the vanilla screen shows — while the injected helpers feed it the game's
// statistics and draw what it answers.
//
// Clean-room: the feature set follows BetterStats Screen (search/filter/sort
// over the statistics list, a visual item grid); no code, asset or text of
// that mod is used — the statistics themselves, their names and the number
// formats are the vanilla game's, which Horizon already implements.
//
// Copyright (c) 2026 hakergeniusz. MIT.
/// The mod's registry id: the settings folder and the Options > Mods row.
pub const ID: &str = "statview";
/// The version the row shows.
pub const VERSION: &str = "1.0.0";
/// The mod's description for its row.
pub const DESCRIPTION: &str = "A rebuilt Statistics screen: search that filters, \
columns that sort, and an item grid with counts at a glance. Clean-room \
take on BetterStats Screen's core ideas.";

/// The toggle's setting file (one file per key under the mod's data dir),
/// read fresh from disk every time a gate runs.
pub const SETTING: &str = "enabled";

/// Whether the toggle is on. A missing file means ON — installing the mod is
/// asking for the rebuilt screen; only an explicit `false` turns it off.
pub fn setting_on(get: impl FnOnce(&str) -> Option<String>) -> bool {
    !matches!(get(SETTING).as_deref(), Some(v) if v.trim() == "false")
}

/// The item columns, in the vanilla screen's order: the stat type each one
/// reads (`minecraft:<ty>` under the hood) and the badge it shows.
pub const ITEM_COLUMNS: [&str; 6] = ["mined", "broken", "crafted", "used", "picked_up", "dropped"];

/// The General tab's rows, in the vanilla registration order, with the
/// formatter each one reads.
pub const CUSTOM: &[(&str, Format)] = &[
    ("leave_game", Format::Default),
    ("play_time", Format::Time),
    ("total_world_time", Format::Time),
    ("time_since_death", Format::Time),
    ("time_since_rest", Format::Time),
    ("sneak_time", Format::Time),
    ("walk_one_cm", Format::Distance),
    ("crouch_one_cm", Format::Distance),
    ("sprint_one_cm", Format::Distance),
    ("walk_on_water_one_cm", Format::Distance),
    ("fall_one_cm", Format::Distance),
    ("climb_one_cm", Format::Distance),
    ("fly_one_cm", Format::Distance),
    ("walk_under_water_one_cm", Format::Distance),
    ("minecart_one_cm", Format::Distance),
    ("boat_one_cm", Format::Distance),
    ("pig_one_cm", Format::Distance),
    ("happy_ghast_one_cm", Format::Distance),
    ("horse_one_cm", Format::Distance),
    ("aviate_one_cm", Format::Distance),
    ("swim_one_cm", Format::Distance),
    ("strider_one_cm", Format::Distance),
    ("nautilus_one_cm", Format::Distance),
    ("jump", Format::Default),
    ("drop", Format::Default),
    ("damage_dealt", Format::Tenths),
    ("damage_dealt_absorbed", Format::Tenths),
    ("damage_dealt_resisted", Format::Tenths),
    ("damage_taken", Format::Tenths),
    ("damage_blocked_by_shield", Format::Tenths),
    ("damage_absorbed", Format::Tenths),
    ("damage_resisted", Format::Tenths),
    ("deaths", Format::Default),
    ("mob_kills", Format::Default),
    ("animals_bred", Format::Default),
    ("player_kills", Format::Default),
    ("fish_caught", Format::Default),
    ("talked_to_villager", Format::Default),
    ("traded_with_villager", Format::Default),
    ("eat_cake_slice", Format::Default),
    ("fill_cauldron", Format::Default),
    ("use_cauldron", Format::Default),
    ("clean_armor", Format::Default),
    ("clean_banner", Format::Default),
    ("clean_shulker_box", Format::Default),
    ("interact_with_brewingstand", Format::Default),
    ("interact_with_beacon", Format::Default),
    ("inspect_dropper", Format::Default),
    ("inspect_hopper", Format::Default),
    ("inspect_dispenser", Format::Default),
    ("play_noteblock", Format::Default),
    ("tune_noteblock", Format::Default),
    ("pot_flower", Format::Default),
    ("trigger_trapped_chest", Format::Default),
    ("open_enderchest", Format::Default),
    ("enchant_item", Format::Default),
    ("play_record", Format::Default),
    ("interact_with_furnace", Format::Default),
    ("interact_with_crafting_table", Format::Default),
    ("open_chest", Format::Default),
    ("sleep_in_bed", Format::Default),
    ("sleep_in_straw_bed", Format::Default),
    ("open_shulker_box", Format::Default),
    ("open_barrel", Format::Default),
    ("interact_with_blast_furnace", Format::Default),
    ("interact_with_smoker", Format::Default),
    ("interact_with_lectern", Format::Default),
    ("interact_with_campfire", Format::Default),
    ("interact_with_cartography_table", Format::Default),
    ("interact_with_loom", Format::Default),
    ("interact_with_stonecutter", Format::Default),
    ("bell_ring", Format::Default),
    ("raid_trigger", Format::Default),
    ("raid_win", Format::Default),
    ("interact_with_anvil", Format::Default),
    ("interact_with_grindstone", Format::Default),
    ("target_hit", Format::Default),
    ("interact_with_smithing_table", Format::Default),
];

/// The number formats the vanilla screen shows (`StatFormatter`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    Default,
    Tenths,
    Distance,
    Time,
}

fn two(v: f64) -> String {
    format!("{v:.2}")
}

/// The value as the screen prints it: thousands separators, tenths of a
/// heart, centimetres rolled into metres and kilometres, ticks into
/// minutes, hours, days and years.
pub fn format(f: Format, v: i32) -> String {
    match f {
        Format::Default => {
            let digits = v.unsigned_abs().to_string();
            let mut out = String::new();
            for (i, c) in digits.chars().enumerate() {
                if i > 0 && (digits.len() - i).is_multiple_of(3) {
                    out.push(',');
                }
                out.push(c);
            }
            if v < 0 {
                out.insert(0, '-');
            }
            out
        }
        Format::Tenths => two(f64::from(v) * 0.1),
        Format::Distance => {
            let m = f64::from(v) / 100.0;
            let km = m / 1000.0;
            if km > 0.5 {
                format!("{} km", two(km))
            } else if m > 0.5 {
                format!("{} m", two(m))
            } else {
                format!("{v} cm")
            }
        }
        Format::Time => {
            let s = f64::from(v) / 20.0;
            let (min, h) = (s / 60.0, s / 3600.0);
            let (d, y) = (h / 24.0, h / 24.0 / 365.0);
            if y > 0.5 {
                format!("{} y", two(y))
            } else if d > 0.5 {
                format!("{} d", two(d))
            } else if h > 0.5 {
                format!("{} h", two(h))
            } else if min > 0.5 {
                format!("{} min", two(min))
            } else {
                format!("{s:.1} s")
            }
        }
    }
}

/// Whether an entry answers the search: every whitespace-separated word must
/// appear in the entry's id or its display name, case-insensitively — so
/// `dia pic` finds deepslate diamond ore by its name and `diamond_ore` by
/// its id.
pub fn matches(search: &str, id: &str, display: &str) -> bool {
    let hay = format!("{id}\n{display}").to_lowercase();
    search
        .split_whitespace()
        .all(|word| hay.contains(&word.to_lowercase()))
}

/// The view model of one open Statistics screen.
#[derive(Debug, Default)]
pub struct View {
    /// 0 General, 1 Items, 2 Mobs.
    pub tab: usize,
    /// The search box's text and caret (a char offset).
    pub search: String,
    pub caret: usize,
    /// Whether the search box holds the keyboard.
    pub focused: bool,
    /// The Items tab's sort column (an [`ITEM_COLUMNS`] index) and direction.
    pub sort_col: usize,
    pub sort_asc: bool,
    /// The list's scroll offset, in pixels.
    pub scroll: f32,
    /// The Items tab's open slot, by item id, if one is picked.
    pub selected: Option<String>,
    /// The list region and content height the last frame drew —
    /// `(top, bottom, content)` — for the wheel and the scrollbar between
    /// frames. Set by the draw, read by the wheel.
    pub layout: Option<(f32, f32, f32)>,
    /// The item ids the last frame drew, in grid order — the click finds a
    /// slot's item by its index here. Set by the draw.
    pub drawn: Vec<String>,
}

impl View {
    pub fn new() -> Self {
        Self::default()
    }

    /// Everything forgotten: the screen opened again.
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// The item a grid slot holds, by its drawn index; none past the end.
    pub fn drawn_at(&self, i: usize) -> Option<&str> {
        self.drawn.get(i).map(String::as_str)
    }

    /// Typed text lands at the caret.
    pub fn insert(&mut self, text: &str) {
        let at = self
            .search
            .char_indices()
            .nth(self.caret)
            .map(|(i, _)| i)
            .unwrap_or(self.search.len());
        self.search.insert_str(at, text);
        self.caret += text.chars().count();
    }

    /// The key before the caret goes away; nothing when there is none.
    pub fn backspace(&mut self) {
        if self.caret == 0 {
            return;
        }
        let start = self
            .search
            .char_indices()
            .nth(self.caret - 1)
            .map(|(i, _)| i)
            .unwrap_or(0);
        let at = self
            .search
            .char_indices()
            .nth(self.caret)
            .map(|(i, _)| i)
            .unwrap_or(self.search.len());
        self.search.replace_range(start..at, "");
        self.caret -= 1;
    }

    /// The key at the caret goes away; nothing when there is none.
    pub fn delete(&mut self) {
        let at = self
            .search
            .char_indices()
            .nth(self.caret)
            .map(|(i, _)| i)
            .unwrap_or(self.search.len());
        if at >= self.search.len() {
            return;
        }
        let end = self
            .search
            .char_indices()
            .nth(self.caret + 1)
            .map(|(i, _)| i)
            .unwrap_or(self.search.len());
        self.search.replace_range(at..end, "");
    }

    /// The caret one character to the left, clamped at the start.
    pub fn left(&mut self) {
        self.caret = self.caret.saturating_sub(1);
    }

    /// The caret one character to the right, clamped at the end.
    pub fn right(&mut self) {
        self.caret = (self.caret + 1).min(self.search.chars().count());
    }

    pub fn home(&mut self) {
        self.caret = 0;
    }

    pub fn end(&mut self) {
        self.caret = self.search.chars().count();
    }

    /// The tab a digit (1..3) picks, or the next one Tab cycles to.
    pub fn tab_key(&mut self, digit: Option<usize>, cycle: isize) -> bool {
        if let Some(d) = digit {
            let t = d.saturating_sub(1).min(2);
            if t != self.tab {
                self.tab = t;
                self.scroll = 0.0;
                self.selected = None;
            }
            return true;
        }
        let t = (self.tab as isize + cycle).rem_euclid(3) as usize;
        if t != self.tab {
            self.tab = t;
            self.scroll = 0.0;
            self.selected = None;
        }
        true
    }

    /// The Items tab's header clicked: the same column flips its direction,
    /// a new one sorts descending by it (the biggest counts first, as the
    /// vanilla screen starts).
    pub fn sort_clicked(&mut self, col: usize) {
        if self.sort_col == col {
            self.sort_asc = !self.sort_asc;
        } else {
            self.sort_col = col.min(ITEM_COLUMNS.len() - 1);
            self.sort_asc = false;
        }
        self.scroll = 0.0;
    }

    /// The rows the screen shows: the entries whose id or display name
    /// answers the search, sorted by the Items tab's column (values by the
    /// `value` closure, ties by id) or by name otherwise. `values(col, id)`
    /// reads a column's count; the caller supplies it so the model stays
    /// free of game types.
    pub fn rows(
        &self,
        entries: Vec<(String, String)>,
        value: impl Fn(usize, &str) -> i32,
    ) -> Vec<(String, String)> {
        let mut rows: Vec<(String, String)> = entries
            .into_iter()
            .filter(|(id, display)| matches(&self.search, id, display))
            .collect();
        if self.tab == 1 {
            let (col, asc) = (self.sort_col, self.sort_asc);
            rows.sort_by(|a, b| {
                let (va, vb) = (value(col, &a.0), value(col, &b.0));
                va.cmp(&vb)
                    .then_with(|| a.0.cmp(&b.0))
                    .then_with(|| a.1.cmp(&b.1))
            });
            if !asc {
                rows.reverse();
            }
        } else {
            rows.sort_by(|a, b| {
                a.1.to_lowercase()
                    .cmp(&b.1.to_lowercase())
                    .then_with(|| a.0.cmp(&b.0))
            });
        }
        rows
    }

    /// The scroll offset for a wheel tick, clamped to the content.
    pub fn scrolled(&mut self, amount: f32, top: f32, bottom: f32, content: f32) -> f32 {
        self.scroll = (self.scroll - amount * 20.0).clamp(0.0, (content - (bottom - top)).max(0.0));
        self.scroll
    }

    /// The scroll offset clamped after the content shrank or grew.
    pub fn clamp(&mut self, top: f32, bottom: f32, content: f32) -> f32 {
        self.scroll = self.scroll.clamp(0.0, (content - (bottom - top)).max(0.0));
        self.scroll
    }
}

// The model, one per thread (the window's Game drives it).
thread_local! {
    static VIEW: std::cell::RefCell<View> = std::cell::RefCell::new(View::new());
}

/// Run `f` with this thread's view. The helpers call it for every event;
/// the calls never nest.
pub fn with_view<R>(f: impl FnOnce(&mut View) -> R) -> R {
    VIEW.with_borrow_mut(f)
}

/// The settings page's single row.
pub fn label(on: bool) -> String {
    format!("Enabled: {}", if on { "ON" } else { "OFF" })
}

/// A click on the settings page's row `i`: the first row toggles the mod,
/// the rest are nothing. Returns the new value when the click was the
/// toggle's, so the caller stores only real clicks.
pub fn click(i: usize, on: bool) -> Option<bool> {
    (i == 0).then_some(!on)
}

/// What to store for a value: the one setting key and its file text.
pub fn stored(on: bool) -> (&'static str, String) {
    (SETTING, if on { "true" } else { "false" }.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setting(value: &str) -> impl Fn(&str) -> Option<String> + '_ {
        move |key| (key == SETTING).then(|| value.to_owned())
    }

    /// Installed and never touched means on; only an explicit `false` turns
    /// the rebuilt screen off, and what the click writes reads back.
    #[test]
    fn installed_means_on_until_declined() {
        assert!(setting_on(|_| None), "the default must be on");
        assert!(setting_on(setting(" true \n")));
        assert!(!setting_on(setting("false")));
        for on in [true, false] {
            let (key, value) = stored(on);
            assert_eq!(key, SETTING);
            assert_eq!(setting_on(setting(&value)), on, "{value} reads back {on}");
        }
        assert_eq!(label(true), "Enabled: ON");
        assert_eq!(click(0, false), Some(true));
        assert_eq!(click(1, false), None, "the page has the one row");
    }

    /// The formats the vanilla screen prints: separators, tenths, distances
    /// rolled up, times rolled up.
    #[test]
    fn the_formats_print_as_the_screen_prints() {
        assert_eq!(format(Format::Default, 1234), "1,234");
        assert_eq!(format(Format::Default, -9876), "-9,876");
        assert_eq!(format(Format::Tenths, 35), "3.50");
        assert_eq!(format(Format::Distance, 40), "40 cm");
        assert_eq!(format(Format::Distance, 150), "1.50 m");
        assert_eq!(format(Format::Distance, 1_500_000), "15.00 km");
        assert_eq!(format(Format::Time, 30), "1.5 s");
        assert_eq!(format(Format::Time, 3_600), "3.00 min");
        assert_eq!(format(Format::Time, 72_000), "1.00 h");
    }

    /// The search: every word must hit the id or the display name, case
    /// does not matter, and an empty search answers everything.
    #[test]
    fn the_search_filters_by_id_and_name() {
        assert!(matches("", "minecraft:diamond_ore", "Diamond Ore"));
        assert!(matches("diamond", "minecraft:diamond_ore", "Diamond Ore"));
        assert!(matches(
            "dia ore",
            "minecraft:deepslate_diamond_ore",
            "Deepslate Diamond Ore"
        ));
        assert!(matches("DIAMOND", "minecraft:diamond_ore", "Diamond Ore"));
        assert!(!matches("gold", "minecraft:diamond_ore", "Diamond Ore"));
        assert!(!matches(
            "diamond gold",
            "minecraft:diamond_ore",
            "Diamond Ore"
        ));
    }

    /// The caret edits: insert at the caret, backspace and delete around it,
    /// arrows and Home/End clamp, multi-byte characters stay whole.
    #[test]
    fn the_caret_edits_keep_characters_whole() {
        let mut v = View::new();
        v.insert("dia");
        v.insert("mond");
        assert_eq!(v.search, "diamond");
        assert_eq!(v.caret, 7);
        for _ in 0..4 {
            v.left();
        }
        v.insert("_");
        assert_eq!(v.search, "dia_mond");
        assert_eq!(v.caret, 4);
        v.backspace();
        assert_eq!(v.search, "diamond");
        v.home();
        v.backspace();
        assert_eq!(v.search, "diamond", "nothing before the start");
        v.end();
        v.delete();
        assert_eq!(v.search, "diamond", "nothing after the end");
        let mut v = View::new();
        v.insert("éé");
        assert_eq!(v.caret, 2);
        v.left();
        v.backspace();
        assert_eq!(v.search, "é");
        assert_eq!(v.caret, 0);
    }

    /// The rows: the search filters, the Items tab sorts by its column
    /// (descending first, ties by id), the other tabs sort by name.
    #[test]
    fn the_rows_filter_and_sort() {
        let entries = vec![
            ("minecraft:apple".to_owned(), "Apple".to_owned()),
            ("minecraft:diamond_ore".to_owned(), "Diamond Ore".to_owned()),
            ("minecraft:stone".to_owned(), "Stone".to_owned()),
        ];
        let value = |col: usize, id: &str| match (col, id) {
            (_, "minecraft:apple") => 3,
            (_, "minecraft:diamond_ore") => 64,
            _ => 0,
        };
        let mut v = View::new();
        v.tab = 1;
        assert_eq!(v.rows(entries.clone(), value)[0].0, "minecraft:diamond_ore");
        v.sort_asc = true;
        assert_eq!(v.rows(entries.clone(), value)[0].0, "minecraft:stone");
        v.search = "dia".to_owned();
        let got = v.rows(entries.clone(), value);
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].0, "minecraft:diamond_ore");
        v.tab = 2;
        v.search.clear();
        let got = v.rows(entries, value);
        assert_eq!(got[0].1, "Apple", "no column sort off the Items tab");
    }

    /// The sort header: a new column starts descending, the same column
    /// flips, the scroll restarts.
    #[test]
    fn the_sort_header_flips_and_resets() {
        let mut v = View::new();
        v.tab = 1;
        v.sort_col = 2;
        v.sort_asc = true;
        v.scroll = 90.0;
        v.sort_clicked(2);
        assert!(!v.sort_asc);
        assert_eq!(v.scroll, 0.0);
        v.sort_clicked(4);
        assert_eq!(v.sort_col, 4);
        assert!(!v.sort_asc, "a new column starts descending");
    }

    /// The scroll clamps to the content: never past the end, never above
    /// zero, and a shrinking list pulls the offset back.
    #[test]
    fn the_scroll_clamps_to_the_content() {
        let mut v = View::new();
        assert_eq!(
            v.scrolled(2.0, 24.0, 300.0, 50.0),
            0.0,
            "content fits, no scroll"
        );
        assert_eq!(v.scrolled(-2.0, 24.0, 300.0, 900.0), 40.0);
        v.scroll = 900.0;
        assert_eq!(v.clamp(24.0, 300.0, 500.0), 224.0);
    }
}
