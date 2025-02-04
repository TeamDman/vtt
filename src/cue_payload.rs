use crate::VttTimestamp;
use std::{
    fmt,
    ops::{Deref, DerefMut},
    str::FromStr,
};

/// Represents a segment (or “fragment”) of a cue’s payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VttCueFragment {
    /// A literal text fragment.
    Text(String),
    /// A timed fragment that starts at a given timestamp and has associated text.
    TimedText {
        timestamp: VttTimestamp,
        text: String,
    },
}
impl VttCueFragment {
    pub fn text(&self) -> &str {
        match self {
            VttCueFragment::Text(text) => text,
            VttCueFragment::TimedText { text, .. } => text,
        }
    }
}
impl fmt::Display for VttCueFragment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VttCueFragment::Text(text) => write!(f, "{}", text),
            VttCueFragment::TimedText { timestamp, text } => {
                write!(f, "<{}><c>{}</c>", timestamp, text)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VttCuePayload(Vec<VttCueFragment>);
impl Deref for VttCuePayload {
    type Target = Vec<VttCueFragment>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for VttCuePayload {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<T> From<T> for VttCuePayload
where
    T: AsRef<str>,
{
    fn from(s: T) -> Self {
        VttCuePayload(parse_cue_payload(s.as_ref()))
    }
}
impl fmt::Display for VttCuePayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for fragment in &self.0 {
            write!(f, "{}", fragment)?;
        }
        Ok(())
    }
}

/// Parses a cue payload (a &str) into a series of semantic fragments.
///
/// This function looks for patterns like:
///   <timestamp><c> ... </c>
/// and splits the payload accordingly. Any text outside those tags is kept as literal text.
///
/// For example, given an input like:
///   "when<00:00:00.199><c> I</c><00:00:00.280><c> started</c>"
/// it will produce a fragment for the literal "when" and timed fragments for " I" and " started".
pub fn parse_cue_payload(payload: &str) -> Vec<VttCueFragment> {
    let mut fragments = Vec::new();
    let mut rest = payload;

    while let Some(start_idx) = rest.find('<') {
        // Take any literal text that comes before the next tag.
        if start_idx > 0 {
            let literal = &rest[..start_idx];
            if !literal.trim().is_empty() {
                fragments.push(VttCueFragment::Text(literal.to_string()));
            }
        }
        // Now, rest starts with '<'
        rest = &rest[start_idx..];

        // Try to find the closing '>' of this tag.
        if let Some(end_idx) = rest.find('>') {
            let tag_content = &rest[1..end_idx]; // content between '<' and '>'
                                                 // Try to parse the tag content as a timestamp.
            if let Ok(timestamp) = VttTimestamp::from_str(tag_content) {
                // We’ve recognized a timestamp tag.
                rest = &rest[end_idx + 1..];
                // Look for an optional <c> tag immediately following.
                if rest.starts_with("<c>") {
                    // Find the corresponding closing tag </c>
                    if let Some(close_idx) = rest.find("</c>") {
                        // Extract text inside the <c> ... </c> tags.
                        let text = &rest[3..close_idx];
                        fragments.push(VttCueFragment::TimedText {
                            timestamp,
                            text: text.to_string(),
                        });
                        // Advance past the closing tag.
                        rest = &rest[close_idx + 4..];
                    } else {
                        // If no closing tag is found, treat the rest as literal.
                        fragments.push(VttCueFragment::Text(rest.to_string()));
                        break;
                    }
                } else {
                    // If there isn’t a <c> tag, you might decide to treat this as a timed fragment with empty text
                    // or simply skip it. Here we push an empty timed fragment.
                    fragments.push(VttCueFragment::TimedText {
                        timestamp,
                        text: String::new(),
                    });
                }
            } else {
                // If the content between '<' and '>' isn’t a valid timestamp,
                // then we treat the '<' as literal text.
                fragments.push(VttCueFragment::Text("<".to_string()));
                rest = &rest[1..];
            }
        } else {
            // If we can't find a closing '>', push the rest as literal.
            fragments.push(VttCueFragment::Text(rest.to_string()));
            break;
        }
    }

    // If any text remains after the loop, add it as literal.
    if !rest.trim().is_empty() {
        fragments.push(VttCueFragment::Text(rest.to_string()));
    }

    fragments
}
