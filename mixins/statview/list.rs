// The mod's row in Options > Mods: one toggle, no keys.
// Copyright (c) 2026 hakergeniusz. MIT.
fn statview_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::statview as statview;
    mods.push(crate::game::native_hooks::NativeMod {
        id: statview::ID.into(),
        title: format!("StatView {}", statview::VERSION),
        description: statview::DESCRIPTION.into(),
        keys: vec![],
    });
}
