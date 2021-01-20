#[allow(unused_imports)]
use crate::{Chat, ChatBuilder, ChatOrString, ChatType, ClickEvent, Color, HoverEvent, HoverEventAction, HoverEventBuilder, Score};

#[test]
fn string_serialize() {
    let chat = Chat {
        value: ChatType::String {
            text: "test".to_string(),
        },
        ..Chat::default()
    };
    assert_eq!(serde_json::to_string(&chat).unwrap(), "{\"text\":\"test\"}")
}

#[test]
fn string_deserialize() {
    let chat = Chat {
        value: ChatType::String {
            text: "test".to_string(),
        },
        ..Chat::default()
    };
    assert_eq!(chat, serde_json::from_str("{\"text\":\"test\"}").unwrap())
}

#[test]
fn big_test() {
    let chat = Chat {
        value: ChatType::String {
            text: "test".to_string(),
        },
        bold: true,
        italic: true,
        underlined: true,
        strikethrough: true,
        obfuscated: true,
        color: "#ffff00".to_string(),
        insertion: Some("test".to_string()),
        click_event: Some(ClickEvent {
            action: "action".to_string(),
            value: "value".to_string(),
        }),
        hover_event: Some(HoverEvent {
            action: "action".to_string(),
            value: ChatOrString::String("string".to_string()),
        }),
        extra: vec![Chat {
            value: ChatType::Score {
                score: Score::new("Name".to_string(), "objective".to_string(), 42.to_string())
            },
            bold: false,
            italic: false,
            underlined: true,
            strikethrough: false,
            obfuscated: true,
            color: "reset".to_string(),
            insertion: None,
            click_event: Some(ClickEvent {
                action: "action2".to_string(),
                value: "value2".to_string(),
            }),
            hover_event: Some(HoverEvent {
                action: "action".to_string(),
                value: ChatOrString::Chat(Box::new(Chat {
                    value: ChatType::String {
                        text: "test".to_string(),
                    },
                    ..Default::default()
                })),
            }),
            extra: vec![],
        }],
    };
    assert_eq!(serde_json::to_string(&chat).unwrap(), "{\"text\":\"test\",\"bold\":true,\"italic\":true,\"underlined\":true,\"strikethrough\":true,\"obfuscated\":true,\"color\":\"#ffff00\",\"insertion\":\"test\",\"click_event\":{\"action\":\"action\",\"value\":\"value\"},\"hover_event\":{\"action\":\"action\",\"value\":\"string\"},\"extra\":[{\"score\":{\"name\":\"Name\",\"objective\":\"objective\",\"value\":\"42\"},\"underlined\":true,\"obfuscated\":true,\"click_event\":{\"action\":\"action2\",\"value\":\"value2\"},\"hover_event\":{\"action\":\"action\",\"value\":{\"text\":\"test\"}}}]}");
}

#[test]
fn builder() {
    let message = ChatBuilder::default()
        .value(ChatType::String {
            text: "Hello!".to_string(),
        })
        .bold(true)
        .color(Color::RGB(0, 12, 255))
        .hover_event(
            HoverEventBuilder::default()
                .action(HoverEventAction::ShowText)
                .value(ChatOrString::String("Test".to_string()))
                .build()
                .ok(),
        )
        .build()
        .unwrap();
    assert_eq!(serde_json::to_string(&message).unwrap(), "{\"text\":\"Hello!\",\"bold\":true,\"color\":\"#000CFF\",\"hover_event\":{\"action\":\"show_text\",\"value\":\"Test\"}}")
}
