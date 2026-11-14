// The rebuilt Statistics screen, drawn whole: a left sidebar — the search
// on top, the three categories under it — and the main area beside it, the
// layout BetterStats Screen made the screen's own. Rows read uniform white
// with a hover highlight instead of vanilla's stripes; the Items tab lays a
// slot grid with count badges and, while an item is picked, a details strip
// along the right with that item's columns. Everything on it answers to the
// mod's view model; the vanilla drawing is skipped while the mod asks (the
// hook breaks here). Clicks, keys and the wheel belong to the same view and
// arrive through their own helpers, over the same geometry the runtime
// holds.
// Copyright (c) 2026 hakergeniusz. MIT.
fn statview_draw(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
) -> core::ops::ControlFlow<()> {
    use crate::game::statview as statview;
    use horizon_client::{items::ItemStack, text::Text, ui};

    if !game.native_on(statview::ID)
        || !statview::setting_on(|key| game.native_setting(statview::ID, key))
    {
        return core::ops::ControlFlow::Continue(());
    }

    horizon_client::widgets::begin_screen(|| {
        horizon_client::widgets::ScreenInfo::new(
            "stats",
            crate::game::screen_info::tr("gui.stats", &[]),
        )
    });
    let w = batch.width;
    let h = batch.height;
    let atlas = &game.gui.atlas;
    ui::menu_background(batch, atlas, 0.0, 0.0, w, h, true);

    const ROW_H: f32 = 14.0;
    // The look's palette: dark panels, one indigo accent for what is
    // selected, white text, a soft row highlight on hover.
    const PANEL: u32 = 0xF014_1418;
    const SIDEBAR: u32 = 0xF01A_1A1F;
    const ACCENT: u32 = 0xFF5B_6EE1;
    const HOVER: u32 = 0x28FF_FFFF;
    const HAIRLINE: u32 = 0xFF3C_3C44;
    const DIM: u32 = 0xFFB0_B0B8;
    const BADGE: u32 = 0xFFFF_FF55;

    // The view's own numbers first: what the search holds, and the rows each
    // tab shows once the search filtered them.
    let (tab, search, caret, focused, sort_col, sort_asc, selected) =
        statview::with_view(|v| {
            (
                v.tab,
                v.search.clone(),
                v.caret,
                v.focused,
                v.sort_col,
                v.sort_asc,
                v.selected.clone(),
            )
        });
    let item_name = |game: &crate::game::Game<'_>, id: &str| {
        let stack = ItemStack::new(id, 1);
        let names = |i: &str| game.pack.names.display_item(i);
        horizon_client::stations::hover_name(&names, &stack)
    };
    let general_rows: Vec<(String, String)> = {
        let pairs: Vec<(String, String)> = statview::CUSTOM
            .iter()
            .map(|(k, _)| {
                let label = crate::game::screen_info::tr(&format!("stat.minecraft.{k}"), &[]);
                (format!("minecraft:{k}"), label)
            })
            .collect();
        statview::with_view(|v| v.rows(pairs, |_, _| 0))
    };
    let item_rows: Vec<(String, String)> = {
        let ids = game.stat_items();
        let pairs = ids
            .into_iter()
            .map(|id| {
                let display = item_name(game, &id);
                (id, display)
            })
            .collect();
        statview::with_view(|v| {
            v.rows(pairs, |col, id| game.stats.get(statview::ITEM_COLUMNS[col], id))
        })
    };
    let mob_rows: Vec<(String, String)> = {
        let ids = game.stat_mobs();
        let pairs = ids
            .into_iter()
            .map(|m| {
                let name = Text::translate(&format!("entity.{}", m.replace(':', ".")), &[]).plain();
                (m, name)
            })
            .collect();
        statview::with_view(|v| v.rows(pairs, |_, _| 0))
    };
    // The grid's order, for the click helper to find a slot's item.
    let drawn: Vec<String> = item_rows.iter().map(|(id, _)| id.clone()).collect();
    statview::with_view(|v| v.drawn = drawn);

    let main = statview::main_rect(w, h);
    let [mx, my, mw, mh] = main;
    let details = tab == 1 && selected.is_some();
    let cols = statview::grid_cols(w, h, details);
    let grid_rows = item_rows.len().div_ceil(cols);
    let content = match tab {
        0 => ROW_H * general_rows.len() as f32 + 4.0,
        1 => statview::SORT_H + 2.0 + statview::SLOT * grid_rows as f32 + 4.0,
        _ => MOB_ROW * mob_rows.len() as f32 + 4.0,
    };
    let scroll = statview::with_view(|v| {
        v.clamp(my, my + mh, content);
        v.layout = Some((my, my + mh, content));
        v.scroll
    });
    const MOB_ROW: f32 = 36.0;

    // ===== The sidebar =====
    let side = statview::sidebar_rect(h);
    batch.fill(atlas, side[0], side[1], side[2], side[3], SIDEBAR);
    batch.fill(atlas, side[0], side[1], 1.0, side[3], HAIRLINE);
    batch.fill(
        atlas,
        side[0] + side[2] - 1.0,
        side[1],
        1.0,
        side[3],
        HAIRLINE,
    );
    // The search box, its text and its caret.
    let [fx, fy, fw, fh] = statview::SEARCH;
    batch.fill(
        atlas,
        fx,
        fy,
        fw,
        fh,
        if focused { 0xFF10_1010 } else { 0xFF18_1818 },
    );
    let edge = if focused { 0xFF5B_6EE1 } else { HAIRLINE };
    batch.fill(atlas, fx, fy, fw, 1.0, edge);
    batch.fill(atlas, fx, fy + fh - 1.0, fw, 1.0, edge);
    if search.is_empty() && !focused {
        batch.text(atlas, "Search", fx + 5.0, fy + 5.0, 0xFF80_8080, true);
    } else {
        batch.text(atlas, &search, fx + 5.0, fy + 5.0, u32::MAX, true);
        if focused {
            let before: String = search.chars().take(caret).collect();
            let cx = fx + 6.0 + HudBatch::text_width(atlas, &before);
            batch.fill(atlas, cx, fy + 3.0, 1.0, fh - 6.0, u32::MAX);
        }
    }
    // The categories, the tab's own order; the picked one carries the accent.
    const TABS: [&str; 3] = ["stat.generalButton", "stat.itemsButton", "stat.mobsButton"];
    for (i, key) in TABS.into_iter().enumerate() {
        let [x, y, cw, ch] = statview::cat_rect(i);
        let hover = ui::hovered(game.cursor, x, y, cw, ch);
        let fill = if tab == i {
            ACCENT
        } else if hover {
            HOVER
        } else {
            0x18FF_FFFF
        };
        batch.fill(atlas, x, y, cw, ch, fill);
        batch.text(
            atlas,
            &Text::translate(key, &[]).plain(),
            x + 8.0,
            y + 6.0,
            if tab == i { u32::MAX } else { DIM },
            true,
        );
    }

    // ===== The main area =====
    batch.fill(atlas, mx, my, mw, mh, PANEL);
    batch.fill(atlas, mx, my, mw, 1.0, HAIRLINE);
    let row_y = |i: f32, rh: f32| my + 4.0 + i * rh - scroll;
    let visible = |y: f32, rh: f32| y >= my && y + rh <= my + mh;
    // The hovered row, for the highlight the stripes used to fake.
    let hovered_row = |rh: f32| {
        let (cx, cy) = game.cursor;
        if cx < mx || cx >= mx + mw - statview::BAR_W || cy < my || cy >= my + mh {
            return usize::MAX;
        }
        let i = ((cy - my - 4.0 + scroll) / rh).floor();
        (i >= 0.0).then(|| i as usize).unwrap_or(usize::MAX)
    };
    match tab {
        // General: the label left, the value right, uniform white.
        0 => {
            let pick = hovered_row(ROW_H);
            for (i, (key, label)) in general_rows.iter().enumerate() {
                let y = row_y(i as f32, ROW_H);
                if !visible(y, ROW_H) {
                    continue;
                }
                if i == pick {
                    batch.fill(atlas, mx + 2.0, y, mw - statview::BAR_W - 4.0, ROW_H, HOVER);
                }
                let f = statview::CUSTOM
                    .iter()
                    .find(|(k, _)| &format!("minecraft:{k}") == key)
                    .map(|(_, f)| *f)
                    .unwrap_or(statview::Format::Default);
                let value = statview::format(f, game.stats.get("custom", key));
                batch.text(atlas, label, mx + 6.0, (y + 3.0).floor(), u32::MAX, true);
                let tw = HudBatch::text_width(atlas, &value);
                batch.text(
                    atlas,
                    &value,
                    mx + mw - statview::BAR_W - tw - 8.0,
                    (y + 3.0).floor(),
                    DIM,
                    true,
                );
            }
        }
        // Items: the sort header, the grid with count badges, and the
        // picked item's details strip along the right.
        1 => {
            let grid_w = statview::grid_w(w, h, details);
            for (c, ty) in statview::ITEM_COLUMNS.into_iter().enumerate() {
                let hx = mx + 4.0 + c as f32 * 64.0;
                if hx >= mx + grid_w {
                    break;
                }
                let label = crate::game::screen_info::tr(&format!("stat_type.minecraft.{ty}"), &[]);
                batch.text(atlas, &label, hx, my + 4.0, DIM, true);
                let marker = if sort_col == c {
                    if sort_asc { "^" } else { "v" }
                } else {
                    " "
                };
                batch.text(atlas, marker, hx + 52.0, my + 4.0, ACCENT, true);
            }
            let grid_top = my + statview::SORT_H + 4.0;
            let offset = scroll.max(0.0);
            for (i, (id, _)) in item_rows.iter().enumerate() {
                let (x, uy) = statview::slot_at(w, h, i, details);
                let y = uy - offset;
                if y < grid_top - statview::SLOT || y + statview::SLOT > my + mh {
                    continue;
                }
                // Fresh per row: the tooltip at the bottom takes the game
                // mutably, and this loop runs again after it.
                let atlas = &game.gui.atlas;
                let stack = ItemStack::new(id, 1);
                let picked = selected.as_deref() == Some(id.as_str());
                batch.sprite(atlas, "gui/sprites/container/slot", x, y);
                game.draw_item(batch, &stack, x + 2.0, y + 2.0);
                if picked {
                    let edge = ACCENT;
                    batch.fill(atlas, x, y, statview::SLOT, 1.0, edge);
                    batch.fill(atlas, x, y + statview::SLOT - 1.0, statview::SLOT, 1.0, edge);
                    batch.fill(atlas, x, y, 1.0, statview::SLOT, edge);
                    batch.fill(atlas, x + statview::SLOT - 1.0, y, 1.0, statview::SLOT, edge);
                } else if ui::hovered(game.cursor, x, y, statview::SLOT, statview::SLOT) {
                    batch.fill(atlas, x, y, statview::SLOT, statview::SLOT, HOVER);
                }
                let count = game.stats.get(statview::ITEM_COLUMNS[sort_col], id);
                if count > 0 {
                    let text = statview::format(statview::Format::Default, count);
                    let tw = HudBatch::text_width(atlas, &text);
                    batch.text(
                        atlas,
                        &text,
                        x + statview::SLOT - tw - 1.0,
                        y + statview::SLOT - 10.0,
                        BADGE,
                        true,
                    );
                }
                if ui::hovered(game.cursor, x, y, statview::SLOT, statview::SLOT) {
                    game.draw_stack_tooltip(batch, &stack, x + statview::SLOT, y + statview::SLOT);
                }
            }
            // The tooltip took the game mutably, so the atlas borrow is
            // retaken here for the details strip.
            let atlas = &game.gui.atlas;
            // The details strip: the picked item's name and its six columns,
            // one line a piece, over the accent the selection carries.
            if details {
                let id = selected.expect("details only while an item is picked");
                let [dx, dy, dw, dh] = statview::details_rect(w, h);
                batch.fill(atlas, dx, dy, dw, dh, SIDEBAR);
                batch.fill(atlas, dx, dy, 1.0, dh, ACCENT);
                let name = item_name(game, &id);
                let mut y = dy + 6.0;
                batch.text(atlas, &name, dx + 6.0, y, u32::MAX, true);
                y += ROW_H + 2.0;
                for ty in statview::ITEM_COLUMNS {
                    let label = crate::game::screen_info::tr(&format!("stat_type.minecraft.{ty}"), &[]);
                    let value = statview::format(statview::Format::Default, game.stats.get(ty, &id));
                    batch.text(atlas, &label, dx + 6.0, y, DIM, true);
                    batch.text(atlas, &value, dx + 6.0, y + 9.0, u32::MAX, true);
                    y += 21.0;
                    if y + 20.0 > dy + dh {
                        break;
                    }
                }
            }
        }
        // Mobs: the name, its kills and its killed-by, uniform white.
        _ => {
            let pick = hovered_row(MOB_ROW);
            for (i, (mob, _name)) in mob_rows.iter().enumerate() {
                let y = row_y(i as f32, MOB_ROW);
                if !visible(y, MOB_ROW) {
                    continue;
                }
                if i == pick {
                    batch.fill(atlas, mx + 2.0, y, mw - statview::BAR_W - 4.0, MOB_ROW, HOVER);
                }
                let mob_name = Text::translate(&format!("entity.{}", mob.replace(':', ".")), &[]);
                let kills = game.stats.get("killed", mob);
                let by = game.stats.get("killed_by", mob);
                let k = if kills == 0 {
                    Text::translate(
                        "stat_type.minecraft.killed.none",
                        std::slice::from_ref(&mob_name),
                    )
                } else {
                    Text::translate(
                        "stat_type.minecraft.killed",
                        &[Text::literal(kills.to_string()), mob_name.clone()],
                    )
                };
                let b = if by == 0 {
                    Text::translate(
                        "stat_type.minecraft.killed_by.none",
                        std::slice::from_ref(&mob_name),
                    )
                } else {
                    Text::translate(
                        "stat_type.minecraft.killed_by",
                        &[mob_name.clone(), Text::literal(by.to_string())],
                    )
                };
                let (kc, bc) = (
                    if kills > 0 { DIM } else { 0xFF80_8080 },
                    if by > 0 { DIM } else { 0xFF80_8080 },
                );
                batch.text_styled(atlas, &mob_name, mx + 6.0, y + 1.0, u32::MAX, true);
                batch.text_styled(atlas, &k, mx + 16.0, y + 10.0, kc, true);
                batch.text_styled(atlas, &b, mx + 16.0, y + 19.0, bc, true);
            }
        }
    }
    // Retaken for the scrollbar and the footer: the details strip's borrow
    // ended with its block.
    let atlas = &game.gui.atlas;
    // The scrollbar, down the main area's right.
    let (bar_x, top, bottom) = statview::bar_at(w, h);
    ui::scrollbar(batch, atlas, bar_x, top, bottom, content, scroll);
    // The footer's Done button.
    let [bx, by, bw, bh] = statview::done_rect(w, h);
    let hover = ui::hovered(game.cursor, bx, by, bw, bh);
    ui::action_button(
        batch,
        atlas,
        horizon_client::widgets::Action::Done,
        bx,
        by,
        bw,
        bh,
        &Text::translate("gui.done", &[]),
        true,
        hover,
    );
    core::ops::ControlFlow::Break(())
}
