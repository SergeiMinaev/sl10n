#[cfg(test)]
mod tests {
    use super::*;
	use sl10n::define_l10n;
	use std::collections::HashMap;

	define_l10n! {
		Continue => {
			en: "Continue",
			ru: "Продолжить"
		},
		Greeting => {
			en: "Hello, {name1} and {name2}!",
			ru: "Привет, {name1} and {name2}!"
		},
		Farewell => {
			en: "Goodbye, {name}.",
			ru: "Пока, {name}."
		}
	}

    #[test]
	pub fn static_msg() {
		let msgs = Msgs::new();
		let msg_ru = msgs.msg(Msg::Continue, "ru");
		assert_eq!(msg_ru, "Продолжить");
	}

    #[test]
	pub fn dynamic_msg() {
		let msgs = Msgs::new();

		let mut params = HashMap::new();
		params.insert("name1", "Alice");
		params.insert("name2", "David");

		let msg_ru = msgs.dyn_msg(Msg::Greeting, "en", &params);
		assert_eq!(msg_ru, "Hello, Alice and David!");
	}

    #[test]
	pub fn custom_names() {
		define_l10n! {
			CustomMsgs, // struct name
			CustomMsg, // enum name
			Continue => {
				en: "Continue",
				ru: "Продолжить"
			},
		}

		let msgs = CustomMsgs::new();
		let msg_ru = msgs.msg(CustomMsg::Continue, "en");
		assert_eq!(msg_ru, "Continue");
	}
}
