#[allow(clippy::wildcard_imports)]
use crate::data::io::*;

pub struct Decision {
	pub prompt:           &'static str,
	pub possible_actions: &'static [Action],
	pub cancel_choice:    Answer
}
pub struct Action {
	pub answer:    Answer,
	pub action_fn: fn()
}
pub struct Answer {
	pub key:         &'static str,
	pub choice_text: &'static str
}

impl Answer {
	pub const fn cancel_answer() -> Self {
		Answer {
			key:         "C",
			choice_text: "Cancel, go back"
		}
	}

	pub const fn exit_answer() -> Self {
		Answer {
			key:         "E",
			choice_text: "Exit"
		}
	}
}
impl From<(&'static str, &'static str)> for Answer {
	fn from((key, choice_text): (&'static str, &'static str)) -> Self {
		Answer { key, choice_text }
	}
}

impl From<(Answer, fn())> for Action {
	fn from((answer, action_fn): (Answer, fn())) -> Self { Action { answer, action_fn } }
}

impl Decision {
	pub fn ask_continue(&self) -> bool {
		println!();
		println!("{}", self.prompt);
		for action_answer in self.possible_actions.iter().map(|action| &action.answer) {
			println!(" - [{}] {}", action_answer.key, action_answer.choice_text);
		}
		println!(
			" - [{}] {}",
			self.cancel_choice.key, self.cancel_choice.choice_text
		);
		let chosen_action = 'input_loop: loop {
			let reply = get_reply().to_uppercase();
			for action in self.possible_actions {
				if reply.contains(action.answer.key) {
					break 'input_loop action;
				}
			}
			if reply.contains(self.cancel_choice.key) {
				return false;
			}
			println!("You must use one of the key letters above marked in '[]' Try again.");
		};
		println!();
		(chosen_action.action_fn)();
		true
	}
	pub fn ask_continue(&self) -> bool {
		println!();
		println!("{}", self.prompt);
		for action_answer in self.possible_actions.iter().map(|action| &action.answer) {
			println!(" - [{}] {}", action_answer.key, action_answer.choice_text);
		}
		println!(
			" - [{}] {}",
			self.cancel_choice.key, self.cancel_choice.choice_text
		);
		let chosen_action = 'input_loop: loop {
			let reply = get_reply().to_uppercase();
			for action in self.possible_actions {
				if reply.contains(action.answer.key) {
					break 'input_loop action;
				}
			}
			if reply.contains(self.cancel_choice.key) {
				return false;
			}
			println!("You must use one of the key letters above marked in '[]' Try again.");
		};
		println!();
		(chosen_action.action_fn)();
		true
	}
}

impl Default for Decision {
	fn default() -> Self {
		Self {
			prompt:           "What do you want to do?",
			possible_actions: Default::default(),
			cancel_choice:    Answer::cancel_answer()
		}
	}
}
