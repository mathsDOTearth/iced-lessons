# iced for Rust — Lesson 01 — Opening a Window

This series introduces graphical simulation development in **Rust** using the **iced** GUI toolkit. The material is inspired in part by the [Iced Book](https://book.iced.rs/), but is tailored specifically for learners intending to build interactive and visually rich simulations.

The purpose of Lesson 01 is straightforward: **successfully open a window** using iced 0.13.1 and understand the structure of an iced application. Although deceptively simple, this foundation is essential for later lessons, where timing loops, drawing, input handling, and simulation logic will be layered on top.

---

## 1. Preparing the Project

In a new Rust project, begin by adding iced to your `Cargo.toml`:

```toml
[dependencies]
iced = "0.13.1"
```

The default feature set includes support for GPU-accelerated rendering (via `wgpu`) and a suitable text rendering pipeline, so no further configuration is required for this introductory lesson.

---

## 2. Understanding the Iced Application Structure

Iced adopts a declarative and state-driven approach to GUI development. You describe:

* **State** the data your application holds
* **Update logic** how the state changes in response to messages
* **View logic** how the state is rendered to the user

This triad resembles **Elm**, and more broadly follows functional-reactive design principles. Even though Lesson 01 has almost no behaviour, setting up these components from the outset will support future simulation features such as particle updates, timers, and user controls.

---

## 3. Minimal Example: Creating Your First Window

Below is the Lesson 01 Rust source code. Save this in `src/main.rs`:

```rust
use iced::{
    widget,
    window::{self, Position},
    Element, Size, Task, Theme,
};

/// Messages represent events that can affect the application’s state.
/// 
/// In this introductory lesson there are no interactive features,
/// but setting up the enumeration now prepares the structure for
/// later simulations and user input.
#[derive(Debug, Clone, Copy)]
enum Message {
    // Placeholder for future events
}

/// This struct holds the internal application state.
/// 
/// The first lesson only displays a window with a line of text,
/// so there is no data to store yet. Later lessons will introduce
/// simulation data, timers, and user-controlled settings.
#[derive(Default)]
struct SimulationWindow;

impl SimulationWindow {
    /// Creates the initial application state.
    fn new() -> Self {
        Self
    }

    /// The update method is called whenever the programme receives
    /// a `Message`. This is where state transitions occur.
    ///
    /// At this stage nothing changes, so we return a no-operation
    /// task to indicate there is nothing to do.
    fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    /// The view method describes how the user interface should
    /// appear, based on the current state.
    ///
    /// Here we simply draw a line of text so learners can confirm
    /// that their windowing setup is functioning correctly.
    fn view(&self) -> Element<'_, Message> {
        widget::text("Window initialised successfully.")
            .size(32)
            .into()
    }
}

pub fn main() -> Result<(), iced::Error> {
    // The `application` function constructs a native desktop
    // programme using the iced framework. It binds together:
    //
    // - a window title
    // - the update function (state transitions)
    // - the view function (rendering)
    //
    // This functional structure keeps simulation logic and
    // presentation cleanly separated.
    iced::application(
        "Simulation Tutorial – Lesson 1",
        SimulationWindow::update,
        SimulationWindow::view,
    )
    // Select a theme for the demonstration window. The dark theme
    // provides a neutral starting point for graphical simulations.
    .theme(|_| Theme::Dark)

    // Configure the initial window: centre it on screen, choose
    // a comfortable default size, and allow resizing.
    .window(window::Settings {
        position: Position::Centered,
        size: Size::new(800.0, 600.0),
        resizable: true,
        ..Default::default()
    })

    // `run_with` allows custom construction of both the initial
    // application state and the initial task (none required here).
    .run_with(|| (SimulationWindow::new(), Task::none()))
}
```

---

## 4. How the Example Works

Although small, this programme demonstrates several essential iced concepts.

### 4.1 Application State

`SimulationWindow` acts as the persistent state object.
Even though it is empty now, future lessons may introduce things like:

* simulation parameters
* positions and velocities
* render mode toggles
* user configuration

Keeping the state encapsulated in a dedicated struct helps maintain clarity as your simulation grows.

### 4.2 Messages

The `Message` enum defines the events the application can receive.
In iced:

* user interactions (mouse, keyboard)
* timers and subscriptions
* asynchronous tasks

…all flow through this enumeration.
In Lesson 01 it contains no variants, but it will soon become central to simulation updates.

### 4.3 Update Function

`update(&mut self, message) -> Task<Message>` is where all state transitions occur.
Returning `Task::none()` signals that nothing happens yet.
Future lessons will demonstrate:

* responding to user input
* advancing a simulation step at fixed intervals
* loading simulation data asynchronously

### 4.4 View Function

The view is entirely declarative: you describe what the interface *should* look like, and iced renders it automatically.
Here the interface is a single line of text.

As simulations become more complex, the view will include:

* custom canvas rendering
* controls and side panels
* simulation statistics and graphs
* multiple panes or tabs

### 4.5 Launching the Application

The call to `iced::application` constructs the GUI programme.
Builder-style configuration then sets the theme and window properties before launching it using `.run_with`.

---

## 5. Running the Lesson

From the root of your project:

```bash
cargo run
```

A window should appear centred on your screen, displaying:

```
Window initialised successfully.
```

If the window opens correctly, your iced environment is working and you are ready to proceed.

