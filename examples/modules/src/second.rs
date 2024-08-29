use std::collections::HashMap;
use once_cell::sync::Lazy;
use sl10n::define_l10n;


define_l10n! {
    ChangesSaved => {
        ru: "Изменения сохранены.",
        en: "Changes saved.",
        es: "Cambios guardados."
    },
    ChangesSavedErr => {
        ru: "Не удалось сохранить изменения.",
        en: "Failed to save changes.",
        es: "No se pudieron guardar los cambios."
    }
}

// Msgs::new() will run only once.
static MSGS: Lazy<Msgs> = Lazy::new(|| Msgs::new());

// Use this function to get localized messages.
pub fn t(msg: Msg, lang: &str, params: Option<&HashMap<&str, &str>>) -> String {
	MSGS.get_msg(msg, lang, params)
}
