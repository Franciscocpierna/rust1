use iced::widget::{button, Button,Column,Container, Row, Text};
use iced::{executor, Application, Command, Element, Length, Settings, Theme};

struct Calculator{
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message{
    Increment,
    Decrement,
}
impl Application for Calculator{
   type Executor = executor::Default;
   type Message = Message; 
   type Flags = ();
   type Theme = Theme;
fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
    (Self { value: 0}, Command::none())
}

fn title(&self) -> String{
    String::from("Calculadora") 
}
fn update(&mut self, message: Self::Message) -> Command<Self::Message>{
    match message{
        Message::Increment => self.value += 1,
        Message::Decrement => self.value -= 1,
    }
    Command::none()
}
fn view(&self) -> Element<Self::Message>{
    let increment_button = Button::new(Text::new("+")).on_press(Message::Increment);
    let decrement_button = Button::new(Text::new("-")).on_press(Message::Decrement);

    let content  = Column::new()
           .push(Text::new(format!("Valor atual: {}", self.value)).size(50))
           .push(Row::new().spacing(20).push(increment_button).push(decrement_button))
           .spacing(20)
           .padding(20)
           .align_items(iced::Alignment::Center);

        Container::new(content)
                 .width(Length::Fill)
                 .height(Length::Fill)
                 .center_x()
                 .center_y()
                 .into()
} 
}



fn main() -> iced::Result {
    Calculator::run(Settings::default())
}
