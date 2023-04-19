use iced::{
    button, scrollable, Application, Button, Column, Command, Container, Element, Image, Length,
    Row, Scrollable, Text,
};
use nodium_controller::NodiumController;

pub struct NodiumAppIced {
  controller: NodiumController,
  scroll: scrollable::State,
  buttons: Vec<button::State>,
  theme: Theme,
}

impl Application for NodiumAppIced {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = ();

    fn new(_flags: ()) -> (NodiumAppIced, Command<Self::Message>) {
        let mut controller = NodiumController::new();

        controller.add_icon(
            "/home/roggen/Documents/GitHub/nodium/libs/style/icons/file-tray-stacked.svg"
                .to_string(),
        );
        controller.add_icon(
            "/home/roggen/Documents/GitHub/nodium/libs/style/src/iced_renderer.rs".to_string(),
        );
        controller.add_icon(
            "/home/roggen/Documents/GitHub/nodium/libs/style/icons/options.svg".to_string(),
        );

        (
            NodiumAppIced {
                controller: controller,
                scroll: scrollable::State::new(),
                buttons: vec![button::State::new(); 10],
                theme: Theme::dark(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Nodium")
    }

    fn update(
        &mut self,
        _message: Self::Message,
        _clipboard: &mut iced::Clipboard,
    ) -> Command<Self::Message> {
      return Command::none();
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let top_panel = Row::new()
            .push(Text::new("App Title").color(self.theme.primary))
            .push(Text::new("File"))
            .push(Text::new("Edit"))
            .push(Text::new("View"));

        let bottom_panel = Text::new("Status bar content");

        let left_panel_buttons = self
            .controller
            .icons()
            .iter()
            .zip(self.buttons.iter_mut())
            .map(|(icon_path, state)| {
                let icon = Image::new(icon_path).width(Length::Units(30));
                Button::new(state, icon).on_press(()) // Replace '()' with your message type
            })
            .fold(Column::new(), |column, button| column.push(button));

        let left_panel = Scrollable::new(&mut self.scroll)
            .push(left_panel_buttons)
            .width(Length::Units(50));

        let right_panel = Text::new("Right sidebar content");

        let main_content = Text::new("Main content area");

        let center_panel = Column::new()
            .push(
                Container::new(main_content)
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
            .push(Row::new().push(left_panel).push(right_panel));

        let content = Column::new()
            .push(top_panel)
            .push(center_panel)
            .push(bottom_panel);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

// theme
