use std::str::FromStr;

use vtt::{VttHeader, WebVtt};

fn get_test_vtt_str() -> &'static str {
    include_str!("test.vtt")
}

#[test]
fn ahoy() {
    let vtt_str = get_test_vtt_str();
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
    for cue in vtt.cues {
        println!("{}", cue.payload);
    }
}
