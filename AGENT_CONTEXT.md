# Strangely Charming

Version 0.2.10. This Horizon native mod uses the `strangely_charming` namespace. The supported engine and mixin compiler revisions are recorded in `compatibility.json`. Mod feature flags default to true and are applied at startup.

## Development

Edit resources in `content/` and native hooks in `mixins/strangely_charming/`. Edit the embedding template in `tools/embed_content.py` when changing config-dependent resource selection. The packager regenerates `content_bundle.rs`. Keep the mod version in both manifests and Cargo metadata in sync with `VERSION`.

Run `cargo test --locked`, then `python3 tools/package.py` and `cargo horizon-mod check target/strangely_charming-0.2.10.hnmod --json`. Install the resulting archive with the Horizon launcher on the pinned Nightly. Gameplay requires the compiled native mod on client and server.

## Releases and rollback

Release tags use `vX.Y.Z`. A matching source archive is attached to each published release. Back up the world before changing versions and restore its matching snapshot when rolling back a namespace migration.

## Current scope

Rune circles, icy chambers and Nether ruins are native searchable jigsaw structures. The Runebreaker destroys a carved capstone without a drop. Generation switches remove structure sets while retaining saved block and template IDs. Quest rewards and XP storage are authoritative on the server. Keep upstream artwork and license notices with redistributions.
