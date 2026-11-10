// A click on the mod's settings page: flip Enabled and keep it in the
// setting file the gates re-read, so the change lands within the tick that
// follows the click.
// Copyright (c) 2026 hakergeniusz. MIT.
fn statview_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::statview as statview;
    if mod_id != statview::ID {
        return;
    }
    let clicked = {
        let game = &*game;
        statview::click(button, statview::setting_on(|key| {
            game.native_setting(statview::ID, key)
        }))
    };
    if let Some(on) = clicked {
        let (key, value) = statview::stored(on);
        game.native_set_setting(statview::ID, key, &value);
    }
}
