use iced::{
    Color, Element, Length, Point, Rectangle, Renderer, Theme,
    mouse::Cursor,
    widget::{
        Canvas,
        canvas::{self, Frame, Geometry, Path},
    },
};
use lib_simulation::{Animal, Food, Simulation};

use crate::Message;

pub struct SimulationCanvas<'a> {
    pub simulation: &'a Simulation,
}

impl<'a> canvas::Program<Message> for SimulationCanvas<'a> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        for food in self.simulation.world().foods() {
            draw_food(&mut frame, food, bounds.size());
        }

        for animal in self.simulation.world().animals() {
            draw_animal(&mut frame, animal, bounds.size());
        }

        vec![frame.into_geometry()]
    }
}

pub fn simulation_canvas(simulation: &Simulation) -> Element<'_, Message> {
    Canvas::new(SimulationCanvas { simulation })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn draw_animal(frame: &mut Frame, animal: &Animal, bounds_size: iced::Size) {
    let position = Point::new(
        animal.position().x * bounds_size.width,
        animal.position().y * bounds_size.height,
    );

    frame.with_save(|frame| {
        frame.translate(iced::Vector::new(position.x, position.y));
        frame.rotate(animal.rotation().angle());

        let path = Path::new(|builder| {
            builder.move_to(Point::new(0.0, 8.0));
            builder.line_to(Point::new(5.0, -6.0));
            builder.line_to(Point::new(-5.0, -6.0));
            builder.close();
        });

        frame.fill(&path, Color::from_rgb(0.9, 0.3, 0.2));
    });
}

fn draw_food(frame: &mut Frame, food: &Food, bounds_size: iced::Size) {
    let position = Point::new(
        food.position().x * bounds_size.width,
        food.position().y * bounds_size.height,
    );

    let circle = Path::circle(position, 2.);
    frame.fill(&circle, Color::from_rgb(0., 1., 0.));
}
