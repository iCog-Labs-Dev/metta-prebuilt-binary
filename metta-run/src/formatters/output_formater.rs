// use crate::runners;
use colored::Colorize;
use regex::Regex;

pub fn format(metta_output: (String, String)) {
    let mut indent_level = 0;
    let (metta_err, metta_str) = metta_output;

    // println!("this is working ++++++++++++++++++++++++++++++++ {}",);

    if !metta_err.is_empty() {
        if let Some(last_line) = metta_err.lines().last() {
            println!("{}", last_line.red());
        }
    }
    if !metta_str.is_empty() {
        selective_print(split_outputs(metta_str))
    }
}

fn selective_print(outputs: Vec<String>) {
    for output in outputs {
        let formatted = prettify(&output);
        let flag  = ".";
        if output.contains("(Error") {
            println!("{} [{}]",flag.dimmed().bold().blue(), formatted.red());
        } else {
            println!("{} [{}]",flag.dimmed().bold().blue(), formatted.green());
        }
    }
}

fn split_outputs(output: String) -> Vec<String> {
    // Define the regex pattern to match substrings in the form of [ SOMETHING]
    let re = Regex::new(r"\[([^\]]+)\]").unwrap();

    // Find all matches and collect them into a vector
    let matches: Vec<String> = re
        .captures_iter(&output)
        .map(|caps| caps.get(1).map_or("", |m| m.as_str()).to_string())
        .collect();

    return matches;
}

fn prettify(input: &str) -> String {
    let mut prettified = String::new();
    let mut indent_level = 0;

    for line in input.chars() {
        match line {
            '(' => {
                prettified.push(line);
                prettified.push('\n');
                indent_level += 2;
                prettified.push_str(&" ".repeat(indent_level));
            }
            ')' => {
                prettified.push('\n');
                indent_level -= 2;
                prettified.push_str(&" ".repeat(indent_level));
                prettified.push(line);
            }
            ';' => {
                prettified.push('\n');
                prettified.push_str(&" ".repeat(indent_level));
                prettified.push(line);
            }
            ' ' if prettified.ends_with('\n') => {
                prettified.push_str(&" ".repeat(indent_level));
            }
            _ => {
                prettified.push(line);
            }
        }
    }

    prettified
}