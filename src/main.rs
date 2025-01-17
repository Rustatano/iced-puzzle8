use number8::models::counter::Counter;

pub fn main() -> iced::Result {
    iced::run("A cool counter", Counter::update, Counter::view)
}
