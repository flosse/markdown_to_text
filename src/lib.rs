#![warn(clippy::all, clippy::pedantic)]

use log::debug;
use pulldown_cmark::{Event, Options, Parser, Tag};

#[must_use]
pub fn strip_markdown(markdown: &str) -> String {
    // GFM tables and tasks lists are not enabled.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(&markdown, options);
    let mut tags_stack = Vec::new();
    let mut buffer = String::new();

    // For each event we push into the buffer to produce the 'stripped' version.
    for event in parser {
        debug!("{:?}", event);
        match event {
            // The start and end events don't contain the text inside the tag. That's handled by the `Event::Text` arm.
            Event::Start(tag) => {
                start_tag(&tag, &mut buffer);
                tags_stack.push(tag);
            }
            Event::End(tag) => {
                end_tag(&tag, &mut buffer);
                tags_stack.pop();
            }
            Event::Text(content) => {
                if !tags_stack.iter().any(is_strikethrough) {
                    buffer.push_str(&content)
                }
            }
            Event::Code(content) => buffer.push_str(&content),
            Event::SoftBreak => buffer.push(' '),
            _ => (),
        }
    }
    buffer.trim().to_string()
}

fn start_tag(tag: &Tag, buffer: &mut String) {
    match tag {
        Tag::Link(_, _, title) | Tag::Image(_, _, title) => buffer.push_str(&title),
        Tag::Item => buffer.push_str("• "),
        _ => (),
    }
}

fn end_tag(tag: &Tag, buffer: &mut String) {
    match tag {
        Tag::Paragraph | Tag::Table(_) | Tag::Heading(_) | Tag::List(_) => buffer.push_str("\n\n"),
        Tag::CodeBlock(_) | Tag::TableHead | Tag::TableRow | Tag::Item => buffer.push('\n'),
        _ => (),
    }
}

fn is_strikethrough(tag: &Tag) -> bool {
    match tag {
        Tag::Strikethrough => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_inline_strong() {
        let markdown = r#"**Hello**"#;
        let expected = "Hello";
        assert_eq!(strip_markdown(markdown), expected);
    }

    #[test]
    fn basic_inline_emphasis() {
        let markdown = r#"_Hello_"#;
        let expected = "Hello";
        assert_eq!(strip_markdown(markdown), expected);
    }

    #[test]
    fn basic_header() {
        let markdown = r#"# Header

End paragraph."#;
        let expected = "Header

End paragraph.";
        assert_eq!(strip_markdown(markdown), expected);
    }

    #[test]
    fn alt_header() {
        let markdown = r#"
Header
======

End paragraph."#;
        let expected = "Header

End paragraph.";
        assert_eq!(strip_markdown(markdown), expected);
    }

    #[test]
    fn strong_emphasis() {
        let markdown = r#"**asterisks and _underscores_**"#;
        let expected = "asterisks and underscores";
        assert_eq!(strip_markdown(markdown), expected);
    }

    #[test]
    fn strikethrough() {
        let markdown = r#"This was ~~erased~~ deleted."#;
        let expected = "This was  deleted.";
        assert_eq!(strip_markdown(markdown), expected);
    }

    #[test]
    fn mixed_list() {
        let markdown = r#"
1. First ordered list item
2. Another item
1. Actual numbers don't matter, just that it's a number
  1. Ordered sub-list
4. And another item.
"#;

        let expected = r#"• First ordered list item
• Another item
• Actual numbers don't matter, just that it's a number
• Ordered sub-list
• And another item."#;
        assert_eq!(strip_markdown(markdown), expected);
    }

    #[test]
    fn basic_list() {
        let markdown = r#"
* alpha
* beta
"#;
        let expected = r#"• alpha
• beta"#;
        assert_eq!(strip_markdown(markdown), expected);
    }

    #[test]
    fn list_with_header() {
        let markdown = r#"# Title
* alpha
* beta
"#;
        let expected = r#"Title

• alpha
• beta"#;
        assert_eq!(strip_markdown(markdown), expected);
    }

    #[test]
    fn basic_link() {
        let markdown = "I'm an [inline-style link](https://www.google.com).";
        let expected = "I'm an inline-style link.";
        assert_eq!(strip_markdown(markdown), expected)
    }

    #[ignore]
    #[test]
    fn link_with_itself() {
        let markdown = "Go to [https://www.google.com].";
        let expected = "Go to https://www.google.com.";
        assert_eq!(strip_markdown(markdown), expected)
    }

    #[test]
    fn basic_image() {
        let markdown = "As displayed in ![img alt text](https://github.com/adam-p/markdown-here/raw/master/src/common/images/icon48.png).";
        let expected = "As displayed in img alt text.";
        assert_eq!(strip_markdown(markdown), expected);
    }

    #[test]
    fn inline_code() {
        let markdown = "This is `inline code`.";
        let expected = "This is inline code.";
        assert_eq!(strip_markdown(markdown), expected);
    }

    #[test]
    fn code_block() {
        let markdown = r#"Start paragraph.
```javascript
var s = "JavaScript syntax highlighting";
alert(s);
```
End paragraph."#;
        let expected = r#"Start paragraph.

var s = "JavaScript syntax highlighting";
alert(s);

End paragraph."#;
        assert_eq!(strip_markdown(markdown), expected);
    }

    #[test]
    fn block_quote() {
        let markdown = r#"Start paragraph.

> Blockquotes are very handy in email to emulate reply text.
> This line is part of the same quote.

End paragraph."#;
        let expected = "Start paragraph.

Blockquotes are very handy in email to emulate reply text. This line is part of the same quote.

End paragraph.";
        assert_eq!(strip_markdown(markdown), expected);
    }

    #[test]
    fn paragraphs() {
        let markdown = r#"Paragraph 1.

Paragraph 2."#;
        let expected = "Paragraph 1.

Paragraph 2.";
        assert_eq!(strip_markdown(markdown), expected);
    }
}
