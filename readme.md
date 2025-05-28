# Foxtrot

A 3D reference project and tech demo for the Bevy Engine. Try it out [in your browser on itch.io](https://janhohenheim.itch.io/foxtrot) :)

![image](https://github.com/user-attachments/assets/5050ba4e-45c1-4e73-b2ae-3d4aad3f4712)

<details><summary>Gameplay</summary>
    
https://github.com/user-attachments/assets/f648a085-4b3f-4fcb-9f54-e7d5249dc735
</details>

## Running the demo locally

Everything works out of the box as you would expect!
You can just clone the repo and do
```sh
cargo run
```
or, for users of the [Bevy CLI alpha](https://github.com/TheBevyFlock/bevy_cli):
```sh
bevy run
```

Web builds are as easy as
```sh
bevy run web
```


### (Optional) Hotpatching

All systems in Foxtrot are hotpatched, meaning that they can be edited at runtime and the changed will be patched live into the running executable.
To set your system up for this, see the documentation at [bevy_simple_subsecond_system](https://github.com/TheBevyFlock/bevy_simple_subsecond_system).
Once you're done, you can run
```sh
dx serve --hotpatched
```

## Motivation

Foxtrot is primarily a showcase of how to implement things that *I* care about. These are primarily:
- First person movement
- Grabbing and throwing objects
- Branching dialog
- Navigation
- Basic menus where accessibility features can live
- Debugging views
- Hotpatching
- Builds for desktop and web

There is intentionally no actual gameplay or goal in the demo to avoid showcasing too specific solutions. This is also the reason why 
there is no quest system or save states. These should, in my opinion, be highly specific to your use-case.

This is a big codebase, and it is not meant to be used as a generic template. Rather, treat this as a kind of mega-example
for how a real-life Bevy project might look like. Or take the code for a specific feature as inspiration for when you want to implement it yourself.

The code is very opinionated by design. That said, the coding style tries to follow
the one used in [Bevy New 2D](https://github.com/TheBevyFlock/bevy_new_2d) as close as possible.

## Organization

Foxtrot was generated using the [Bevy New 2D](https://github.com/TheBevyFlock/bevy_new_2d) template.
Check out its [documentation](https://github.com/TheBevyFlock/bevy_new_2d/blob/main/README.md) to understand the basis employed in this project.

## Level Editing

Foxtrot uses [TrenchBroom](https://trenchbroom.github.io/) as a level editor by integrating it with Bevy via [bevy_trenchbroom](https://github.com/Noxmore/bevy_trenchbroom).
To edit the levels, download the [latest TrenchBroom release](https://github.com/TrenchBroom/TrenchBroom/releases/latest) and run Foxtrot once.
You should see a message in your logs saying that some TrenchBroom config was written. After that, you can simply open `assets/maps/volta_i/volta_i.map` in TrenchBroom
to edit the main map.