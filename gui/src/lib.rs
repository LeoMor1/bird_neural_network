mod simulation_canvas;

use std::time::Duration;

pub use self::simulation_canvas::*;

use iced::widget::column;
use iced::{Element, Subscription, Task, Theme, time};
use lib_simulation::Simulation;
use rand::rngs::ThreadRng;

pub enum Message {
    Tick,
}

pub struct SimApp {
    simulation: Simulation,
    rng: ThreadRng,
}

impl SimApp {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let simulation = Simulation::random(&mut rng);
        Self { simulation, rng }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                self.simulation.step(&mut self.rng);
            }
        }
        Task::none()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(16)).map(|_| Message::Tick)
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![simulation_canvas(&self.simulation)].into()
    }

    pub fn theme(&self) -> Theme {
        Theme::Dark
    }
}

impl Default for SimApp {
    fn default() -> Self {
        Self::new()
    }
}
