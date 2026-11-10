// A wheel tick on the rebuilt screen: the view's scroll moves, clamped to
// what the last frame drew.
// Copyright (c) 2026 hakergeniusz. MIT.
fn statview_scroll(game: &mut crate::game::Game<'_>, amount: f32) -> core::ops::ControlFlow<()> {
    use crate::game::statview as statview;

    if !game.native_on(statview::ID)
        || !statview::setting_on(|key| game.native_setting(statview::ID, key))
    {
        return core::ops::ControlFlow::Continue(());
    }
    statview::with_view(|v| {
        if let Some((top, bottom, content)) = v.layout {
            v.scrolled(amount, top, bottom, content);
        }
    });
    core::ops::ControlFlow::Break(())
}
