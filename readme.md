# Foxtrot
[![Live Demo](https://img.shields.io/badge/live%20demo-v0.1.10-blue)](https://janhohenheim.github.io/foxtrot/)

The all-in-one Bevy 3D game template.  


![Foxtrot in action](https://media.giphy.com/media/NKBVjKZewDfttXSx56/giphy.gif)

I created Foxtrot because I wanted to have a quick starting point for jams, prototypes and projects supporting features
that I want to use in my games. Since the target audience is me, the code is not super well documented, but it should 
be good enough for others to find inspiration, copy parts they like or troubleshoot their own implementations :)
 

## What does this template give you?
- A 3D character controller
- Physics via `bevy_rapier`
- Audio via `bevy_kira_audio`
- Pathfinding via `oxidized_navigation`
- `bevy_editor_pls` in the `dev` feature, bound to 'Q'
- Custom editor for the game state found in the windows for `bevy_editor_pls`.
- Saving / loading levels
- Saving / loading the game state
- Animations
- A custom dialog system
- Shaders
- GLTF imports, including colliders and navmeshes
- dynamic builds in the `native-dev` feature

## Usage

### Cloning the repo

Make sure you have [Git LFS](https://docs.github.com/en/repositories/working-with-files/managing-large-files/installing-git-large-file-storage) installed before cloning the repo, otherwise most runtime resources will be missing!

### Running the game
Native:
```bash
cargo run
```
WASM (runs best on Chromium based browsers):
```bash
trunk serve --no-default-features --features wasm_dev
```

Building in general requires setting up LLD or ZLD as described in the [Bevy book](https://bevyengine.org/learn/book/getting-started/setup/#enable-fast-compiles-optional).
Don't worry, it's super easy:
- **Ubuntu**: `sudo apt-get install lld`
- **Arch**: `sudo pacman -S lld`
- **Windows**: Ensure you have the latest [cargo-binutils](https://github.com/rust-embedded/cargo-binutils)

    ```sh
    cargo install -f cargo-binutils
    rustup component add llvm-tools-preview
    ```

- **MacOS**: Modern LLD does not yet support MacOS, but we can use zld instead: `brew install michaeleisel/zld/zld`


Building WASM requires `trunk`:

```bash
cargo install --locked trunk
```

### Updating assets

You should keep the `credits` directory up to date. The release workflow automatically includes the directory in every build.

### Updating the icons
 1. Replace `build/windows/icon.ico` (used for windows executable and as favicon for the web-builds)
 2. Replace `build/macos/icon_1024x1024.png` with a `1024` times `1024` pixel png icon and run `create_icns.sh` (make sure to run the script inside the `macos` directory) - _Warning: sadly this seems to require a mac..._


