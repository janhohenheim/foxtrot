# My personal Bevy game template

Template for a Game using the awesome [Bevy engine][bevy] featuring out of the box builds for Windows, Linux, macOS, and Web (Wasm).
Adapted to my needs from [NiklasEi/bevy_game_template](https://github.com/NiklasEi/bevy_game_template).
# What does this template give you?
* small example ["game"](https://janhohenheim.github.io/bevy-game-template/) (*warning: biased; e.g., split into a lot of plugins and using `bevy_kira_audio` for sound*)
* networking using ggrs
* easy setup for running the web build using [trunk] (`trunk serve`) 
* run the native version with `cargo run`
* workflow for GitHub actions creating releases for Windows, Linux, macOS, and Web (Wasm) ready for distribution
    * push a tag in the form of `v[0-9]+.[0-9]+.[0-9]+*` (e.g. `v1.1.42`) to trigger the flow

## How to use this template?
 1. Click "Use this template" on the repository's page
 2. Look for `ToDo` to use your own game name everywhere
 3. [Update the icons as described below](#updating-the-icons)
 4. Start coding :tada:
    * Start the native app: `cargo run`
    * Start the web build: `trunk serve`
       * requires [trunk]: `cargo install --locked trunk`
       * requires `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
       * this will serve your app on `8080` and automatically rebuild + reload it after code changes

You should keep the `credits` directory up to date. The release workflow automatically includes the directory in every build.

### Updating the icons
 1. Replace `build/windows/icon.ico` (used for windows executable and as favicon for the web-builds)
 2. Replace `build/macos/icon_1024x1024.png` with a `1024` times `1024` pixel png icon and run `create_icns.sh` (make sure to run the script inside the `macos` directory) - _Warning: sadly this seems to require a mac..._

### Deploy web build to GitHub pages
 1. Activate [GitHub pages](https://pages.github.com/) for your repository
    1. Source from the `gh-pages` branch
 2. Trigger the `deploy-github-page` workflow
 3. After a few minutes your game is live at `http://username.github.io/repository`

## Known issues

[Does currently not work on Firefox](https://github.com/johanhelsing/matchbox/issues/36)

## License

This project is licensed under [CC0 1.0 Universal](LICENSE) except some content of `assets` and the Bevy icons in the `build` directory (see [Credits](credits/CREDITS.md)).

[bevy]: https://bevyengine.org/
[Bevy Cheat Book]: https://bevy-cheatbook.github.io/introduction.html
[trunk]: https://trunkrs.dev/

