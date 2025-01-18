use iced::Alignment::Center;
use iced::Length::Fill;
use iced::widget::{
    button, center, column, container, horizontal_space, row, scrollable, slider, text,
};
use iced::{Element, Size, Task, Theme, widget};
use rand::seq::SliceRandom;

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
    ChangeGridSize(u8),
}

struct App {
    grid_values: Vec<Vec<u8>>,
    theme: Theme,
    button_heigth: f32,
    button_width: f32,
    grid_element_size: f32,
    grid_size: u8,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let theme = Theme::Moonfly;
        (
            Self {
                grid_values: generate(3),
                theme,
                button_heigth: 50.0,
                button_width: 200.0,
                grid_element_size: 80.0,
                grid_size: 3,
            },
            widget::focus_next(),
        )
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Generate => {
                self.grid_values = generate(self.grid_size);
            }

            Message::Solve => self.solve(),
            Message::ChangeGridSize(size) => {
                self.grid_size = size;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let control_panel = column![
            slider(2..=4, self.grid_size, Message::ChangeGridSize)
                .step(1)
                .default(3)
                .width(self.button_width),
            text("grid size: ".to_owned() + &self.grid_size.to_string()).size(20),
            button(text("Generate").size(30))
                .on_press(Message::Generate)
                .width(self.button_width)
                .height(self.button_heigth),
            button(text("Solve").size(30))
                .on_press(Message::Solve)
                .width(self.button_width)
                .height(self.button_heigth),
            scrollable(column![
                text("step 1").size(50),
                text("step 2").size(50),
                text("step 3").size(50),
            ])
            .height(100)
            .width(self.button_width),
        ]
        .spacing(30)
        .padding(20);

        let grid = container(column![
            row![
                text(self.grid_values[0][0])
                    .size(self.grid_element_size)
                    .width(self.grid_element_size)
                    .align_x(Center)
                    .align_y(Center),
                text(self.grid_values[0][1])
                    .size(self.grid_element_size)
                    .width(self.grid_element_size)
                    .align_x(Center)
                    .align_y(Center),
                text(self.grid_values[0][2])
                    .size(self.grid_element_size)
                    .width(self.grid_element_size)
                    .align_x(Center)
                    .align_y(Center),
            ],
            row![
                text(self.grid_values[1][0])
                    .size(self.grid_element_size)
                    .width(self.grid_element_size)
                    .align_x(Center)
                    .align_y(Center),
                text(self.grid_values[1][1])
                    .size(self.grid_element_size)
                    .width(self.grid_element_size)
                    .align_x(Center)
                    .align_y(Center),
                text(self.grid_values[1][2])
                    .size(self.grid_element_size)
                    .width(self.grid_element_size)
                    .align_x(Center)
                    .align_y(Center),
            ],
            row![
                text(self.grid_values[2][0])
                    .size(self.grid_element_size)
                    .width(self.grid_element_size)
                    .align_x(Center)
                    .align_y(Center),
                text(self.grid_values[2][1])
                    .size(self.grid_element_size)
                    .width(self.grid_element_size)
                    .align_x(Center)
                    .align_y(Center),
                text(self.grid_values[2][2])
                    .size(self.grid_element_size)
                    .width(self.grid_element_size)
                    .align_x(Center)
                    .align_y(Center),
            ]
        ])
        .padding(20)
        .style(container::dark);

        center(row![control_panel, horizontal_space().width(Fill), grid,].spacing(20)).into()
    }

    fn solve(&mut self) {
        println!("solve");
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

fn generate(grid_size: u8) -> Vec<Vec<u8>> {
    let mut rng = rand::thread_rng();
    let mut arr = vec![];
    for i in 0..grid_size * grid_size {
        arr.push(i);
    }
    let mut out = vec![];
    arr.shuffle(&mut rng);

    for i in 0..grid_size {
        out.push(vec![]);
        for j in 0..grid_size {
            out[i as usize].push(0);
            out[i as usize][j as usize] = arr[(i * grid_size + j) as usize];
        }
    }

    out
}
