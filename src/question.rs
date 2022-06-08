use std::io::stdin;

/// It is a class that handle validation of questions in the terminal
pub struct Question {
	message: String,
	invalids: Vec<String>,
	invalids_contents: Vec<String>
}

impl Question {
	/// Creates a new instance of the question
	pub fn new(msg: &str) -> Self {
		
		Self {
			message: msg.to_string(),
			invalids: Vec::new(),
			invalids_contents: Vec::new()
		}
	}

	/// Sets the question configuration to refuse null answers
	pub fn not_null(&mut self) -> &mut Self {
		self.invalids.push("".to_string());

		self
	}

	/// Adds an invalid answer
	pub fn not_valid(&mut self, value: String) -> &mut Self {
		self.invalids.push(value);

		self
	}

	/// Adds a restriction for the answer
	pub fn not_containing(&mut self, value: String) -> &mut Self {
		self.invalids_contents.push(value);

		self
	}

	/// Asks the configured question and returns a value
	pub fn ask(&self) -> String {
		let mut answer = String::new();
		let mut answered = false;

		while !answered {
			println!("{}", self.message);

			stdin().read_line(&mut answer).expect("Error reading the answer");
			answer = answer.trim().to_string();

			answered = true;

			for invalid in &self.invalids {
				if answer==*invalid {
						answered = false;
				}
			}

			for inv_cont in &self.invalids_contents {
				if answer.contains(inv_cont) {
					answered = false;

					println!("Cannot have \"{}\" in the answer!", inv_cont);
				}
			}

			if !answered {
				println!("Invalid answer!");
			}
		}

		answer
	}
}