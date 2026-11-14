// A click on the rebuilt screen: the Done button, the sidebar's categories,
// the search box, the Items header's sort columns and slots, and the
// scrollbar all answer here, over the same geometry the runtime holds; the
// mod owns the screen's clicks, so nothing falls through to the vanilla
// handler while it asks.
// Copyright (c) 2026 hakergeniusz. MIT.
fn statview_screen_click(game: &mut crate::game::Game<'_>, size: [u32; 2]) -> core::ops::ControlFlow<()> {
    use crate::game::statview as statview;
    use horizon_client::ui;

    if !game.native_on(statview::ID)
        || !statview::setting_on(|key| game.native_setting(statview::ID, key))
    {
        return core::ops::ControlFlow::Continue(());
    }

    const SLOT: f32 = statview::SLOT;
    let (w, h) = (size[0] as f32, size[1] as f32);
    let (cx, cy) = game.cursor;

    // The Done button.
    let [bx, by, bw, bh] = statview::done_rect(w, h);
    if ui::hovered(game.cursor, bx, by, bw, bh) {
        game.screen = crate::game::Screen::Paused;
        return core::ops::ControlFlow::Break(());
    }

    // The sidebar's categories.
    for i in 0..3 {
        let [x, y, cw, ch] = statview::cat_rect(i);
        if ui::hovered(game.cursor, x, y, cw, ch) {
            statview::with_view(|v| {
                v.tab_key(Some(i + 1), 0);
            });
            return core::ops::ControlFlow::Break(());
        }
    }

    // The scrollbar: a click jumps the view there.
    let (bar_x, top, bottom) = statview::bar_at(w, h);
    if cx >= bar_x && cx <= bar_x + statview::BAR_W && cy >= top && cy <= bottom {
        statview::with_view(|v| {
            if let Some((t, b, content)) = v.layout {
                let part = ((cy - t) / (b - t)).clamp(0.0, 1.0);
                v.scroll = (part * content - (b - t) / 2.0).clamp(0.0, (content - (b - t)).max(0.0));
            }
        });
        return core::ops::ControlFlow::Break(());
    }

    // The search box: clicking it puts the caret by the letter (the font's
    // own widths decide), clicking anywhere else lets go of it.
    let [fx, fy, fw, fh] = statview::SEARCH;
    if ui::hovered(game.cursor, fx, fy, fw, fh) {
        let atlas = &game.gui.atlas;
        statview::with_view(|v| {
            v.focused = true;
            let mut at = v.search.chars().count();
            for (i, n) in (0..=v.search.chars().count())
                .map(|i| (i, HudBatch::text_width(atlas, &v.search.chars().take(i).collect::<String>())))
            {
                if cx - fx - 5.0 < n + 3.0 {
                    at = i;
                    break;
                }
            }
            v.caret = at;
        });
        return core::ops::ControlFlow::Break(());
    }

    // The main area: the Items tab's header and grid.
    let main = statview::main_rect(w, h);
    let in_main = ui::hovered(game.cursor, main[0], main[1], main[2], main[3]);
    if !in_main {
        statview::with_view(|v| v.focused = false);
        return core::ops::ControlFlow::Break(());
    }
    let details = {
        let v = statview::with_view(|v| v.selected.clone());
        v.is_some()
    };
    statview::with_view(|v| {
        v.focused = false;
        if v.tab != 1 {
            return;
        }
        // The sort header: a column flips or takes over.
        for (c, _) in statview::ITEM_COLUMNS.into_iter().enumerate() {
            let hx = main[0] + 4.0 + c as f32 * 64.0;
            if ui::hovered(game.cursor, hx, main[1], 60.0, statview::SORT_H) {
                v.sort_clicked(c);
                return;
            }
        }
        // The grid: a slot toggles the details strip's pick.
        let grid_top = main[1] + statview::SORT_H + 4.0;
        let offset = v.scroll.max(0.0);
        let count = v.drawn.len();
        for i in 0..count {
            let (x, uy) = statview::slot_at(w, h, i, details);
            let y = uy - offset;
            if y >= main[1] + main[3] {
                break;
            }
            if y < grid_top - SLOT {
                continue;
            }
            if ui::hovered(game.cursor, x, y, SLOT, SLOT) {
                let id = v.drawn_at(i).map(str::to_owned);
                if v.selected == id {
                    v.selected = None;
                } else {
                    v.selected = id;
                }
                return;
            }
        }
    });
    core::ops::ControlFlow::Break(())
}
