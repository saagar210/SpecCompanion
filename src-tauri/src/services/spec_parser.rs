use pulldown_cmark::{Parser, Event, Tag, TagEnd, HeadingLevel};
use uuid::Uuid;
use crate::models::spec::Requirement;

/// Heading hierarchy tracker
#[derive(Debug, Clone)]
struct HeadingStack {
    h1: Option<String>,
    h2: Option<String>,
    h3: Option<String>,
    h4: Option<String>,
}

impl HeadingStack {
    fn new() -> Self {
        Self {
            h1: None,
            h2: None,
            h3: None,
            h4: None,
        }
    }

    fn update(&mut self, level: HeadingLevel, text: String) {
        match level {
            HeadingLevel::H1 => {
                self.h1 = Some(text);
                self.h2 = None;
                self.h3 = None;
                self.h4 = None;
            }
            HeadingLevel::H2 => {
                self.h2 = Some(text);
                self.h3 = None;
                self.h4 = None;
            }
            HeadingLevel::H3 => {
                self.h3 = Some(text);
                self.h4 = None;
            }
            HeadingLevel::H4 => {
                self.h4 = Some(text);
            }
            _ => {}
        }
    }

    fn get_full_path(&self) -> String {
        let mut parts = Vec::new();
        if let Some(h1) = &self.h1 {
            parts.push(h1.as_str());
        }
        if let Some(h2) = &self.h2 {
            parts.push(h2.as_str());
        }
        if let Some(h3) = &self.h3 {
            parts.push(h3.as_str());
        }
        if let Some(h4) = &self.h4 {
            parts.push(h4.as_str());
        }
        if parts.is_empty() {
            "General".to_string()
        } else {
            parts.join(" > ")
        }
    }

    fn get_current_section(&self) -> String {
        self.h4.clone()
            .or_else(|| self.h3.clone())
            .or_else(|| self.h2.clone())
            .or_else(|| self.h1.clone())
            .unwrap_or_else(|| "General".to_string())
    }
}

