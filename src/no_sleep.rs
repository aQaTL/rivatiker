use std::sync::mpsc::{channel, RecvTimeoutError, Sender};
use std::time::Duration;
use winapi::um::winbase::*;
use winapi::um::winnt::*;

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum State {
	Default,
	NoSystemSleep,
	NoMonitorSleep,
	AwayMode,
}

pub fn set_state(state: State) {
	let new_state = match state {
		State::Default => ES_CONTINUOUS,
		State::NoSystemSleep => ES_CONTINUOUS | ES_SYSTEM_REQUIRED,
		State::NoMonitorSleep => ES_CONTINUOUS | ES_DISPLAY_REQUIRED,
		State::AwayMode => ES_CONTINUOUS | ES_AWAYMODE_REQUIRED,
	};

	unsafe {
		SetThreadExecutionState(new_state);
	}
}

pub fn start_state_setter(state: State) -> Sender<State> {
	let (sender, receiver) = channel();

	std::thread::spawn(move || {
		let mut state = state;
		loop {
			match receiver.recv_timeout(Duration::from_secs(60)) {
				Err(RecvTimeoutError::Timeout) => {
					set_state(state);
				}
				Ok(new_state) => {
					println!("Setting {:?}", new_state);
					set_state(new_state);
					state = new_state;
				}
				Err(RecvTimeoutError::Disconnected) => {
					set_state(State::Default);
					break;
				}
			}
		}
	});

	sender
}
