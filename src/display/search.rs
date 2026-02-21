use crate::api::types::SearchResult;
use crate::display::utils::get_term_width;
use colored::*;

pub fn display_search_result(result: &SearchResult, index: usize) {
    let width = get_term_width();
    let inner_width = width.saturating_sub(4);

    println!(
        "{}",
        format!("╭{}╮", "─".repeat(width.saturating_sub(2))).dimmed()
    );

    let title = result.title.as_deref().unwrap_or("(comment)");
    let score = result.similarity.unwrap_or(0.0);
    let score_display = if score > 1.0 {
        format!("{:.1}", score)
    } else {
        format!("{:.0}%", score * 100.0)
    };

    let title_space = inner_width.saturating_sub(score_display.chars().count() + 6); // #1 + space + space + score
    let title_display = if title.chars().count() > title_space {
        let t: String = title.chars().take(title_space.saturating_sub(3)).collect();
        format!("{}...", t)
    } else {
        title.to_string()
    };

    let padding = inner_width
        .saturating_sub(4 + title_display.chars().count() + score_display.chars().count());
    println!(
        "│ #{:<2} {}{:>p$} │",
        index,
        title_display.bright_cyan().bold(),
        score_display.green(),
        p = padding + score_display.chars().count()
    );

    println!(
        "{}",
        format!("├{}┤", "─".repeat(width.saturating_sub(2))).dimmed()
    );

    let author = result.author.name.yellow();
    let type_label = result.result_type.blue();

    let left_len = result.author.name.chars().count() + result.result_type.chars().count() + 8;
    let meta_padding = inner_width.saturating_sub(left_len);

    println!(
        "│ 👤 {}  •  {}{:>p$} │",
        author,
        type_label,
        "",
        p = meta_padding
    );

    println!("│ {:>w$} │", "", w = inner_width);
    if let Some(content) = &result.content {
        let wrapped_width = inner_width.saturating_sub(2);
        let wrapped = textwrap::fill(content, wrapped_width);
        for (i, line) in wrapped.lines().enumerate() {
            if i >= 3 {
                println!("│  {: <w$} │", "...".dimmed(), w = wrapped_width);
                break;
            }
            println!("│  {:<w$}│", line, w = wrapped_width);
        }
    }

    println!(
        "{}",
        format!("╰{}╯", "─".repeat(width.saturating_sub(2))).dimmed()
    );
    if let Some(post_id) = &result.post_id {
        println!("   Post ID: {}", post_id.dimmed());
    }
    println!();
}
