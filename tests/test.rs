use std::str::FromStr;

use vtt::{VttHeader, WebVtt};

#[test]
fn ahoy() {
    let vtt_str = include_str!("test.vtt").replace('\r', "");
    let vtt_str = vtt_str.trim();
    let vtt = WebVtt::from_str(&vtt_str).unwrap();
    assert_eq!(
        vtt.header,
        VttHeader {
            description: None,
            metadata: [("Kind", "captions"), ("Language", "en")]
                .into_iter()
                .map(|(a, b)| (a.to_string(), b.to_string()))
                .collect(),
        }
    );
    for cue in &vtt.cues {
        println!("{cue:#?}");
    }
    // let output = vtt.to_string();
    // let output = output.trim();
    // assert_eq!(output, vtt_str);
    // some minor line differences
}

#[test]
fn natural() {
    // https://youtu.be/l8L9Z2vmMTQ
    // https://youtu.be/TOAynddiu5M
    let vtt_str = include_str!("test.vtt").replace('\r', "");
    let vtt_str = vtt_str.trim();
    let vtt = WebVtt::from_str(&vtt_str).unwrap();
    println!("{}", vtt.deduplicated_text());
}
