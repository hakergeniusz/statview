// The rebuilt Statistics screen, drawn whole: the vanilla screen's chrome —
// menu background, three tabs, Done button — over a search box that filters,
// Items as a grid of slots with their count as a badge, Mobs and General as
// rows. Everything on it answers to this mod's view model; the vanilla
// drawing is skipped while the mod asks (the hook breaks here). Clicks, keys
// and the wheel belong to the same view and arrive through their own
// helpers. The atlas is borrowed per call: the grid draws items and tooltips
// through `&mut game` in its middle.
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
    ui::menu_background(batch, &game.gui.atlas, 0.0, 0.0, w, h, true);

    // The screen's bones: the vanilla tab bar's geometry, a search row under
    // it, the list between, the Done button's footer.
    const TAB_H: f32 = 24.0;
    const FOOTER: f32 = 33.0;
    const SEARCH_H: f32 = 16.0;
    const ROW_H: f32 = 14.0;
    const SLOT: f32 = 20.0;
    const MOB_ROW: f32 = 36.0;
    let total = w.min(400.0) - 28.0;
    let tab_w = ((total / 3.0) / 2.0).round() * 2.0;
    let tab_x0 = ((w - tab_w * 3.0) / 2.0).floor();
    let left = ((w - 340.0) / 2.0).floor().max(4.0);
    let content_w = w - left - 16.0;
    let (top, bottom) = (TAB_H + 4.0, h - FOOTER);
    let list_top = top + SEARCH_H + 4.0;
    ui::separators(batch, &game.gui.atlas, TAB_H, bottom, true);

    // The view's own numbers first: what the search holds, and the rows each
    // tab shows once the search filtered them. The atlas is borrowed per
    // call everywhere below — the grid draws items and tooltips through
    // `&mut game` in its middle.
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
        // The same search filter the other tabs answer — 1.0.2 drew this tab
        // raw, so its rows ignored the query.
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

    // The list's height, by the tab.
    let cols = ((content_w / SLOT).floor() as usize).max(1);
    let grid_rows = item_rows.len().div_ceil(cols);
    let content = match tab {
        0 => ROW_H * general_rows.len() as f32 + 4.0,
        1 => SLOT * grid_rows as f32 + 4.0 + 14.0,
        _ => MOB_ROW * mob_rows.len() as f32 + 4.0,
    };
    let scroll = statview::with_view(|v| {
        v.clamp(list_top, bottom, content);
        v.layout = Some((list_top, bottom, content));
        v.scroll
    });

    // The tab bar, as the vanilla screen draws it.
    const TABS: [&str; 3] = ["stat.generalButton", "stat.itemsButton", "stat.mobsButton"];
    for (i, key) in TABS.into_iter().enumerate() {
        let x = tab_x0 + tab_w * i as f32;
        let selected = tab == i;
        let hover = ui::hovered(game.cursor, x, 0.0, tab_w, TAB_H);
        let sprite = match (selected, hover) {
            (true, true) => "gui/sprites/widget/tab_selected_highlighted",
            (true, false) => "gui/sprites/widget/tab_selected",
            (false, true) => "gui/sprites/widget/tab_highlighted",
            (false, false) => "gui/sprites/widget/tab",
        };
        ui::nine_slice(batch, &game.gui.atlas, sprite, x, 0.0, tab_w, TAB_H, u32::MAX);
        ui::menu_tab_label(
            batch,
            &game.gui.atlas,
            &Text::translate(key, &[]),
            (x, 0.0, tab_w, TAB_H),
            selected,
            true,
            horizon_client::hud_overlays::now_ms(),
        );
        if selected {
            batch.fill(&game.gui.atlas, x + 2.0, TAB_H - 3.0, tab_w - 4.0, 1.0, u32::MAX);
        }
    }

    // The search box, its text and its caret.
    let (bx, bw) = (left, content_w);
    batch.fill(
        &game.gui.atlas,
        bx,
        top,
        bw,
        SEARCH_H,
        if focused { 0xFF10_1010 } else { 0xFF18_1818 },
    );
    batch.fill(
        &game.gui.atlas,
        bx,
        top,
        bw,
        1.0,
        if focused { 0xFF70_7070 } else { 0xFF40_4040 },
    );
    batch.fill(
        &game.gui.atlas,
        bx,
        top + SEARCH_H - 1.0,
        bw,
        1.0,
        if focused { 0xFF70_7070 } else { 0xFF40_4040 },
    );
    if search.is_empty() && !focused {
        batch.text(&game.gui.atlas, "Search", bx + 5.0, top + 4.0, 0xFF80_8080, true);
    } else {
        batch.text(&game.gui.atlas, &search, bx + 5.0, top + 4.0, u32::MAX, true);
        if focused {
            let before: String = search.chars().take(caret).collect();
            let cx = bx + 6.0 + HudBatch::text_width(&game.gui.atlas, &before);
            batch.fill(&game.gui.atlas, cx, top + 3.0, 1.0, SEARCH_H - 6.0, u32::MAX);
        }
    }

    // The rows, clipped to the list.
    let row_y = |i: f32, rh: f32| list_top + 4.0 + i * rh - scroll;
    let visible = |y: f32, rh: f32| y >= list_top && y + rh <= bottom;
    match tab {
        // General: the label left, the value right.
        0 => {
            for (i, (key, label)) in general_rows.iter().enumerate() {
                let y = row_y(i as f32, ROW_H);
                if !visible(y, ROW_H) {
                    continue;
                }
                let f = statview::CUSTOM
                    .iter()
                    .find(|(k, _)| &format!("minecraft:{k}") == key)
                    .map(|(_, f)| *f)
                    .unwrap_or(statview::Format::Default);
                let value = statview::format(f, game.stats.get("custom", key));
                let color = if i % 2 == 0 { u32::MAX } else { 0xFFBA_BABA };
                batch.text(&game.gui.atlas, label, left + 2.0, (y + 2.0).floor(), color, true);
                let tw = HudBatch::text_width(&game.gui.atlas, &value);
                batch.text(
                    &game.gui.atlas,
                    &value,
                    left + content_w - tw - 6.0,
                    (y + 2.0).floor(),
                    color,
                    true,
                );
            }
        }
        // Items: a grid of slots, the sorted column's count as a badge, the
        // picked slot's columns in a strip above the footer.
        1 => {
            for (c, ty) in statview::ITEM_COLUMNS.into_iter().enumerate() {
                let hx = left + 2.0 + c as f32 * 64.0;
                let hy = list_top;
                if hy >= bottom {
                    break;
                }
                let label = crate::game::screen_info::tr(&format!("stat_type.minecraft.{ty}"), &[]);
                batch.text(&game.gui.atlas, &label, hx, hy + 3.0, 0xFF80_8080, true);
                let marker = if sort_col == c {
                    if sort_asc { "^" } else { "v" }
                } else {
                    " "
                };
                batch.text(&game.gui.atlas, marker, hx + 52.0, hy + 3.0, u32::MAX, true);
            }
            let grid_top = list_top + 14.0;
            let offset = scroll.max(0.0);
            for (i, (id, _)) in item_rows.iter().enumerate() {
                let (gx, gy) = (i % cols, i / cols);
                let (x, y) = (
                    left + 2.0 + gx as f32 * SLOT,
                    grid_top + gy as f32 * SLOT - offset,
                );
                if y < grid_top - SLOT || y + SLOT > bottom {
                    continue;
                }
                let stack = ItemStack::new(id, 1);
                let picked = selected.as_deref() == Some(id.as_str());
                batch.sprite(&game.gui.atlas, "gui/sprites/container/slot", x, y);
                game.draw_item(batch, &stack, x + 2.0, y + 2.0);
                if picked {
                    batch.fill(&game.gui.atlas, x, y, SLOT, 1.0, u32::MAX);
                    batch.fill(&game.gui.atlas, x, y + SLOT - 1.0, SLOT, 1.0, u32::MAX);
                    batch.fill(&game.gui.atlas, x, y, 1.0, SLOT, u32::MAX);
                    batch.fill(&game.gui.atlas, x + SLOT - 1.0, y, 1.0, SLOT, u32::MAX);
                }
                let count = game.stats.get(statview::ITEM_COLUMNS[sort_col], id);
                if count > 0 {
                    let text = statview::format(statview::Format::Default, count);
                    let tw = HudBatch::text_width(&game.gui.atlas, &text);
                    batch.text(
                        &game.gui.atlas,
                        &text,
                        x + SLOT - tw - 1.0,
                        y + SLOT - 10.0,
                        0xFFFF_FF55,
                        true,
                    );
                }
                if ui::hovered(game.cursor, x, y, SLOT, SLOT) {
                    game.draw_stack_tooltip(batch, &stack, x + SLOT, y + SLOT);
                }
            }
            if let Some(id) = &selected {
                let name = item_name(game, id);
                let strip_y = bottom - 16.0;
                batch.fill(&game.gui.atlas, left, strip_y, content_w, 14.0, 0xFF10_1010);
                batch.text(&game.gui.atlas, &name, left + 4.0, strip_y + 2.0, u32::MAX, true);
                let mut x = left + content_w * 0.3;
                for (c, ty) in statview::ITEM_COLUMNS.into_iter().enumerate() {
                    let label =
                        crate::game::screen_info::tr(&format!("stat_type.minecraft.{ty}"), &[]);
                    let value =
                        statview::format(statview::Format::Default, game.stats.get(ty, id));
                    batch.text(
                        &game.gui.atlas,
                        &format!("{label}: {value}"),
                        x,
                        strip_y + 2.0,
                        0xFFBA_BABA,
                        true,
                    );
                    x += 90.0;
                    if c == 1 {
                        break;
                    }
                }
            }
        }
        // Mobs: the name, its kills and its killed-by, as the vanilla rows read.
        _ => {
            for (i, (mob, _name)) in mob_rows.iter().enumerate() {
                let y = row_y(i as f32, MOB_ROW);
                if !visible(y, MOB_ROW) {
                    continue;
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
                    if kills > 0 { 0xFFBA_BABA } else { 0xFF80_8080 },
                    if by > 0 { 0xFFBA_BABA } else { 0xFF80_8080 },
                );
                batch.text_styled(&game.gui.atlas, &mob_name, left + 2.0, y + 1.0, u32::MAX, true);
                batch.text_styled(&game.gui.atlas, &k, left + 12.0, y + 10.0, kc, true);
                batch.text_styled(&game.gui.atlas, &b, left + 12.0, y + 19.0, bc, true);
            }
        }
    }
    let bar_x = w - 12.0;
    ui::scrollbar(batch, &game.gui.atlas, bar_x, list_top, bottom, content, scroll);
    // The footer's Done button.
    let (bx, by) = ((w / 2.0 - 100.0).floor(), h - FOOTER + 6.0);
    let hover = ui::hovered(game.cursor, bx, by, 200.0, 20.0);
    ui::action_button(
        batch,
        &game.gui.atlas,
        horizon_client::widgets::Action::Done,
        bx,
        by,
        200.0,
        20.0,
        &Text::translate("gui.done", &[]),
        true,
        hover,
    );
    core::ops::ControlFlow::Break(())
}
