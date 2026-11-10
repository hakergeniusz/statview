// A mouse release on the rebuilt screen: the mod owns every click here, so
// the vanilla scrollbar's hold-and-release never runs while it asks.
// Copyright (c) 2026 hakergeniusz. MIT.
fn statview_release(game: &mut crate::game::Game<'_>) -> core::ops::ControlFlow<()> {
    use crate::game::statview as statview;
    if game.native_on(statview::ID)
        && statview::setting_on(|key| game.native_setting(statview::ID, key))
    {
        core::ops::ControlFlow::Break(())
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
