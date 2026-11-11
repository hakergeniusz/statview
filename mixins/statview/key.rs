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
                // A printable key: broken here so the engine's char callback
                // (which only follows a canceled key) delivers the platform's
                // own character into the box. Control-held keys stay
                // shortcuts.
                _ if !ctrl
                    && matches!(
                        key,
                        KeyCode::KeyA
                            | KeyCode::KeyB
                            | KeyCode::KeyC
                            | KeyCode::KeyD
                            | KeyCode::KeyE
                            | KeyCode::KeyF
                            | KeyCode::KeyG
                            | KeyCode::KeyH
                            | KeyCode::KeyI
                            | KeyCode::KeyJ
                            | KeyCode::KeyK
                            | KeyCode::KeyL
                            | KeyCode::KeyM
                            | KeyCode::KeyN
                            | KeyCode::KeyO
                            | KeyCode::KeyP
                            | KeyCode::KeyQ
                            | KeyCode::KeyR
                            | KeyCode::KeyS
                            | KeyCode::KeyT
                            | KeyCode::KeyU
                            | KeyCode::KeyV
                            | KeyCode::KeyW
                            | KeyCode::KeyX
                            | KeyCode::KeyY
                            | KeyCode::KeyZ
                            | KeyCode::Space
                            | KeyCode::Digit0
                            | KeyCode::Digit1
                            | KeyCode::Digit2
                            | KeyCode::Digit3
                            | KeyCode::Digit4
                            | KeyCode::Digit5
                            | KeyCode::Digit6
                            | KeyCode::Digit7
                            | KeyCode::Digit8
                            | KeyCode::Digit9
                            | KeyCode::Minus
                            | KeyCode::Equal
                            | KeyCode::BracketLeft
                            | KeyCode::BracketRight
                            | KeyCode::Semicolon
                            | KeyCode::Quote
                            | KeyCode::Backslash
                            | KeyCode::IntlBackslash
                            | KeyCode::Comma
                            | KeyCode::Period
                            | KeyCode::Slash
                            | KeyCode::Backquote
                            | KeyCode::Numpad0
                            | KeyCode::Numpad1
                            | KeyCode::Numpad2
                            | KeyCode::Numpad3
                            | KeyCode::Numpad4
                            | KeyCode::Numpad5
                            | KeyCode::Numpad6
                            | KeyCode::Numpad7
                            | KeyCode::Numpad8
                            | KeyCode::Numpad9
                    ) =>
                {
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
