pub fn indent_string(s: &str) -> String {
    format!("  {}", s.replace("\n", "\n  "))
}

pub fn get_position_in_content(content: &str, location: usize) -> (usize, usize) {
    let mut i = 0;
    let mut line_number = 0;
    for (j, line) in content.split("\n").enumerate() {
        line_number = j;
        let start_of_next_line = i + line.len() + 1;
        if start_of_next_line > location {
            break;
        }
        i = start_of_next_line;
    }

    let character_position = content[i..]
        .char_indices()
        .position(|(j, _)| j == location - i)
        .unwrap_or(0);

    (line_number, character_position)
}

fn arrow_to(character_position: usize) -> Vec<String> {
    let mut lines = vec![];
    lines.push(format!("          {}Λ", " ".repeat(character_position + 1)));
    lines.push(format!("          {}│", " ".repeat(character_position + 1)));
    lines.push(format!("  here ───{}┘", "─".repeat(character_position + 1)));

    lines
}

pub fn pretty_print_file_error(content: &str, location: usize, message: &str) -> String {
    let (line_number, character_position) = get_position_in_content(content, location);

    let offset = 5;
    let all_lines: Vec<&str> = content.split("\n").collect();

    let min_line = line_number.saturating_sub(offset);
    let max_line = if line_number + offset <= all_lines.len() {
        line_number + offset
    } else {
        all_lines.len()
    };

    let mut the_lines: Vec<String> = vec!["=============================".to_string()];
    for (i, line) in all_lines[min_line..max_line].iter().enumerate() {
        if i == line_number - min_line {
            the_lines.push(format!("{: >10}|{line}", i + min_line + 1));
            the_lines.extend(arrow_to(character_position));
        } else {
            the_lines.push(format!("{: >10}|{line}", i + min_line + 1));
        }
    }
    the_lines.push("=============================".to_string());
    the_lines.push(message.to_string());

    the_lines.join("\n")
}
