use crate::style::TogglableButton;
use iced::*;
use std::sync::mpsc::Sender;

mod no_sleep;
#[cfg(test)]
mod tests;

fn main() {
	<Rivatiker as Application>::run(Settings {
		window: window::Settings {
			size: (450, 50),
			..Default::default()
		},
		..Default::default()
	});
}

struct Rivatiker {
	power_state: PowerState,
	state_sender: Sender<no_sleep::State>,
}

#[derive(Default)]
struct PowerState {
	button_states: PowerButtonStates,
}

#[derive(Default, Debug)]
struct PowerButtonStates {
	default: (button::State, style::TogglableButton),
	no_system_sleep: (button::State, style::TogglableButton),
	no_monitor_sleep: (button::State, style::TogglableButton),
}

#[derive(Debug, Clone)]
enum Message {
	PowerButtonState(no_sleep::State),
}

impl Application for Rivatiker {
	type Executor = executor::Default;
	type Message = Message;

	fn new() -> (Self, Command<Message>) {
		(
			Self {
				power_state: PowerState {
					button_states: PowerButtonStates {
						default: (Default::default(), TogglableButton::Toggled),
						..Default::default()
					},
					..Default::default()
				},
				state_sender: no_sleep::start_state_setter(no_sleep::State::Default),
			},
			Command::none(),
		)
	}

	fn title(&self) -> String {
		String::from(env!("CARGO_PKG_NAME"))
	}

	fn update(&mut self, message: Self::Message) -> Command<Message> {
		match message {
			Message::PowerButtonState(power_state) => {
				self.power_state.button_states.default.1 = TogglableButton::NotToggled;
				self.power_state.button_states.no_system_sleep.1 = TogglableButton::NotToggled;
				self.power_state.button_states.no_monitor_sleep.1 = TogglableButton::NotToggled;
				use no_sleep::State::*;
				match power_state {
					Default => self.power_state.button_states.default.1 = TogglableButton::Toggled,
					NoSystemSleep => {
						self.power_state.button_states.no_system_sleep.1 = TogglableButton::Toggled
					}
					NoMonitorSleep => {
						self.power_state.button_states.no_monitor_sleep.1 = TogglableButton::Toggled
					}
					_ => unimplemented!(),
				}
				if let Err(e) = self.state_sender.send(power_state) {
					println!("Error sending {:?}: {:?}", power_state, e);
				}
			}
		}
		Command::none()
	}

	fn subscription(&self) -> Subscription<Message> {
		Subscription::none()
	}

	fn view(&mut self) -> Element<Message> {
		Container::new(
			Row::new()
				.spacing(20)
				.push(
					Button::new(
						&mut self.power_state.button_states.default.0,
						Text::new("Off"),
					)
					.on_press(Message::PowerButtonState(no_sleep::State::Default))
					.style(self.power_state.button_states.default.1),
				)
				.push(
					Button::new(
						&mut self.power_state.button_states.no_system_sleep.0,
						Text::new("No system sleep"),
					)
					.on_press(Message::PowerButtonState(no_sleep::State::NoSystemSleep))
					.style(self.power_state.button_states.no_system_sleep.1),
				)
				.push(
					Button::new(
						&mut self.power_state.button_states.no_monitor_sleep.0,
						Text::new("No monitor sleep"),
					)
					.on_press(Message::PowerButtonState(no_sleep::State::NoMonitorSleep))
					.style(self.power_state.button_states.no_monitor_sleep.1),
				),
		)
		.width(Length::Fill)
		.height(Length::Fill)
		.center_x()
		.center_y()
		.into()
	}
}

mod style {
	use iced::widget::button;
	use iced::*;

	#[derive(Debug, Clone, Copy)]
	pub enum TogglableButton {
		Toggled,
		NotToggled,
	}

	impl Default for TogglableButton {
		fn default() -> Self {
			TogglableButton::NotToggled
		}
	}

	impl button::StyleSheet for TogglableButton {
		fn active(&self) -> button::Style {
			let style: Box<dyn button::StyleSheet> = Default::default();
			let mut style = style.active();
			if let TogglableButton::Toggled = self {
				style.background = Some(Background::Color(Color::from(crate::HexColor(0x8160dc))));
			}
			style
		}
	}
}

pub struct HexColor(u32);

impl From<HexColor> for Color {
	fn from(c: HexColor) -> Self {
		Color {
			r: (c.0 >> 8 * 2 & 0xff) as f32 / 255.0,
			g: (c.0 >> 8 * 1 & 0xff) as f32 / 255.0,
			b: (c.0 >> 8 * 0 & 0xff) as f32 / 255.0,
			a: 1.0,
		}
	}
}
