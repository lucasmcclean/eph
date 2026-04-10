# eph

A simple, transparent CLI task management tool.

> Focus on *doing*, not *managing*

eph follows a two-point philosophy:

- *Tasks* are **simple** & **focused**
- The *tool* is **transparent** & **fluid**

## Distinct Features

eph differs from the numerous other CLI task managers in a few ways:

1. GitHub-backed CRDT syncing

   - Ensures multiple devices/users can modify independently and be assured of
     convergence on sync

1. Keyboard driven (Neovim inspired) and fluid interface

   - The design is opinionated but rewards leaning into the design with a
     frictionless workflow

1. Preference for fewer features

   - Simplicity implies a lower likelihood of bugs and a more consistent (and
     hopefully pleasant) experience

## Implementation

The app has three primary layers:

- The interaction layer (`commands/`)

  - Handles user interaction via the CLI or TUI

- The application layer (`app/`, `task/`)

  - Serves as the logic layer between the user and storage

- The storage layer (`storage/`)

  - Interacts with the file system to store and load tasks

This is an extremely common approach because it's clean and simple. Since eph is
not architecturally complex, this is an effective way to keep the data flow
modular while avoiding unnecessary complexity.

The primary tradeoffs of the current implementation are:

- We read and write all tasks for all actions except for the addition of a new
  task.

  - This does increase resource usage, but task lists would have to become
    unrealistically large for the performance difference to be perceivable. And
    the simplicity of this design makes desiging a *correct* and *maintainable*
    implementation far easier.

- Using the stored task list as the only source of truth.

  - In a similar vein to the previous tradeoff, we sacrifice a small amount of
    performance (in more common file system interactions) in exchange for
    simplicity. In this instance, we don't have to balance an in-memory and
    on-disk version of the task list.
