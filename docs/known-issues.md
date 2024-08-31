# Known Issues

## My audio is stuttering on web

There are a number of issues with audio on web, so this is not an exhaustive list. The short version is that you can try the following:

- If you use materials, make sure to force render pipelines to [load at the start of the game](https://github.com/rparrett/bevy_pipelines_ready/blob/main/src/lib.rs).
- Keep the FPS high.
- Advise your users to play on Chromium-based browsers.
- Apply the suggestions from the blog post [Workaround for the Choppy Music in Bevy Web Builds](https://necrashter.github.io/bevy-choppy-music-workaround).

## My game window is flashing white for a split second when I start the game on native

The game window is created before the GPU is ready to render everything.
This means that it will start with a white screen for a little bit.
The workaround is to [spawn the Window hidden](https://github.com/bevyengine/bevy/blob/release-0.14.0/examples/window/window_settings.rs#L29-L32)
and then [make it visible after a few frames](https://github.com/bevyengine/bevy/blob/release-0.14.0/examples/window/window_settings.rs#L56-L64).

## My character or camera is not moving smoothly

Choppy movement is often caused by movement updates being tied to the frame rate.
See the [physics_in_fixed_timestep](https://github.com/bevyengine/bevy/blob/main/examples/movement/physics_in_fixed_timestep.rs) example
for how to fix this.

A camera not moving smoothly is pretty much always caused by the camera position being tied too tightly to the character's position.
To give the camera some inertia, use the [`smooth_nudge`](https://github.com/bevyengine/bevy/blob/main/examples/movement/smooth_follow.rs#L127-L142)
to interpolate the camera position towards its target position.
