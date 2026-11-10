// Typed characters, while the search box holds the keyboard on the rebuilt
// screen: they land in the search, and nothing else sees them.
// Copyright (c) 2026 hakergeniusz. MIT.
fn statview_text(game: &mut crate::game::Game<'_>, text: &str) -> core::ops::ControlFlow<Result<()>> {
    use crate::game::statview as statview;

    if game.screen != crate::game::Screen::Stats
        || !game.native_on(statview::ID)
        || !statview::setting_on(|key| game.native_setting(statview::ID, key))
    {
        return core::ops::ControlFlow::Continue(());
    }
    let focused = statview::with_view(|v| {
        if v.focused {
            v.insert(text);
            true
        } else {
            false
        }
    });
    if focused {
        core::ops::ControlFlow::Break(Ok(()))
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
