// A key on the rebuilt screen, before the vanilla one runs. The search box
// takes its editing keys while it is focused (Escape lets go of it); Ctrl+F
// takes it; the digits and Ctrl+Tab move between tabs as the vanilla screen
// taught. Everything else goes on to the vanilla handler.
// Copyright (c) 2026 hakergeniusz. MIT.
fn statview_key(game: &mut crate::game::Game<'_>, key: winit::keyboard::KeyCode) -> core::ops::ControlFlow<()> {
    use crate::game::statview as statview;
    use winit::keyboard::KeyCode;

    if !game.native_on(statview::ID)
        || !statview::setting_on(|key| game.native_setting(statview::ID, key))
    {
        return core::ops::ControlFlow::Continue(());
    }
    let ctrl = game.has_control_down_with_quirk();
    let shift = game.has_shift_down();
    let handled = statview::with_view(|v| {
        if v.focused {
            match key {
                KeyCode::Escape => {
                    v.focused = false;
                    true
                }
                KeyCode::Backspace => {
                    v.backspace();
                    true
                }
                KeyCode::Delete => {
                    v.delete();
                    true
                }
                KeyCode::ArrowLeft => {
                    v.left();
                    true
                }
                KeyCode::ArrowRight => {
                    v.right();
                    true
                }
                KeyCode::Home => {
                    v.home();
                    true
                }
                KeyCode::End => {
                    v.end();
                    true
                }
                KeyCode::Enter | KeyCode::NumpadEnter => {
                    v.focused = false;
                    true
                }
                _ => false,
            }
        } else {
            match key {
                KeyCode::KeyF if ctrl => {
                    v.focused = true;
                    true
                }
                KeyCode::Tab if ctrl => {
                    v.tab_key(None, if shift { -1 } else { 1 });
                    true
                }
                KeyCode::Digit1 => v.tab_key(Some(1), 0),
                KeyCode::Digit2 => v.tab_key(Some(2), 0),
                KeyCode::Digit3 => v.tab_key(Some(3), 0),
                _ => false,
            }
        }
    });
    if handled {
        core::ops::ControlFlow::Break(())
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
