# Foxtrot

The all-in-one Bevy 3D game template.

https://user-images.githubusercontent.com/9047632/226387411-70f662de-0681-47ff-b1d1-ccc59b02fa7b.mov

## What does this template give you?

- A 3D character controller via [`bevy-tnua`](https://crates.io/crates/bevy-tnua)
- Physics via [`bevy_xpbd`](https://crates.io/crates/bevy_xpbd_3d)
- Audio via [`bevy_kira_audio`](https://crates.io/crates/bevy_kira_audio)
- Pathfinding via [`oxidized_navigation`](https://crates.io/crates/oxidized_navigation)
- [`bevy_editor_pls`](https://crates.io/crates/bevy_editor_pls) bound to 'Q'
- Custom editor found in the windows selection for `bevy_editor_pls`.
- Animations
- Dialogs via [`Yarn Spinner for Rust`](https://crates.io/crates/bevy_yarnspinner)
- Shaders, using the code from [DGriffin's tutorial](https://www.youtube.com/watch?v=O6A_nVmpvhc)
- GLTF imports, including auto-insertion of markers via the GLTF extras field
- Smooth cameras via [`bevy_dolly`](https://crates.io/crates/bevy_dolly)
- Simple error handling via [`bevy_mod_sysfail`](https://crates.io/crates/bevy_mod_sysfail)
- Particle effects via [`bevy_hanabi`](https://crates.io/crates/bevy_hanabi)
- Procedural skies via [`bevy_atmosphere`](https://crates.io/crates/bevy_atmosphere)
- Grass via [`warbler_grass`](https://crates.io/crates/warbler_grass)
- Integration with Blender as an editor via
  the [`Blender_bevy_components_workflow`](https://github.com/kaosat-dev/Blender_bevy_components_workflow) set of tools

## Usage

Simply use the [template button on GitHub](https://github.com/janhohenheim/foxtrot/generate) to create a new repository
from this template.
Then, replace all instances of the word `foxtrot` with the name of your game. Change the game version as well as the
author information in the following files:

- `Cargo.toml`
- `build/windows/installer/Package.wxs`
- `build/macos/src/Game.app/Contents/Resources/Info.plist`

### Updating assets

You should keep the `credits` directory up to date. The release workflow automatically includes the directory in every
build.

### Updating the icons

1. Replace `build/windows/icon.ico` (used for windows executable and as favicon for the web-builds)
2. Replace `build/macos/icon_1024x1024.png` with a `1024` times `1024` pixel png icon and run `create_icns.sh` (make
   sure to run the script inside the `macos` directory) - _Warning: sadly this seems to require a mac..._

## Help and Discussion

Feel free to shoot a message in the
dedicated [help thread on the Bevy Discord](https://discord.com/channels/691052431525675048/1110648523558506597)
or [open an issue on GitHub](https://github.com/janhohenheim/foxtrot/issues/new) if you want to discuss something or
need help :)
