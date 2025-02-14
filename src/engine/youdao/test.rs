//! Generated file, do not edit by hand, see `/build.rs`

use super::Translator;
use crate::{Translate, Translation};
#[test]
fn cn_word() {
    let trans = Translator;
    let r = trans.translate("你好");
    assert!(r.is_ok());
    let r = r.unwrap();
    assert!(r.is_some());
    let r = r.unwrap();
    let expected: Translation = serde_json::from_str(
        r#"{
            "query": "你好",
            "url": "http://dict.youdao.com/w/%E4%BD%A0%E5%A5%BD",
            "prons": [
                {
                    "tag": "CN",
                    "value": "nǐ hǎo",
                    "audio": null
                }
            ],
            "exps": [
                {
                    "tag": "Phrase",
                    "items": [
                        "hello",
                        "hi",
                        "how do you do"
                    ]
                },
                {
                    "tag": "Web",
                    "items": [
                        "Hello",
                        "How do you do",
                        "hi",
                        "What's up"
                    ]
                }
            ],
            "phrases": [
                [
                    "你好吗",
                    [
                        "How are you",
                        "How Do You Do",
                        "Are You OK"
                    ]
                ],
                [
                    "你好小娜",
                    [
                        "Hey Cortana"
                    ]
                ],
                [
                    "你好啊",
                    [
                        "Hello",
                        "Hey"
                    ]
                ],
                [
                    "先生你好",
                    [
                        "Hello Sir"
                    ]
                ]
            ]
        }"#,
    )
    .unwrap();
    let msg = format!(
        "\n=== Json dump:\n{}\n=== ",
        serde_json::to_string(&r).unwrap()
    );
    assert_eq!(r.query, expected.query, "{}", msg);
    assert_eq!(r.url, expected.url, "{}", msg);
    assert_eq!(r.exps, expected.exps, "{}", msg);
    assert_eq!(r.phrases, expected.phrases, "{}", msg);
}
#[test]
fn cn_phrase() {
    let trans = Translator;
    let r = trans.translate("你好，世界");
    assert!(r.is_ok());
    let r = r.unwrap();
    assert!(r.is_some());
    let r = r.unwrap();
    let expected: Translation = serde_json::from_str(
        r#"{
            "query": "你好,世界",
            "url": "http://dict.youdao.com/w/%E4%BD%A0%E5%A5%BD%EF%BC%8C%E4%B8%96%E7%95%8C",
            "prons": [],
            "exps": [
                {
                    "tag": "Machine",
                    "items": [
                        "Hello, world"
                    ]
                },
                {
                    "tag": "Web",
                    "items": []
                }
            ],
            "phrases": [
                [
                    "你好世界",
                    [
                        "hello world"
                    ]
                ],
                [
                    "你好世界新闻和密钥",
                    [
                        "hello world press and key"
                    ]
                ],
                [
                    "你好世界新闻和关键",
                    [
                        "hello world press and key"
                    ]
                ],
                [
                    "你好世界命令行参数",
                    [
                        "hello world command line args"
                    ]
                ],
                [
                    "世界你好",
                    [
                        "Hello World",
                        "World how are"
                    ]
                ]
            ]
        }"#,
    )
    .unwrap();
    let msg = format!(
        "\n=== Json dump:\n{}\n=== ",
        serde_json::to_string(&r).unwrap()
    );
    assert_eq!(r.query, expected.query, "{}", msg);
    assert_eq!(r.url, expected.url, "{}", msg);
    assert_eq!(r.exps, expected.exps, "{}", msg);
    assert_eq!(r.phrases, expected.phrases, "{}", msg);
}
#[test]
fn en_word() {
    let trans = Translator;
    let r = trans.translate("hello");
    assert!(r.is_ok());
    let r = r.unwrap();
    assert!(r.is_some());
    let r = r.unwrap();
    let expected: Translation = serde_json::from_str(
        r#"{
            "query": "hello",
            "url": "http://dict.youdao.com/w/hello",
            "prons": [
                {
                    "tag": "UK",
                    "value": "həˈləʊ",
                    "audio": "https://dict.youdao.com/dictvoice?audio=hello&type=1"
                },
                {
                    "tag": "US",
                    "value": "həˈloʊ",
                    "audio": "https://dict.youdao.com/dictvoice?audio=hello&type=2"
                }
            ],
            "exps": [
                {
                    "tag": {
                        "Pos": "int."
                    },
                    "items": [
                        "喂，你好（用于问候或打招呼）",
                        "喂，你好（打电话时的招呼语）",
                        "喂，你好（引起别人注意的招呼语）",
                        "<非正式>喂，嘿 (认为别人说了蠢话或分心)",
                        "<英，旧>嘿（表示惊讶）"
                    ]
                },
                {
                    "tag": {
                        "Pos": "n."
                    },
                    "items": [
                        "招呼，问候",
                        "（Hello）（法、印、美、俄）埃洛（人名）"
                    ]
                },
                {
                    "tag": {
                        "Pos": "v."
                    },
                    "items": [
                        "说（或大声说）“喂”",
                        "打招呼"
                    ]
                },
                {
                    "tag": "Web",
                    "items": [
                        "您好",
                        "哈啰",
                        "喂"
                    ]
                }
            ],
            "phrases": [
                [
                    "Hello Kitty",
                    [
                        "凯蒂猫",
                        "吉蒂猫"
                    ]
                ],
                [
                    "Hello Neighbor",
                    [
                        "你好邻居"
                    ]
                ]
            ]
        }"#,
    )
    .unwrap();
    let msg = format!(
        "\n=== Json dump:\n{}\n=== ",
        serde_json::to_string(&r).unwrap()
    );
    assert_eq!(r.query, expected.query, "{}", msg);
    assert_eq!(r.url, expected.url, "{}", msg);
    assert_eq!(r.exps, expected.exps, "{}", msg);
    assert_eq!(r.phrases, expected.phrases, "{}", msg);
}
#[test]
fn en_phrase() {
    let trans = Translator;
    let r = trans.translate("Hello, world");
    assert!(r.is_ok());
    let r = r.unwrap();
    assert!(r.is_some());
    let r = r.unwrap();
    let expected: Translation = serde_json::from_str(
        r#"{
            "query": "Hello, world",
            "url": "http://dict.youdao.com/w/Hello,%20world",
            "prons": [],
            "exps": [
                {
                    "tag": "Machine",
                    "items": [
                        "你好，世界"
                    ]
                },
                {
                    "tag": "Web",
                    "items": []
                }
            ],
            "phrases": [
                [
                    "hello world",
                    [
                        "你好世界",
                        "开始",
                        "别来无恙",
                        "举个例子"
                    ]
                ],
                [
                    "Hello world UI",
                    [
                        "实现方法"
                    ]
                ],
                [
                    "hello world press and key",
                    [
                        "您好世界新闻和关键",
                        "你好世界新闻和密钥"
                    ]
                ],
                [
                    "Hello world only",
                    [
                        "世间只有你好"
                    ]
                ],
                [
                    "Hello world always",
                    [
                        "世间始终你好"
                    ]
                ],
                [
                    "air hello world",
                    [
                        "调税计算器"
                    ]
                ],
                [
                    "Hello Kitty Cutie World",
                    [
                        "凯蒂猫顽皮世界"
                    ]
                ],
                [
                    "HELLO WORLD&amp",
                    [
                        "第一行代码"
                    ]
                ]
            ]
        }"#,
    )
    .unwrap();
    let msg = format!(
        "\n=== Json dump:\n{}\n=== ",
        serde_json::to_string(&r).unwrap()
    );
    assert_eq!(r.query, expected.query, "{}", msg);
    assert_eq!(r.url, expected.url, "{}", msg);
    assert_eq!(r.exps, expected.exps, "{}", msg);
    assert_eq!(r.phrases, expected.phrases, "{}", msg);
}

// vim: ro
