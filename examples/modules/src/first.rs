use std::collections::HashMap;
use once_cell::sync::Lazy;
use sl10n::define_l10n;


define_l10n! {
    Greeting => {
        en: "Hello!",
        ru: "Привет!",
        es: "¡Hola!"
    },
    Farewell => {
        en: "Goodbye, {name}.",
        ru: "Пока, {name}.",
        es: "Adiós, {name}."
    }
}

// Msgs::new() will run only once.
static MSGS: Lazy<Msgs> = Lazy::new(|| Msgs::new());

// Use this function to get localized messages.
pub fn t(msg: Msg, lang: &str, params: Option<&HashMap<&str, &str>>) -> String {
	MSGS.get_msg(msg, lang, params)
}