pub fn parse_spec(spec_id: &str, content: &str) -> Vec<Requirement> {
    let parser = Parser::new(content);
    let mut requirements = Vec::new();
    let mut heading_stack = HeadingStack::new();
    let mut in_heading = false;
    let mut heading_text = String::new();
    let mut current_heading_level = HeadingLevel::H1;
    let mut in_list_item = false;
    let mut list_item_text = String::new();
    let mut is_requirement_section = false;

    // Table tracking
    let mut in_table = false;
    let mut table_headers: Vec<String> = Vec::new();
    let mut table_row: Vec<String> = Vec::new();
    let mut in_table_head = false;
    let mut in_table_cell = false;
    let mut table_cell_text = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                in_heading = true;
                heading_text.clear();
                current_heading_level = level;
            }
            Event::End(TagEnd::Heading(_)) => {
                in_heading = false;
                heading_stack.update(current_heading_level, heading_text.trim().to_string());
                is_requirement_section = is_requirement_like_section(&heading_stack.get_current_section());
            }
            Event::Start(Tag::Table(_)) => {
                in_table = true;
                table_headers.clear();
            }
            Event::End(TagEnd::Table) => {
                in_table = false;
            }
            Event::Start(Tag::TableHead) => {
                in_table_head = true;
            }
            Event::End(TagEnd::TableHead) => {
                in_table_head = false;
            }
            Event::Start(Tag::TableRow) => {
                table_row.clear();
            }
            Event::End(TagEnd::TableRow) => {
                // Process completed table row as potential requirement
                if !in_table_head && !table_row.is_empty() {
                    if let Some(requirement) = extract_requirement_from_table_row(
                        spec_id,
                        &table_headers,
                        &table_row,
                        &heading_stack,
                    ) {
                        requirements.push(requirement);
                    }
                }
            }
            Event::Start(Tag::TableCell) => {
                in_table_cell = true;
                table_cell_text.clear();
            }
            Event::End(TagEnd::TableCell) => {
                in_table_cell = false;
                if in_table_head {
                    table_headers.push(table_cell_text.trim().to_string());
                } else {
                    table_row.push(table_cell_text.trim().to_string());
                }
            }
            Event::Start(Tag::Item) => {
                in_list_item = true;
                list_item_text.clear();
            }
            Event::End(TagEnd::Item) => {
                in_list_item = false;
                let text = list_item_text.trim().to_string();
                if !text.is_empty() && (is_requirement_section || looks_like_requirement(&text)) {
                    let (clean_text, marker) = extract_requirement_marker(&text);
                    let req_type = classify_requirement_type(&heading_stack.get_current_section(), &clean_text);
                    let priority = classify_priority(&clean_text);
                    requirements.push(Requirement {
                        id: Uuid::new_v4().to_string(),
                        spec_id: spec_id.to_string(),
                        section: heading_stack.get_full_path(),
                        description: if let Some(m) = marker {
                            format!("[{}] {}", m, clean_text)
                        } else {
                            clean_text
                        },
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
                } else if in_table_cell {
                    table_cell_text.push_str(&text);
                }
            }
            Event::Code(code) => {
                if in_heading {
                    heading_text.push_str(&code);
                } else if in_list_item {
                    list_item_text.push_str(&code);
                } else if in_table_cell {
                    table_cell_text.push_str(&code);
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

fn extract_requirement_marker(text: &str) -> (String, Option<String>) {
    // Check for explicit requirement markers: REQ-001:, US-123:, FR-45:, etc.
    let marker_patterns = [
        r"^(REQ-\d+):\s*",
        r"^(US-\d+):\s*",
        r"^(FR-\d+):\s*",
        r"^(NFR-\d+):\s*",
        r"^(UC-\d+):\s*",
        r"^(FEAT-\d+):\s*",
    ];

    for pattern in &marker_patterns {
        if let Some(caps) = regex::Regex::new(pattern).ok().and_then(|re| re.captures(text)) {
            if let Some(marker) = caps.get(1) {
                let marker_str = marker.as_str().to_string();
                let remaining = text[marker.end()..].trim().to_string();
                return (remaining, Some(marker_str));
            }
        }
    }

    // Check for keyword markers: Must:, Should:, Could:, Won't:
    let keyword_patterns = [
        ("Must:", "MUST"),
        ("Should:", "SHOULD"),
        ("Could:", "COULD"),
        ("Won't:", "WONT"),
        ("Will:", "WILL"),
    ];

    for (prefix, marker) in &keyword_patterns {
        if text.starts_with(prefix) {
            let remaining = text[prefix.len()..].trim().to_string();
            return (remaining, Some(marker.to_string()));
        }
    }

    (text.to_string(), None)
}

fn extract_requirement_from_table_row(
    spec_id: &str,
    headers: &[String],
    row: &[String],
    heading_stack: &HeadingStack,
) -> Option<Requirement> {
    if row.is_empty() || headers.is_empty() {
        return None;
    }

    // Look for common requirement table patterns
    let mut requirement_text = None;
    let mut priority = String::from("medium");
    let mut req_type = String::from("functional");

    for (i, header) in headers.iter().enumerate() {
        if i >= row.len() {
            break;
        }

        let header_lower = header.to_lowercase();
        let cell_value = &row[i];

        if header_lower.contains("requirement")
            || header_lower.contains("description")
            || header_lower.contains("spec")
            || header_lower.contains("user story")
        {
            requirement_text = Some(cell_value.clone());
        } else if header_lower.contains("priority") {
            priority = match cell_value.to_lowercase().as_str() {
                "high" | "critical" | "must" => "high".to_string(),
                "low" | "nice to have" | "optional" => "low".to_string(),
                _ => "medium".to_string(),
            };
        } else if header_lower.contains("type") || header_lower.contains("category") {
            req_type = if cell_value.to_lowercase().contains("non-functional")
                || cell_value.to_lowercase().contains("performance")
                || cell_value.to_lowercase().contains("security")
            {
                "non_functional".to_string()
            } else if cell_value.to_lowercase().contains("constraint") {
                "constraint".to_string()
            } else {
                "functional".to_string()
            };
        }
    }

    // If no explicit requirement column, use first non-ID column
    if requirement_text.is_none() && !row.is_empty() {
        // Skip ID-like columns (REQ-001, etc.)
        for cell in row {
            if !cell.starts_with("REQ-")
                && !cell.starts_with("US-")
                && !cell.starts_with("FR-")
                && !cell.is_empty()
            {
                requirement_text = Some(cell.clone());
                break;
            }
        }
    }

    requirement_text.and_then(|text| {
        if text.is_empty() || text.len() < 10 {
            None
        } else {
            Some(Requirement {
                id: Uuid::new_v4().to_string(),
                spec_id: spec_id.to_string(),
                section: heading_stack.get_full_path(),
                description: text,
                req_type,
                priority,
            })
        }
    })
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
        || lower.starts_with("should ")
        || lower.starts_with("could ")
        || lower.starts_with("req-")
        || lower.starts_with("us-")
        || lower.starts_with("fr-")
        || lower.starts_with("nfr-")
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

    #[test]
    fn test_heading_hierarchy() {
        let content = r#"# Product Spec

## Features

### User Management

- Must allow user registration
- Should support OAuth login

### Data Management

- Must support CSV export
"#;
        let reqs = parse_spec("test-spec-id", content);
        assert_eq!(reqs.len(), 3);
        assert!(reqs[0].section.contains("Product Spec > Features > User Management"));
        assert!(reqs[2].section.contains("Product Spec > Features > Data Management"));
    }

    #[test]
    fn test_requirement_markers() {
        let content = r#"# Spec

## Requirements

- REQ-001: User must be able to login
- US-042: As a user, I want to logout
- FR-10: System shall validate email addresses
- Must: All passwords must be hashed
- Should: Add two-factor authentication
"#;
        let reqs = parse_spec("test-spec-id", content);
        assert_eq!(reqs.len(), 5);
        assert!(reqs[0].description.contains("[REQ-001]"));
        assert!(reqs[1].description.contains("[US-042]"));
        assert!(reqs[2].description.contains("[FR-10]"));
        assert!(reqs[3].description.contains("[MUST]"));
        assert!(reqs[4].description.contains("[SHOULD]"));
    }

    #[test]
    fn test_table_parsing() {
        let content = r#"# Requirements

## Feature Requirements

| ID | Requirement | Priority | Type |
|----|-------------|----------|------|
| REQ-001 | User authentication via email | High | Functional |
| REQ-002 | API response time < 200ms | Critical | Non-Functional |
| REQ-003 | Support CSV export | Medium | Functional |
"#;
        let reqs = parse_spec("test-spec-id", content);
        assert!(reqs.len() >= 3, "Expected at least 3 requirements from table");
        let descriptions: Vec<&str> = reqs.iter().map(|r| r.description.as_str()).collect();
        assert!(descriptions.iter().any(|d| d.contains("User authentication")));
        assert!(descriptions.iter().any(|d| d.contains("API response time")));
    }

    #[test]
    fn test_priority_classification() {
        let content = r#"# Spec

## Requirements

- Critical: Database backups must run daily
- Must have: User authentication
- Nice to have: Dark mode theme
- Optional: Export to PDF
- Regular requirement without keyword
"#;
        let reqs = parse_spec("test-spec-id", content);
        assert_eq!(reqs.len(), 5);
        assert_eq!(reqs[0].priority, "high");
        assert_eq!(reqs[1].priority, "high");
        assert_eq!(reqs[2].priority, "low");
        assert_eq!(reqs[3].priority, "low");
        assert_eq!(reqs[4].priority, "medium");
    }

    #[test]
    fn test_requirement_type_classification() {
        let content = r#"# Spec

## Functional Requirements

- User can create an account

## Non-Functional Requirements

- System shall handle 1000 concurrent users

## Performance Requirements

- API latency must be under 100ms

## Constraints

- Must comply with GDPR

## Security Requirements

- All data must be encrypted at rest
"#;
        let reqs = parse_spec("test-spec-id", content);
        assert!(reqs.len() >= 5);

        let functional = reqs.iter().find(|r| r.description.contains("create an account"));
        assert_eq!(functional.unwrap().req_type, "functional");

        let performance = reqs.iter().find(|r| r.description.contains("1000 concurrent"));
        assert_eq!(performance.unwrap().req_type, "non_functional");

        let latency = reqs.iter().find(|r| r.description.contains("latency"));
        assert_eq!(latency.unwrap().req_type, "non_functional");

        let constraint = reqs.iter().find(|r| r.description.contains("GDPR"));
        assert_eq!(constraint.unwrap().req_type, "constraint");

        let security = reqs.iter().find(|r| r.description.contains("encrypted"));
        assert_eq!(security.unwrap().req_type, "non_functional");
    }

    #[test]
    fn test_nested_hierarchy() {
        let content = r#"# Project

## Module A

### Feature 1

#### Sub-feature 1a

- Requirement at level 4

### Feature 2

- Requirement at level 3
"#;
        let reqs = parse_spec("test-spec-id", content);
        assert_eq!(reqs.len(), 2);
        assert!(reqs[0].section.contains("Module A > Feature 1 > Sub-feature 1a"));
        assert!(reqs[1].section.contains("Module A > Feature 2"));
    }

    #[test]
    fn test_empty_spec() {
        let content = "";
        let reqs = parse_spec("test-spec-id", content);
        assert_eq!(reqs.len(), 0);
    }

    #[test]
    fn test_no_requirements() {
        let content = r#"# Spec

This is just documentation with no requirements.

## Overview

Just text, no list items or tables.
"#;
        let reqs = parse_spec("test-spec-id", content);
        assert_eq!(reqs.len(), 0);
    }

    #[test]
    fn test_mixed_formats() {
        let content = r#"# Spec

## Requirements

- List item requirement

| ID | Description |
|----|-------------|
| R1 | Table requirement |

- REQ-001: Marked requirement
- Should: Keyword requirement
"#;
        let reqs = parse_spec("test-spec-id", content);
        assert!(reqs.len() >= 4, "Should extract from list, table, and markers");
    }
}
