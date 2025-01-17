use number8::models::grid::Grid;

pub fn main() -> iced::Result {
    iced::run("A cool counter", Grid::update, Grid::view)
}
