use std::collections::HashMap;
use sl10n::define_l10n;


define_l10n! {
    Greeting => {
        en: "Hello!",
        ru: "Привет!",
        es: "¡Hola!"
    },
    Farewell => {
        en: "Goodbye, {name}.",
        ru: "До свидания, {name}.",
        es: "Adiós, {name}."
    }
}

fn main() {
    let msgs = Msgs::new();
    let greeting = msgs.msg(Msg::Greeting, "en");

	let params = HashMap::from([("name", "Alice")]);
    let farewell = msgs.dyn_msg(Msg::Farewell, "es", &params);

	assert_eq!(greeting, "Hello!");
	assert_eq!(farewell, "Adiós, Alice.");
}
