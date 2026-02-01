use pulldown_cmark::{Parser, Event, Tag, TagEnd, HeadingLevel};
use uuid::Uuid;
use crate::models::spec::Requirement;

pub fn parse_spec(spec_id: &str, content: &str) -> Vec<Requirement> {
    let parser = Parser::new(content);
    let mut requirements = Vec::new();
    let mut current_section = String::from("General");
    let mut in_heading = false;
    let mut heading_text = String::new();
    let mut in_list_item = false;
    let mut list_item_text = String::new();
    let mut is_requirement_section = false;

    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                in_heading = true;
                heading_text.clear();
                // Top-level headings reset section tracking
                if matches!(level, HeadingLevel::H1 | HeadingLevel::H2 | HeadingLevel::H3) {
                    // will set section when heading ends
                }
            }
            Event::End(TagEnd::Heading(_)) => {
                in_heading = false;
                current_section = heading_text.trim().to_string();
                is_requirement_section = is_requirement_like_section(&current_section);
            }
            Event::Start(Tag::Item) => {
                in_list_item = true;
                list_item_text.clear();
            }
            Event::End(TagEnd::Item) => {
                in_list_item = false;
                let text = list_item_text.trim().to_string();
                if !text.is_empty() && (is_requirement_section || looks_like_requirement(&text)) {
                    let req_type = classify_requirement_type(&current_section, &text);
                    let priority = classify_priority(&text);
                    requirements.push(Requirement {
                        id: Uuid::new_v4().to_string(),
                        spec_id: spec_id.to_string(),
                        section: current_section.clone(),
                        description: text,
                        req_type,
                        priority,
                    });
                }
            }
            Event::Text(text) => {
                if in_heading {
                    heading_text.push_str(&text);
                } else if in_list_item {
                    list_item_text.push_str(&text);
                }
            }
            Event::Code(code) => {
                if in_heading {
                    heading_text.push_str(&code);
                } else if in_list_item {
                    list_item_text.push_str(&code);
                }
            }
            _ => {}
        }
    }

    requirements
}

fn is_requirement_like_section(section: &str) -> bool {
    let lower = section.to_lowercase();
    lower.contains("requirement")
        || lower.contains("user stor")
        || lower.contains("feature")
        || lower.contains("functional")
        || lower.contains("specification")
        || lower.contains("capability")
        || lower.contains("constraint")
        || lower.contains("acceptance criteria")
        || lower.contains("use case")
}

fn looks_like_requirement(text: &str) -> bool {
    let lower = text.to_lowercase();
    // Explicit requirement patterns
    lower.starts_with("as a ")
        || lower.starts_with("the system shall ")
        || lower.starts_with("the system must ")
        || lower.starts_with("the application shall ")
        || lower.starts_with("the application must ")
        || lower.starts_with("shall ")
        || lower.starts_with("must ")
        || lower.contains("**shall**")
        || lower.contains("**must**")
        // Bold text only counts if it contains enough words to be a real requirement
        || (text.starts_with("**") && text.contains(' ') && lower.split_whitespace().count() >= 5)
}

fn classify_requirement_type(section: &str, text: &str) -> String {
    let lower_section = section.to_lowercase();
    let lower_text = text.to_lowercase();

    if lower_section.contains("non-functional")
        || lower_section.contains("performance")
        || lower_section.contains("security")
        || lower_section.contains("scalability")
        || lower_text.contains("performance")
        || lower_text.contains("latency")
        || lower_text.contains("availability")
    {
        "non_functional".to_string()
    } else if lower_section.contains("constraint")
        || lower_text.contains("constraint")
        || lower_text.contains("limitation")
    {
        "constraint".to_string()
    } else {
        "functional".to_string()
    }
}

fn classify_priority(text: &str) -> String {
    let lower = text.to_lowercase();
    if lower.contains("critical") || lower.contains("must have") || lower.contains("**must**") {
        "high".to_string()
    } else if lower.contains("nice to have") || lower.contains("optional") || lower.contains("could") {
        "low".to_string()
    } else {
        "medium".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_spec() {
        let content = r#"# My Spec

## Requirements

- The system shall authenticate users via email and password
- The system shall allow users to reset their password
- As a user, I want to view my dashboard

## Non-Functional Requirements

- The system shall respond within 200ms for all API calls
"#;
        let reqs = parse_spec("test-spec-id", content);
        assert_eq!(reqs.len(), 4);
        assert_eq!(reqs[0].req_type, "functional");
        assert_eq!(reqs[3].req_type, "non_functional");
    }
}
