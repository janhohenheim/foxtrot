# Recommended 3rd-party tools

Check out the [Bevy Assets](https://bevyengine.org/assets/) page for more great options.

## Libraries

A few libraries that the authors of this template have vetted and think you might find useful:

| Name                                                                                   | Category       | Description                           |
| -------------------------------------------------------------------------------------- | -------------- | ------------------------------------- |
| [`leafwing-input-manager`](https://github.com/Leafwing-Studios/leafwing-input-manager) | Input          | Input -> Action mapping               |
| [`bevy_mod_picking`](https://github.com/aevyrie/bevy_mod_picking)                      | Input          | Advanced mouse interaction            |
| [`bevy-inspector-egui`](https://github.com/jakobhellermann/bevy-inspector-egui)        | Debugging      | Live entity inspector                 |
| [`bevy_mod_debugdump`](https://github.com/jakobhellermann/bevy_mod_debugdump)          | Debugging      | Schedule inspector                    |
| [`avian`](https://github.com/Jondolf/avian)                                            | Physics        | Physics engine                        |
| [`bevy_rapier`](https://github.com/dimforge/bevy_rapier)                               | Physics        | Physics engine (not ECS-driven)       |
| [`bevy_common_assets`](https://github.com/NiklasEi/bevy_common_assets)                 | Asset loading  | Asset loaders for common file formats |
| [`bevy_asset_loader`](https://github.com/NiklasEi/bevy_asset_loader)                   | Asset loading  | Asset management tools                |
| [`iyes_progress`](https://github.com/IyesGames/iyes_progress)                          | Asset loading  | Progress tracking                     |
| [`bevy_kira_audio`](https://github.com/NiklasEi/bevy_kira_audio)                       | Audio          | Advanced audio                        |
| [`sickle_ui`](https://github.com/UmbraLuminosa/sickle_ui)                              | UI             | UI widgets                            |
| [`bevy_egui`](https://github.com/mvlabat/bevy_egui)                                    | UI / Debugging | UI framework (great for debug UI)     |
| [`tiny_bail`](https://github.com/benfrankel/tiny_bail)                                 | Error handling | Error handling macros                 |

In particular:

- `leafwing-input-manager` and `bevy_mod_picking` are very likely to be upstreamed into Bevy in the near future.
- `bevy-inspector-egui` and `bevy_mod_debugdump` help fill the gap until Bevy has its own editor.
- `avian` or `bevy_rapier` helps fill the gap until Bevy has its own physics engine. `avian` is easier to use, while `bevy_rapier` is more performant.
- `sickle_ui` is well-aligned with `bevy_ui` and helps fill the gap until Bevy has a full collection of UI widgets.

None of these are necessary, but they can save you a lot of time and effort.

## VS Code extensions

If you're using [VS Code](https://code.visualstudio.com/), the following extensions are highly recommended:

| Name                                                                                                      | Description                       |
|-----------------------------------------------------------------------------------------------------------|-----------------------------------|
| [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)              | Rust support                      |
| [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)          | TOML support                      |
| [vscode-ron](https://marketplace.visualstudio.com/items?itemName=a5huynh.vscode-ron)                      | RON support                       |
| [Dependi](https://marketplace.visualstudio.com/items?itemName=fill-labs.dependi)                          | `crates.io` dependency resolution |
| [EditorConfig for VS Code](https://marketplace.visualstudio.com/items?itemName=EditorConfig.EditorConfig) | `.editorconfig` support           |

> [!Note]
> <details>
> <summary>About the included rust-analyzer settings</summary>
>
> This template sets [`rust-analyzer.cargo.targetDir`](https://rust-analyzer.github.io/generated_config.html#rust-analyzer.cargo.targetDir)
> to `true` in [`.vscode/settings.json`](../.vscode/settings.json).
>
> This makes `rust-analyzer` use a different `target` directory than `cargo`,
> which means that you can run commands like `cargo run` even while `rust-analyzer` is still indexing.
> As a trade-off, this will use more disk space.
>
> If that is an issue for you, you can set it to `false` or remove the setting entirely.
> </details>

## Other templates

There are many other Bevy templates out there.
Check out the [templates category](https://bevyengine.org/assets/#templates) on Bevy Assets for more options.
Even if you don't end up using them, they are a great way to learn how to implement certain features you might be interested in.
