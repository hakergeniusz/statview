// A key on the rebuilt screen, before the vanilla one runs. The search box
// takes its editing keys while it is focused (Escape lets go of it); Ctrl+F
// takes it; the digits and Ctrl+Tab move between tabs as the vanilla screen
// taught. Everything else goes on to the vanilla handler.
//
// While the box is focused, printable keys are typed into it here, from the
// key itself (US layout, shift-aware): the engine's char callback only
// serves mod-opened screens, so a vanilla screen's hooks never hear typed
// characters. The key is broken so nothing else sees it; control-held keys
// stay shortcuts.
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
                _ if !ctrl => {
                    let letter = |base: char| {
                        if shift {
                            base.to_ascii_uppercase().to_string()
                        } else {
                            base.to_string()
                        }
                    };
                    let text = match (key, shift) {
                        (KeyCode::KeyA, _) => Some(letter('a')),
                        (KeyCode::KeyB, _) => Some(letter('b')),
                        (KeyCode::KeyC, _) => Some(letter('c')),
                        (KeyCode::KeyD, _) => Some(letter('d')),
                        (KeyCode::KeyE, _) => Some(letter('e')),
                        (KeyCode::KeyF, _) => Some(letter('f')),
                        (KeyCode::KeyG, _) => Some(letter('g')),
                        (KeyCode::KeyH, _) => Some(letter('h')),
                        (KeyCode::KeyI, _) => Some(letter('i')),
                        (KeyCode::KeyJ, _) => Some(letter('j')),
                        (KeyCode::KeyK, _) => Some(letter('k')),
                        (KeyCode::KeyL, _) => Some(letter('l')),
                        (KeyCode::KeyM, _) => Some(letter('m')),
                        (KeyCode::KeyN, _) => Some(letter('n')),
                        (KeyCode::KeyO, _) => Some(letter('o')),
                        (KeyCode::KeyP, _) => Some(letter('p')),
                        (KeyCode::KeyQ, _) => Some(letter('q')),
                        (KeyCode::KeyR, _) => Some(letter('r')),
                        (KeyCode::KeyS, _) => Some(letter('s')),
                        (KeyCode::KeyT, _) => Some(letter('t')),
                        (KeyCode::KeyU, _) => Some(letter('u')),
                        (KeyCode::KeyV, _) => Some(letter('v')),
                        (KeyCode::KeyW, _) => Some(letter('w')),
                        (KeyCode::KeyX, _) => Some(letter('x')),
                        (KeyCode::KeyY, _) => Some(letter('y')),
                        (KeyCode::KeyZ, _) => Some(letter('z')),
                        (KeyCode::Space, _) => Some(" ".to_owned()),
                        (KeyCode::Digit0, s) => Some(if s { ")" } else { "0" }.to_owned()),
                        (KeyCode::Digit1, s) => Some(if s { "!" } else { "1" }.to_owned()),
                        (KeyCode::Digit2, s) => Some(if s { "@" } else { "2" }.to_owned()),
                        (KeyCode::Digit3, s) => Some(if s { "#" } else { "3" }.to_owned()),
                        (KeyCode::Digit4, s) => Some(if s { "$" } else { "4" }.to_owned()),
                        (KeyCode::Digit5, s) => Some(if s { "%" } else { "5" }.to_owned()),
                        (KeyCode::Digit6, s) => Some(if s { "^" } else { "6" }.to_owned()),
                        (KeyCode::Digit7, s) => Some(if s { "&" } else { "7" }.to_owned()),
                        (KeyCode::Digit8, s) => Some(if s { "*" } else { "8" }.to_owned()),
                        (KeyCode::Digit9, s) => Some(if s { "(" } else { "9" }.to_owned()),
                        (KeyCode::Minus, s) => Some(if s { "_" } else { "-" }.to_owned()),
                        (KeyCode::Equal, s) => Some(if s { "+" } else { "=" }.to_owned()),
                        (KeyCode::BracketLeft, s) => Some(if s { "{" } else { "[" }.to_owned()),
                        (KeyCode::BracketRight, s) => Some(if s { "}" } else { "]" }.to_owned()),
                        (KeyCode::Semicolon, s) => Some(if s { ":" } else { ";" }.to_owned()),
                        (KeyCode::Quote, s) => Some(if s { "\"" } else { "'" }.to_owned()),
                        (KeyCode::Comma, s) => Some(if s { "<" } else { "," }.to_owned()),
                        (KeyCode::Period, s) => Some(if s { ">" } else { "." }.to_owned()),
                        (KeyCode::Slash, s) => Some(if s { "?" } else { "/" }.to_owned()),
                        (KeyCode::Backslash, s) => Some(if s { "|" } else { "\\" }.to_owned()),
                        (KeyCode::IntlBackslash, s) => Some(if s { "|" } else { "\\" }.to_owned()),
                        _ => None,
                    };
                    match text {
                        Some(text) => {
                            v.insert(&text);
                            true
                        }
                        None => false,
                    }
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
