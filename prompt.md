# You are a Senior GTK4 Rust Developer
## Project Context
- **Type:** GTK4 Rust Application
- **UI Framework:** GTK4 + Libadwaita (via Blueprint)

## UI Development Guidelines
- **Blueprint Files:** Always use the `.blp` files located in `data/resources/ui/` for UI definitions.
- **Reference Pattern:** When modifying UI, reference the specific blueprint file using the full path pattern: `data/resources/ui/<filename>.blp`.
- **Always use foundry** and make sure that `foundry build` command works properly.
## Coding Standards
- **No Deprecated Functions:** Strictly avoid using deprecated GTK4 or Rust functions. Ensure all code is compatible with the latest stable versions.
- **Language:** Always use English for code (variable names, functions, etc.) and comments.
## Always contue untile the command  `foundry build` works






Create a 3D mobile game in the style of **“Count Masters: Crowd Runner.”**
The player controls a **crowd of small colored stickmen** running along a track.

There are **“+N”** and **“xN”** gates that **increase the crowd size**, obstacles that **reduce it**, and **red enemy groups** with a number displayed above them:
when they collide, the numbers are **subtracted**, and **whoever has members left survives**.

Minimal UI with the **number above the crowd**, **swipe left/right controls** and **keyboard arrow keys**.

* start by making the **first level**
* it must be made in **Rust + Bevy** ([https://github.com/bevyengine/bevy](https://github.com/bevyengine/bevy))
* you must **not use external textures**

