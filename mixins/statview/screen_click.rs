// A click on the rebuilt screen: the Done button, the tabs, the search box,
// the Items header's sort columns and slots, and the scrollbar all answer
// here; the mod owns the screen's clicks, so nothing falls through to the
// vanilla handler while it asks.
// Copyright (c) 2026 hakergeniusz. MIT.
fn statview_screen_click(game: &mut crate::game::Game<'_>, size: [u32; 2]) -> core::ops::ControlFlow<()> {
    use crate::game::statview as statview;
    use horizon_client::ui;

    if !game.native_on(statview::ID)
        || !statview::setting_on(|key| game.native_setting(statview::ID, key))
    {
        return core::ops::ControlFlow::Continue(());
    }

    const TAB_H: f32 = 24.0;
    const FOOTER: f32 = 33.0;
    const SEARCH_H: f32 = 16.0;
    const SLOT: f32 = 20.0;
    let (w, h) = (size[0] as f32, size[1] as f32);
    let total = w.min(400.0) - 28.0;
    let tab_w = ((total / 3.0) / 2.0).round() * 2.0;
    let tab_x0 = ((w - tab_w * 3.0) / 2.0).floor();
    let left = ((w - 340.0) / 2.0).floor().max(4.0);
    let content_w = w - left - 16.0;
    let (top, bottom) = (TAB_H + 4.0, h - FOOTER);
    let list_top = top + SEARCH_H + 4.0;
    let (cx, cy) = game.cursor;

    // The Done button.
    let (bx, by) = ((w / 2.0 - 100.0).floor(), h - FOOTER + 6.0);
    if ui::hovered(game.cursor, bx, by, 200.0, 20.0) {
        game.screen = crate::game::Screen::Paused;
        return core::ops::ControlFlow::Break(());
    }

    // The tabs.
    for i in 0..3 {
        if ui::hovered(game.cursor, tab_x0 + tab_w * i as f32, 0.0, tab_w, TAB_H) {
            statview::with_view(|v| {
                v.tab_key(Some(i + 1), 0);
            });
            return core::ops::ControlFlow::Break(());
        }
    }

    // The scrollbar: a click jumps the view there.
    let bar_x = w - 12.0;
    if cx >= bar_x && cx <= bar_x + 8.0 && cy >= list_top && cy <= bottom {
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
    if ui::hovered(game.cursor, left, top, content_w, SEARCH_H) {
        let atlas = &game.gui.atlas;
        statview::with_view(|v| {
            v.focused = true;
            let mut at = v.search.chars().count();
            for (i, n) in (0..=v.search.chars().count())
                .map(|i| (i, HudBatch::text_width(atlas, &v.search.chars().take(i).collect::<String>())))
            {
                if cx - left - 5.0 < n + 3.0 {
                    at = i;
                    break;
                }
            }
            v.caret = at;
        });
        return core::ops::ControlFlow::Break(());
    }

    statview::with_view(|v| {
        v.focused = false;
        // The Items tab's header and grid.
        if v.tab == 1 {
            for (c, _) in statview::ITEM_COLUMNS.into_iter().enumerate() {
                let hx = left + 2.0 + c as f32 * 64.0;
                if ui::hovered(game.cursor, hx, list_top, 60.0, 12.0) {
                    v.sort_clicked(c);
                    return;
                }
            }
            let cols = ((content_w / SLOT).floor() as usize).max(1);
            let grid_top = list_top + 14.0;
            let offset = v.scroll.max(0.0);
            for i in 0..v.drawn.len() {
                let (gx, gy) = (i % cols, i / cols);
                let (x, y) = (left + 2.0 + gx as f32 * SLOT, grid_top + gy as f32 * SLOT - offset);
                if y >= bottom {
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
        }
    });
    core::ops::ControlFlow::Break(())
}
