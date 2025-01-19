use std::cmp::Ordering;
use std::collections::BinaryHeap;

use iced::Alignment::Center;
use iced::Length::Fill;
use iced::widget::{
    button, center, column, container, horizontal_space, row, scrollable, slider, text,
};
use iced::{Element, Size, Task, Theme, widget};
use rand::Rng;

const FINAL_MATRIX: [[u8; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 0]];

pub fn main() -> iced::Result {
    iced::application("number8", App::update, App::view)
        .theme(App::theme)
        .window_size(Size {
            width: 1000.0,
            height: 500.0,
        })
        .run_with(App::new)
}

#[derive(Debug, Clone)]
pub struct Node {
    parent: Box<Option<Node>>,
    matrix: Vec<Vec<u8>>,
    empty_tile: [i8; 2],
    cost: u32,
    level: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.cost < other.cost {
            return Ordering::Greater;
        } else if self.cost > other.cost {
            return Ordering::Less;
        }
        Ordering::Equal
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        (self.level, &self.level) == (other.level, &other.level)
    }
}

impl Eq for Node {}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Generate,
    Solve,
    ChangeGridSize(u8),
}

struct App {
    matrix: Vec<Vec<u8>>,
    theme: Theme,
    button_heigth: f32,
    button_width: f32,
    matrix_element_size: f32,
    matrix_size: u8,
    steps: Vec<Vec<Vec<u8>>>,
    str_steps: Vec<String>,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let theme = Theme::Moonfly;
        (
            Self {
                matrix: generate(3),
                theme,
                button_heigth: 50.0,
                button_width: 200.0,
                matrix_element_size: 80.0,
                matrix_size: 3,
                steps: vec![],
                str_steps: vec![],
            },
            widget::focus_next(),
        )
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Generate => {
                self.matrix = generate(self.matrix_size);
                self.str_steps = vec![];
            }

            Message::Solve => self.solve(),

            Message::ChangeGridSize(size) => {
                self.matrix_size = size;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let control_panel = column![
            slider(2..=4, self.matrix_size, Message::ChangeGridSize)
                .step(1)
                .default(3)
                .width(self.button_width),
            text("matrix size: ".to_owned() + &self.matrix_size.to_string()).size(20),
            button(text("Generate").size(30))
                .on_press(Message::Generate)
                .width(self.button_width)
                .height(self.button_heigth),
            button(text("Solve").size(30))
                .on_press(Message::Solve)
                .width(self.button_width)
                .height(self.button_heigth),
            scrollable(column(self.str_steps.iter().map(|step| text(step).size(50).into())))
                .height(100)
                .width(self.button_width),
        ]
        .spacing(30)
        .padding(20)
        .height(Fill);

        let matrix = container(column![
            row![
                text(self.matrix[0][0])
                    .size(self.matrix_element_size)
                    .width(self.matrix_element_size)
                    .align_x(Center)
                    .align_y(Center),
                text(self.matrix[0][1])
                    .size(self.matrix_element_size)
                    .width(self.matrix_element_size)
                    .align_x(Center)
                    .align_y(Center),
                text(self.matrix[0][2])
                    .size(self.matrix_element_size)
                    .width(self.matrix_element_size)
                    .align_x(Center)
                    .align_y(Center),
            ],
            row![
                text(self.matrix[1][0])
                    .size(self.matrix_element_size)
                    .width(self.matrix_element_size)
                    .align_x(Center)
                    .align_y(Center),
                text(self.matrix[1][1])
                    .size(self.matrix_element_size)
                    .width(self.matrix_element_size)
                    .align_x(Center)
                    .align_y(Center),
                text(self.matrix[1][2])
                    .size(self.matrix_element_size)
                    .width(self.matrix_element_size)
                    .align_x(Center)
                    .align_y(Center),
            ],
            row![
                text(self.matrix[2][0])
                    .size(self.matrix_element_size)
                    .width(self.matrix_element_size)
                    .align_x(Center)
                    .align_y(Center),
                text(self.matrix[2][1])
                    .size(self.matrix_element_size)
                    .width(self.matrix_element_size)
                    .align_x(Center)
                    .align_y(Center),
                text(self.matrix[2][2])
                    .size(self.matrix_element_size)
                    .width(self.matrix_element_size)
                    .align_x(Center)
                    .align_y(Center),
            ]
        ])
        .padding(20)
        .style(container::dark)
        .height(Fill);

        center(row![control_panel, horizontal_space().width(Fill), matrix,].spacing(20)).into()
    }

