# StatView

A rebuilt Statistics screen for Horizon: **search that filters**, **columns
that sort**, and **an item grid with counts at a glance** — instead of the
vanilla screen's narrow list you scroll through by hand. A Horizon native
mod (mixin bundle), independent original work under MIT.

Clean-room: the feature set follows [Better Statistics Screen](https://github.com/TheCSDev/betterstats)
(TheCSDev) — search/filter/sort over the statistics list and a visual item
grid — but that mod's license reserves all rights, so **nothing from it is
copied or derived**: no code, no assets, no text. What StatView reads and
shows is the vanilla game's own statistics, which Horizon already tracks.

## What you get

- **Search** — type in the box (Ctrl+F focuses it); every word must hit the
  entry's name or id, so `dia pic` narrows hundreds of items to the pic-
  kned-up diamond things. The caret, backspace, delete, Home/End and arrows
  all work.
- **Tabs** — General, Items and Mobs as before; the digits 1–3 and Ctrl+Tab
  switch them.
- **Items as a grid** — every item with any count, drawn as slots with the
  sorted column's number as a badge; hover for the item's tooltip; click for
  its six columns in a strip under the grid.
- **Sort** — click a column header on the Items tab: it sorts descending by
  that column first (the biggest counts first, as the vanilla screen
  starts), click again to flip. General and Mobs sort by name.
- **Wheel and scrollbar** — like the vanilla screen's.
- **Toggle** — Options > Mods > StatView, `Enabled` (default ON): off gives
  the vanilla screen back, no restart.

What it does not do (yet): 3D mob previews in the grid, saving/loading and
sharing statistics files, and a `/statistics` command.

## How it fits together

The vanilla screen's drawing, keys, clicks, wheel and typed text each run
through their own function; StatView hooks all six and answers with its own
while its toggle is on — the vanilla Statistics screen is one toggle away,
unchanged. The view model (search, caret, tabs, sort, scroll, formats) is
plain standard library Rust, doubled as the crate's library and tested (7
tests); the helpers are thin splices that feed it the game's statistics
(`stats.get`, `stat_items`, `stat_mobs`) and draw its answer.

Verification recipe:

```sh
python3 -I tools/test.py           # fmt, clippy, the view model's tests
python3 -I tools/horizon.py check  # staged cargo check on the pinned revision
python3 -I tools/horizon.py test   # staged view-model tests
python3 -I tools/package.py        # target/statview-1.0.0.hnmod
```

## License

MIT — see [LICENSE](LICENSE). Better Statistics Screen is acknowledged as
the inspiration for the feature set only; it is not used, linked or
redistributed here.
