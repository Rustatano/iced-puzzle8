use iced::widget::{button, column, row, text};
use iced::{Center, Element, Padding, Size, Task, Theme, widget};

pub fn main() -> iced::Result {
    iced::application("number8", App::update, App::view)
        .theme(App::theme)
        .window_size(Size {
            width: 1000.0,
            height: 500.0,
        })
        .run_with(App::new)
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Generate,
    Solve,
}

struct App {
    grid_values: Vec<Vec<i32>>,
    theme: Theme,
    button_heigth: f32,
    button_width: f32,
    grid_element_size: f32,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let theme = Theme::Moonfly;
        (
            Self {
                grid_values: vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]],
                theme,
                button_heigth: 50.0,
                button_width: 200.0,
                grid_element_size: 80.0,
            },
            widget::focus_next(),
        )
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Generate => self.generate(),

            Message::Solve => self.solve(),
        }
    }

    fn view(&self) -> Element<Message> {
        let buttons = column![
            button(text("Generate").size(30))
                .on_press(Message::Generate)
                .width(self.button_width)
                .height(self.button_heigth),
            button(text("Solve").size(30))
                .on_press(Message::Solve)
                .width(self.button_width)
                .height(self.button_heigth),
        ]
        .spacing(30)
        .padding(Padding::new(20.0));

        let row1 = row![
            text(self.grid_values[0][0]).size(self.grid_element_size),
            text(self.grid_values[0][1]).size(self.grid_element_size),
            text(self.grid_values[0][2]).size(self.grid_element_size),
        ];

        let row2 = row![
            text(self.grid_values[1][0]).size(self.grid_element_size),
            text(self.grid_values[1][1]).size(self.grid_element_size),
            text(self.grid_values[1][2]).size(self.grid_element_size),
        ];

        let row3 = row![
            text(self.grid_values[2][0]).size(self.grid_element_size),
            text(self.grid_values[2][1]).size(self.grid_element_size),
            text(self.grid_values[2][2]).size(self.grid_element_size),
        ];

        let grid = column![row1, row2, row3];

        row![buttons, grid].spacing(200).into()
    }

    fn generate(&self) {
        println!("generate");
    }

    fn solve(&self) {
        println!("solve");
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