    fn solve(&mut self) {
        let row = [1, 0, -1, 0];
        let col = [0, -1, 0, 1];

        let mut heap = BinaryHeap::<Node>::new();

        let cost = calculate_cost(&self.matrix, self.matrix_size);

        let mut empty_tile = [0, 0];
        for y in 0..self.matrix_size {
            for x in 0..self.matrix_size {
                if self.matrix[y as usize][x as usize] == 0 {
                    empty_tile[0] = y as i8;
                    empty_tile[1] = x as i8;
                }
            }
        }

        let root = Node {
            parent: Box::new(None),
            matrix: self.matrix.clone(),
            empty_tile,
            cost,
            level: 0,
        };

        heap.push(root);

        while !heap.is_empty() {
            let minimum = heap.pop().unwrap();

            if minimum.cost == 0 {
                self.print_path(&minimum);
                self.print_matrix(&minimum.matrix);
                self.str_steps.reverse();
                println!("-------");
                return;
            } else if minimum.level > 31 {
                println!("Unsolvable");
                println!("-------");
                return;
            }

            for i in 0..4 {
                let new_empty_tile = [
                    minimum.empty_tile[0] + row[i],
                    minimum.empty_tile[1] + col[i],
                ];

                if self.is_safe(new_empty_tile[0], new_empty_tile[1]) {
                    let child = new_node(&minimum, new_empty_tile, self.matrix_size);

                    heap.push(child);
                }
            }
        }
    }

    fn print_path(&mut self, root: &Node) {
        match *root.parent.clone() {
            Some(node) => {
                self.str_steps.push(format!("step {}", root.level + 1));
                self.print_path(&node);
                self.print_matrix(&node.matrix);
            }
            None => self.str_steps.push(format!("step {}", root.level + 1)),
        };
    }

    fn print_matrix(&mut self, matrix: &Vec<Vec<u8>>) {
        for y in 0..self.matrix_size {
            for x in 0..self.matrix_size {
                self.steps.push(matrix.clone());
                print!("{} ", matrix[y as usize][x as usize]);
            }
            println!();
        }
        println!("\n");
    }

    fn is_safe(&self, x: i8, y: i8) -> bool {
        x >= 0 && x < self.matrix_size as i8 && y >= 0 && y < self.matrix_size as i8
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

fn generate(matrix_size: u8) -> Vec<Vec<u8>> {
    let mut out = vec![
        vec![FINAL_MATRIX[0][0], FINAL_MATRIX[0][1], FINAL_MATRIX[0][2]],
        vec![FINAL_MATRIX[1][0], FINAL_MATRIX[1][1], FINAL_MATRIX[1][2]],
        vec![FINAL_MATRIX[2][0], FINAL_MATRIX[2][1], FINAL_MATRIX[2][2]],
    ];

    for _ in 1..rand::thread_rng().gen_range(3..100) {
        let mut empty_tile = [0, 0];
        for y in 0..matrix_size {
            for x in 0..matrix_size {
                if out[y as usize][x as usize] == 0 {
                    empty_tile = [x as i8, y as i8];
                }
            }
        }
        let mut moves = vec![];
        if empty_tile[0] > 0 {
            moves.push([-1, 0]);
        }
        if empty_tile[0] < 2 {
            moves.push([1, 0]);
        }
        if empty_tile[1] > 0 {
            moves.push([0, -1]);
        }
        if empty_tile[1] < 2 {
            moves.push([0, 1]);
        }
        let mov = moves[rand::thread_rng().gen_range(0..moves.len())];
        let tmp = out[empty_tile[1] as usize][empty_tile[0] as usize].clone();
        out[empty_tile[1] as usize][empty_tile[0] as usize] =
            out[(empty_tile[1] + mov[1]) as usize][(empty_tile[0] + mov[0]) as usize];
        out[(empty_tile[1] + mov[1]) as usize][(empty_tile[0] + mov[0]) as usize] = tmp;
    }

    out
}

fn calculate_cost(matrix: &Vec<Vec<u8>>, matrix_size: u8) -> u32 {
    let mut cost = 0;
    for i in 0..matrix_size {
        for j in 0..matrix_size {
            if matrix[i as usize][j as usize] != 0
                && matrix[i as usize][j as usize] != FINAL_MATRIX[i as usize][j as usize]
            {
                cost += 1;
            }
        }
    }

    cost
}

fn new_node(parent: &Node, new_empty_tile: [i8; 2], matrix_size: u8) -> Node {
    let mut new_matrix = parent.matrix.clone();

    let x1 = parent.empty_tile[0] as usize;
    let y1 = parent.empty_tile[1] as usize;
    let x2 = new_empty_tile[0] as usize;
    let y2 = new_empty_tile[1] as usize;
    let tmp = new_matrix[x1][y1];
    new_matrix[x1][y1] = new_matrix[x2][y2];
    new_matrix[x2][y2] = tmp;

    let cost = calculate_cost(&new_matrix, matrix_size);

    Node {
        parent: Box::new(Some(parent.clone())),
        matrix: new_matrix,
        empty_tile: new_empty_tile,
        cost,
        level: parent.level + 1,
    }
}
