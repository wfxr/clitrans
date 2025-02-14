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
            "url": "https://cn.bing.com/dict/search?q=%E4%BD%A0%E5%A5%BD&mkt=zh-cn",
            "prons": [
                {
                    "tag": "CN",
                    "value": "nǐ hǎo",
                    "audio": null
                }
            ],
            "exps": [
                {
                    "tag": {
                        "Pos": "na."
                    },
                    "items": [
                        "hello",
                        "〈正式,口〉how do you do?"
                    ]
                },
                {
                    "tag": {
                        "Pos": "Web"
                    },
                    "items": [
                        "Hello",
                        "Hi",
                        "How do you do"
                    ]
                }
            ],
            "phrases": []
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
    let expected : Translation = serde_json :: from_str (r#"{
            "query": "你好,世界",
            "url": "https://cn.bing.com/dict/search?q=%E4%BD%A0%E5%A5%BD%EF%BC%8C%E4%B8%96%E7%95%8C&mkt=zh-cn",
            "prons": [],
            "exps": [
                {
                    "tag": {
                        "Pos": "Web"
                    },
                    "items": [
                        "hello world",
                        "Well,hello world"
                    ]
                }
            ],
            "phrases": []
        }"#) . unwrap () ;
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
    let expected : Translation = serde_json :: from_str (r#"{
            "query": "hello",
            "url": "https://cn.bing.com/dict/search?q=hello&mkt=zh-cn",
            "prons": [
                {
                    "tag": "US",
                    "value": "heˈləʊ",
                    "audio": "https://dictionary.blob.core.chinacloudapi.cn/media/audio/tom/bf/b1/BFB1169AD46D18FDC9145E494EF4D22B.mp3"
                },
                {
                    "tag": "UK",
                    "value": "hə'ləʊ",
                    "audio": "https://dictionary.blob.core.chinacloudapi.cn/media/audio/george/bf/b1/BFB1169AD46D18FDC9145E494EF4D22B.mp3"
                }
            ],
            "exps": [
                {
                    "tag": {
                        "Pos": "int."
                    },
                    "items": [
                        "你好",
                        "喂",
                        "您好",
                        "哈喽"
                    ]
                },
                {
                    "tag": {
                        "Pos": "Web"
                    },
                    "items": [
                        "哈罗",
                        "哈啰",
                        "大家好"
                    ]
                }
            ],
            "phrases": []
        }"#) . unwrap () ;
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
            "query": "Hello world",
            "url": "https://cn.bing.com/dict/search?q=Hello,%20world&mkt=zh-cn",
            "prons": [],
            "exps": [
                {
                    "tag": {
                        "Pos": "n."
                    },
                    "items": [
                        "世界你好"
                    ]
                },
                {
                    "tag": {
                        "Pos": "Web"
                    },
                    "items": [
                        "你好世界",
                        "别来无恙",
                        "哈罗"
                    ]
                }
            ],
            "phrases": []
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
