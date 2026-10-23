# Oracle Motion app icon

**Creature:** a unicorn, as an engraved head-and-shoulders bust looking out of the tile.

**Style:** an ink engraving portrait in the Crafting Apps' owl-template framing (the FilmCraft owl):
full-bleed colour field, tight head-and-shoulders crop, the body running off the bottom and right edges.

**Tile:** 512 × 512 viewBox, rounded square `rx=112`, clipped; no border or roundel.

**Palette** (exactly three colours):

| Colour | Hex | Used for |
|---|---|---|
| App colour (Oracle Motion pink) | `#e0368f` | the full-bleed field |
| Paper | `#efe9dc` | the figure |
| Ink | `#0b0b0c` | line work and contour |

**Provenance:** the project owner drew it in ArtCraft (a 2880 px engraving-style image, kept in
craftrules at `assets/app-icons/oracle-motion/source.png`). It was vectorised with craftrules
`assets/logo-options/_tools/vectorize_tile.py`, which keys out the field and traces the ink with potrace.
Licence: see [LICENSE.txt](LICENSE.txt) (MIT OR Apache-2.0, like the repo).

## Files

| File | What |
|---|---|
| `oracle-motion.svg` | master vector, traced at 2048 px (canonical) |
| `oracle-motion-small.svg` | lighter vector (~350 KB), traced at 1024 px; also the web favicon |
| `oracle-motion-1024.png` | 1024 px render |
| `oracle-motion-macos-512.png` | runtime Dock icon on macOS, on Apple's 824/1024 grid |
| `oracle-motion.icns` | macOS icon (Apple grid) |
| `oracle-motion.ico` | Windows icon, 16–256 px; embedded in `oracle-motion.exe` by `apps/oracle-motion/build.rs` |
| `hicolor/<n>x<n>/apps/app.oracle.oracle-motion.png` | Linux icon theme, 16–512 px; the 256 px one is the runtime icon on Windows and Linux |
| `hicolor/scalable/apps/app.oracle.oracle-motion.svg` | Linux scalable icon |

App ID: `app.oracle.oracle-motion` (Wayland app ID, `packaging/linux/app.oracle.oracle-motion.desktop`).

## Regenerate

Copy a new `icon-master.svg` / `icon.svg` from craftrules `assets/app-icons/oracle-motion/` over
`oracle-motion.svg` / `oracle-motion-small.svg`, then run `packaging/icons.sh` (needs `resvg`; `iconutil`
on macOS for the `.icns`). Copy `oracle-motion-small.svg` to `apps/oracle-motion-web/web/favicon.svg` too.
