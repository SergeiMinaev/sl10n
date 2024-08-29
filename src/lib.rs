/// The `define_l10n!` macro defines localized messages directly within your Rust code without dependencies.
/// It allows you to create an enum of message keys and a struct to manage translations for multiple languages.
/// Using `enum` for message keys allows compile-time detection of typos, as incorrect key names will result in compilation errors.

/// # Usage
///
/// This macro generates an enum for message keys and a struct to manage the messages.
/// You can optionally provide custom names for the struct and enum, or use the default
/// names `Msgs` and `Msg`.
///
/// ## Syntax
///
/// ```rust
/// use sl10n::define_l10n;
///
/// define_l10n! {
///     // Custom names for struct and enum
///     CustomMsgs, CustomMsg,
///     Greeting => {
///         en: "Hello!",
///         ru: "Привет!",
///         es: "¡Hola!"
///     },
///     Farewell => {
///         en: "Goodbye, {name}.",
///         ru: "До свидания, {name}.",
///         es: "Adiós, {name}."
///     }
///     // ... more messages
/// }
///
/// // Or use default names (Msgs and Msg)
/// define_l10n! {
///     Greeting => {
///         en: "Hello!",
///         ru: "Привет!",
///         es: "¡Hola!"
///     },
///     Farewell => {
///         en: "Goodbye, {name}.",
///         ru: "До свидания, {name}.",
///         es: "Adiós, {name}."
///     }
///     // ... more messages
/// }
/// ```
///
/// ## Parameters
///
/// - `StructName`: (optional) The custom name for the struct that holds all messages.
/// - `EnumName`: (optional) The custom name for the enum representing message keys.
/// - `MessageKey`: The identifier for a particular message. This is used to retrieve the message.
/// - `lang`: A language code representing a specific translation.
/// - `"Translation in language"`: The message text for the specified language.
///
/// ## Example
///
/// ```rust
/// use sl10n::define_l10n;
///
/// define_l10n! {
///     // With custom names
///     CustomMsgs, CustomMsg,
///     Greeting => {
///         en: "Hello!",
///         ru: "Привет!",
///			es: "¡Hola!"
///     },
///     Farewell => {
///         en: "Goodbye.",
///         ru: "До свидания.",
///			es: "Adiós."
///     }
/// }
///
/// // Accessing messages
/// let msgs = CustomMsgs::new();
/// let msg = msgs.msg(CustomMsg::Greeting, "es"); // "¡Hola!"
/// ```
///
/// ## Dynamic Parameters
///
/// You can also replace dynamic placeholders in the message using the `dyn_msg` method:
///
/// ```rust
/// use std::collections::HashMap;
/// use sl10n::define_l10n;
///
/// define_l10n! {
///     // With default names (Msgs and Msg)
///     Greeting => {
///         en: "Hello, {name}!",
///         ru: "Привет, {name}!",
///			es: "¡Hola, {name}!"
///     },
///     Farewell => {
///         en: "Goodbye, {name}.",
///         ru: "До свидания, {name}.",
///			es: "Adiós, {name}."
///     }
/// }
///
/// // Accessing messages
/// let msgs = Msgs::new();
/// let mut params = HashMap::new();
/// params.insert("name", "Alice");
/// let msg = msgs.dyn_msg(Msg::Greeting, "es", &params);
/// // "¡Hola, Alice!"
///
/// // Optionally you can use .get_msg() for handling both - static and dynamic messages
/// let msg = msgs.get_msg(Msg::Greeting, "es", Some(&params));
/// // "¡Hola, Alice!"
/// ```
///
/// ## Separating messages by modules
///
/// ```rust
/// mod mymodule {
///     use std::collections::HashMap;
///     use once_cell::sync::Lazy;
///     use sl10n::define_l10n;
///     
///     
///     define_l10n! {
///         Greeting => {
///             en: "Hello!",
///             ru: "Привет!",
///             es: "¡Hola!"
///         },
///         Farewell => {
///             en: "Goodbye, {name}.",
///             ru: "До свидания, {name}.",
///             es: "Adiós, {name}."
///         }
///     }
///     
///     static MSGS: Lazy<Msgs> = Lazy::new(|| Msgs::new());
///     // Make public function to get localized messages:
///     pub fn t(msg: Msg, lang: &str, params: Option<&HashMap<&str, &str>>) -> String {
///       MSGS.get_msg(msg, lang, params)
///     }
/// }
/// 
/// fn main() {
/// 	//let msg = mymodule::t(mymodule::Msg::Greeting, "en", None);
///  	//println!("{msg}"); // Hello!
/// }
///
/// ```
/// See `./examples/modules/` for a complete code example.
///
///
#[macro_export]
macro_rules! define_l10n {
    ($struct_name:ident, $enum_name:ident, $($key:ident => {$($lang:ident: $value:expr),*}),* $(,)?) => {
        #[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
        pub enum $enum_name {
            $($key,)*
        }

        impl $enum_name {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(
                        $enum_name::$key => stringify!($key),
                    )*
                }
            }
        }

        pub struct $struct_name {
            messages: std::collections::HashMap<$enum_name, std::collections::HashMap<String, &'static str>>,
        }

        impl $struct_name {
            pub fn new() -> Self {
                let mut messages = std::collections::HashMap::new();
                $(
                    let mut lang_map = std::collections::HashMap::new();
                    $(
                        lang_map.insert(stringify!($lang).to_string(), $value);
                    )*
                    messages.insert($enum_name::$key, lang_map);
                )*
                $struct_name { messages }
            }

            pub fn get_msg(&self, key: $enum_name, lang: &str, params: Option<&std::collections::HashMap<&str, &str>>) -> String {
                let message = self.messages.get(&key)
                    .and_then(|langs| langs.get(lang))
                    .copied()
                    .unwrap_or("");

                let mut result = message.to_string();
				if let Some(params) = params {
					for (k, v) in params {
						result = result.replace(&format!("{{{}}}", k), v);
					}
				}
                result
            }

            pub fn msg(&self, key: $enum_name, lang: &str) -> String {
				self.get_msg(key, lang, None)
            }

            pub fn dyn_msg(&self, key: $enum_name, lang: &str, params: &std::collections::HashMap<&str, &str>) -> String {
				self.get_msg(key, lang, Some(params))
            }
        }
    };

	// Version with default names - Msgs and Msg
    ($($key:ident => {$($lang:ident: $value:expr),*}),* $(,)?) => {
        define_l10n!(Msgs, Msg, $($key => {$($lang: $value),*}),*);
    };
}
