use std::collections::HashMap;
use std::io::{stdin, stdout, Error, ErrorKind};
use std::io::Write;
use std::env;
use std::fs;
use std::io;
use std::fmt;
use std::process::{Command, Output};
use std::str::from_utf8;

use termion::{clear, cursor};
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;

const MAX_SUGGESTIONS: usize = 5;


fn main() {
    suggesting_start();
}

fn suggesting_start() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let     stdin = stdin();

    let prompt = "> ";

    write!(stdout, "{}", prompt).unwrap();

    stdout.flush().unwrap();

    let mut suggester: Suggester = Default::default();
    suggester.suggestions = vec![
        "hello there 1".to_string(),
        "hsdfs 2".to_string(),
        "my name jeff 3".to_string(),
        "xdxdxd 4".to_string(),
        "ok there 5".to_string(),
    ];

    for key in stdin.keys() {
        match key.unwrap() {
            Key::Char('\n') => { },
            Key::Ctrl('c') => { write!(stdout, "^C\r\n").unwrap(); break; }
            Key::Down => suggestion_down(&mut suggester),
            Key::Backspace => {},
            Key::Char(key) => insert_char(&mut suggester, key),
            _ => {},
        }

        render(&mut suggester, &prompt, &mut stdout).unwrap();
        stdout.flush().unwrap();
    }



}

#[derive(Default)]
struct Suggester {
    buffer: Vec<char>,
    buffer_cursor: usize,
    // prompt_display: Vec<char>,
    suggestions: Vec<String>,
    suggestion_cursor: usize,
}

fn suggestion_down(suggester: &mut Suggester) {
    if suggester.suggestion_cursor >= suggester.suggestions.len() - 1 { return }

    suggester.suggestion_cursor += 1;

    // suggester.buffer = suggester.suggestions[suggester.suggestion_cursor].chars().collect();
    // suggester.buffer_cursor = suggester.buffer.len();


    // suggester.prompt_display = if suggester.suggestion_cursor > 0 { suggester.suggestions[suggester.suggestion_cursor].chars().collect() } else { suggester.buffer.clone() };
}

fn insert_char(suggester: &mut Suggester, x: char) {
    let idx_suggestion = suggester.suggestion_cursor;

    if idx_suggestion > 0 {
        suggester.buffer = suggester.suggestions[suggester.suggestion_cursor - 1].chars().collect();
        suggester.suggestion_cursor = 0;

        suggester.buffer_cursor = suggester.buffer.len();
    }

    let idx = suggester.buffer_cursor;

    suggester.buffer.insert(idx,  x);
    suggester.buffer_cursor += 1;

    // suggester.prompt_display = suggester.buffer.clone();
}

fn render(suggester: &Suggester, prompt: &str, sink: &mut impl Write) -> io::Result<()> {
    let current: String = suggester.buffer.iter().collect();
    // let display: String = suggester.prompt_display.iter().collect();

    // IDK if this is indexing correctly
    let display = if suggester.suggestion_cursor == 0 { current.clone() } else { suggester.suggestions[suggester.suggestion_cursor - 1].clone() };

    write!(sink, "\r{}{}{}\r\n", clear::AfterCursor, prompt, &display).unwrap();

    

    // let mut prelines = String::new();
    // if suggester.suggestion_cursor == 0 {
    //     prelines += "> ";
    // }
    // prelines += &current;
    // write!(sink, "{}\r\n", prelines)?;
    


    // let base = vec![current];
    // let idk: Vec<String> = base.into_iter().chain(suggester.suggestions.into_iter()).collect();

    let mut suggestions: Vec<&String> = vec![&current];

    let base = &suggester.suggestions;

    suggestions.extend(base.iter());




    for (idx, line) in suggestions.iter().take(MAX_SUGGESTIONS).enumerate() {
        let selected = idx == suggester.suggestion_cursor;
        if selected { write!(sink, "> ")? }
        write!(sink, "{}\r\n", line)?;
    }


    let right = if suggester.suggestion_cursor == 0 { prompt.len() + suggester.buffer_cursor} else { prompt.len() + display.len() };

    write!(sink, "{}{}",
        cursor::Up((MAX_SUGGESTIONS.min(suggestions.len()) + 1).try_into().unwrap()),
        cursor::Right((right).try_into().unwrap()))?;

    Ok(())
}


fn get_command_result(command: &str) -> Result<String, Error> {
    let result = Command::new("sh")
        .args(["-c", command])
        .output();
    
    match result {
        Result::Err(err) => Result::Err(err),
        Result::Ok(output) => handle_output(output),
    }
}

fn handle_output(output: Output) -> Result<String, Error> {
    if let Some(code) = output.status.code() {
        let stdout = from_utf8(output.stdout.as_slice()).unwrap().to_string();
        let stderr = from_utf8(output.stderr.as_slice()).unwrap().to_string();
        if code != 0 {
            Result::Err(Error::new(ErrorKind::Other, stderr))
        } else if stderr != "" {
            Result::Err(Error::new(ErrorKind::Other, stderr))
        } else {
            Result::Ok(stdout)
        }
    } else {
        Result::Err(Error::new(ErrorKind::InvalidData, "No command status code"))
    }
}
