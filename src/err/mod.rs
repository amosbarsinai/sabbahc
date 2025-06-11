use terminal_size::{Height, Width, terminal_size};

pub struct Diagnostic {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub suggestion: Option<String>,
}

impl Diagnostic {
    pub fn out(self, source_code: &String) {
        // ANSI color codes
        const RED: &str = "\x1b[31m";
        const BOLD: &str = "\x1b[1m";
        const RESET: &str = "\x1b[0m";

        eprintln!(
            "{}{}error:{} {} at {}:{}:{}",
            BOLD, RED, RESET, self.message, self.file, self.line, self.column
        );

        render_snippet(source_code, (self.line, self.column));

        if let Some(suggestion) = self.suggestion {
            println!("{}fix:{} {}", BOLD, RESET, suggestion);
        }
    }
}

fn get_terminal_width() -> usize {
    if let Some((Width(w), Height(_))) = terminal_size() {
        w as usize
    } else {
        80 // fallback width if we can't get terminal size
    }
}

fn digit_count(num: i32) -> usize {
    if num == 0 {
        return 1;
    }
    let mut n = num.abs();
    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1;
    }
    if num < 0 {
        count += 1;
    }
    count
}

fn render_snippet(source_code: &String, problem: (usize, usize)) {
    let width = get_terminal_width();
    let lines: Vec<&str> = source_code.lines().collect();
    let num_lines = lines.len();
    let line_num_width = digit_count(num_lines as i32);
    let start = problem.0.saturating_sub(3);
    let end = (problem.0 + 2).min(num_lines);

    println!(
        "{}┼{}─",
        "─".repeat(line_num_width + 1),
        "─".repeat(width - line_num_width - 4)
    );

    if start > 0 {
        println!("{} │ ...", " ".repeat(line_num_width));
    }

    for i in start..end {
        let mut line = lines[i].to_string();
        if line.len() > width - line_num_width - 4 {
            line.truncate(width - line_num_width - 4);
            line.push_str("...");
        }
        println!("{:>width$} │ {}", i + 1, line, width = line_num_width);
        if i + 1 == problem.0 {
            println!(
                "{} │ {}{}",
                " ".repeat(line_num_width),
                " ".repeat(problem.1 - 2),
                "^"
            );
        }
    }

    if end < num_lines {
        println!("{} │ ...", " ".repeat(line_num_width));
    }
}
