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
