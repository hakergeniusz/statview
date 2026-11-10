// The mod's settings page: the single Enabled row, in the state the file
// holds (read fresh from disk, so the page always shows what the gates act
// on).
// Copyright (c) 2026 hakergeniusz. MIT.
fn statview_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::statview as statview;
    if mod_id != statview::ID {
        return;
    }
    let game = &*game;
    let on = statview::setting_on(|key| game.native_setting(statview::ID, key));
    buttons.push(statview::label(on));
}
