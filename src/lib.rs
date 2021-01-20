use derive_builder::Builder;
use serde::{Deserialize, Serialize};

mod tests;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Builder, Clone)]
pub struct Chat {
    #[serde(flatten)]
    value: ChatType,

    #[serde(skip_serializing_if = "is_false")]
    #[serde(default)]
    #[builder(default = "false")]
    bold: bool,

    #[serde(skip_serializing_if = "is_false")]
    #[serde(default)]
    #[builder(default = "false")]
    italic: bool,

    #[serde(skip_serializing_if = "is_false")]
    #[serde(default)]
    #[builder(default = "false")]
    underlined: bool,

    #[serde(skip_serializing_if = "is_false")]
    #[serde(default)]
    #[builder(default = "false")]
    strikethrough: bool,

    #[serde(skip_serializing_if = "is_false")]
    #[serde(default)]
    #[builder(default = "false")]
    obfuscated: bool,

    #[serde(skip_serializing_if = "is_default_color")]
    #[serde(default)]
    #[builder(setter(into))]
    #[builder(default = "Color::None.into()")]
    color: String,

    #[serde(skip_serializing_if = "is_none")]
    #[serde(default)]
    #[builder(default = "Option::None")]
    insertion: Option<String>,

    #[serde(skip_serializing_if = "is_none")]
    #[serde(default)]
    #[builder(default = "Option::None")]
    click_event: Option<ClickEvent>,

    #[serde(skip_serializing_if = "is_none")]
    #[serde(default)]
    #[builder(default = "Option::None")]
    hover_event: Option<HoverEvent>,

    #[serde(skip_serializing_if = "is_empty")]
    #[serde(default)]
    #[builder(default = "Vec::new()")]
    extra: Vec<Chat>,
}

pub enum Color {
    RGB(u8, u8, u8),
    Black,
    DarkBlue,
    DarkGreen,
    DarkCyan,
    DarkRed,
    Purple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Lime,
    Cyan,
    Red,
    Pink,
    Yellow,
    White,
    Reset,
    None,
}

impl Into<String> for Color {
    fn into(self) -> String {
        match self {
            Color::RGB(r, g, b) => format!("#{:02X}{:02X}{:02X}", r, g, b),
            Color::Black => "0".into(),
            Color::DarkBlue => "1".into(),
            Color::DarkGreen => "2".into(),
            Color::DarkCyan => "3".into(),
            Color::DarkRed => "4".into(),
            Color::Purple => "5".into(),
            Color::Gold => "6".into(),
            Color::Gray => "7".into(),
            Color::DarkGray => "8".into(),
            Color::Blue => "9".into(),
            Color::Lime => "a".into(),
            Color::Cyan => "b".into(),
            Color::Red => "c".into(),
            Color::Pink => "d".into(),
            Color::Yellow => "e".into(),
            Color::White => "f".into(),
            Color::Reset => "r".into(),
            Color::None => "".into(),
        }
    }
}

fn is_none<T>(o: &Option<T>) -> bool {
    o.is_none()
}

fn is_false(b: &bool) -> bool {
    return !*b;
}

fn is_default_color(s: &String) -> bool {
    s == "reset" || s == "r" || s == ""
}

fn is_empty<T>(v: &Vec<T>) -> bool {
    v.is_empty()
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Builder, Clone)]
pub struct ClickEvent {
    #[builder(setter(into))]
    action: String,
    value: String,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Builder, Clone)]
pub struct HoverEvent {
    #[builder(setter(into))]
    action: String,
    value: ChatOrString,
}

pub enum ClickEventAction {
    OpenUrl,
    RunCommand,
    SuggestCommand,
    ChangePage,
}

impl Into<String> for ClickEventAction {
    fn into(self) -> String {
        match self {
            ClickEventAction::OpenUrl => "open_url".into(),
            ClickEventAction::RunCommand => "run_command".into(),
            ClickEventAction::SuggestCommand => "suggest_command".into(),
            ClickEventAction::ChangePage => "change_page".into(),
        }
    }
}

pub enum HoverEventAction {
    ShowText,
    ShowItem,
    ShowEntity,
}

impl Into<String> for HoverEventAction {
    fn into(self) -> String {
        match self {
            HoverEventAction::ShowText => "show_text".into(),
            HoverEventAction::ShowItem => "show_item".into(),
            HoverEventAction::ShowEntity => "show_entity".into(),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ChatOrString {
    String(String),
    Chat(Box<Chat>),
}

impl Default for Chat {
    fn default() -> Self {
        Chat {
            value: ChatType::default(),
            bold: false,
            italic: false,
            underlined: false,
            strikethrough: false,
            obfuscated: false,
            color: String::new(),
            insertion: None,
            click_event: None,
            hover_event: None,
            extra: vec![],
        }
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ChatType {
    String { text: String },
    // TODO Translation { translate: String, with: Vec<Chat> },
    // TODO Keybind { keybind: String },
    Score { score: Score }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct Score {
    name: String,
    objective: String,
    value: String
}

impl Score {
    pub fn new(name: String, objective: String, value: String) -> Score {
        Score { name, objective, value }
    }
}

impl Default for ChatType {
    fn default() -> Self {
        ChatType::String {
            text: String::new(),
        }
    }
}
