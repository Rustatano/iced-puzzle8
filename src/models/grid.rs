use iced::Center;
use iced::widget::{Column, button, column, row, text};

use super::message::Message;

pub struct Grid {
    pub grid_values: Vec<Vec<i32>>,
    pub counter_value: i32,
}

impl Grid {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.counter_value += 1;
            }
            Message::Decrement => {
                self.counter_value -= 1;
            }
        }
    }

    pub fn view(&self) -> Column<Message> {
        column![
            row![
                text(self.grid_values[0][0]).size(30),
                text(self.grid_values[0][1]).size(30),
                text(self.grid_values[0][2]).size(30),
            ],
            row![
                text(self.grid_values[1][0]).size(30),
                text(self.grid_values[1][1]).size(30),
                text(self.grid_values[1][2]).size(30),
            ],
            row![
                text(self.grid_values[2][0]).size(30),
                text(self.grid_values[2][1]).size(30),
                text(self.grid_values[2][2]).size(30),
            ],
            button("Increment").on_press(Message::Increment),
            text(self.counter_value).size(50),
            button("Decrement").on_press(Message::Decrement),
        ]
        .padding(20)
        .align_x(Center)
    }
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            grid_values: vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]],
            counter_value: 0,
        }
    }
}
