// The Statistics screen opened: the view starts over, whatever it held when
// the screen was last up.
// Copyright (c) 2026 hakergeniusz. MIT.
fn statview_opened(game: &mut crate::game::Game<'_>) {
    use crate::game::statview as statview;
    let _ = game;
    statview::with_view(|v| v.reset());
}
