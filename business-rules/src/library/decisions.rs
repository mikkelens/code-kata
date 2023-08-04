use std::sync::Arc;

use crate::library::io::get_reply;

pub struct Decision<F: 'static> {
	pub prompt:           Arc<str>,
	pub possible_choices: Vec<Choice<F>>,
	pub cancel_answer:    Answer
}
pub struct Choice<F> {
	pub answer: Answer,
	pub value:  F
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

impl<T> From<(Answer, T)> for Choice<T> {
	fn from((answer, action_fn): (Answer, T)) -> Self {
		Choice {
			answer,
			value: action_fn
		}
	}
}

impl<T> Decision<T>
where
	T: Clone
{
	pub fn run_prompt(&self) -> Option<T> {
		println!();
		println!("{}", self.prompt);
		for action_answer in self.possible_choices.iter().map(|action| &action.answer) {
			println!(" - [{}] {}", action_answer.key, action_answer.choice_text);
		}
		println!(
			" - [{}] {}",
			self.cancel_answer.key, self.cancel_answer.choice_text
		);
		let chosen_action = 'input_loop: loop {
			let reply = get_reply().to_uppercase();
			for action in &self.possible_choices {
				if reply.contains(&action.answer.key.to_uppercase()) {
					break 'input_loop action;
				}
			}
			if reply.contains(self.cancel_answer.key) {
				return None;
			}
			println!("You must use one of the key letters above marked in '[]' Try again.");
		};
		Some(chosen_action.value.clone())
	}
}

impl<T> Default for Decision<T> {
	fn default() -> Self {
		Self {
			prompt:           "What do you want to do?".into(),
			possible_choices: Vec::default(),
			cancel_answer:    Answer::cancel_answer()
		}
	}
}
