use itertools::{put_back, PutBack};
use pulldown_cmark::{Event, Options, Parser, Tag};
use Event::*;

#[derive(Debug)]
pub struct Task<'a> {
    pub checked: bool,
    pub path: Vec<String>,
    pub name: Vec<Event<'a>>,
    pub description: Vec<Event<'a>>,
    pub nested: Vec<Task<'a>>,
}

#[derive(Debug)]
pub struct Header {
    depth: i32,
    text: String,
}

#[derive(Copy, Clone)]
enum TaskParseState {
    WaitingForTaskListMarker,
    ParsingTitle,
    ParsingDescription,
}

use TaskParseState::*;

fn parse_tasks<'a>(out: &mut Vec<Task<'a>>, paths: &[Header], parser: &mut PutBack<Parser<'a>>) {
    while let Some(event) = parser.next() {
        match event {
            Start(Tag::Item) => out.push(parse_task(paths, parser)),
            End(Tag::List(_)) => break,
            other => eprintln!("{:?} not implemented", other),
        }
    }
}

fn parse_task<'a>(paths: &[Header], parser: &mut PutBack<Parser<'a>>) -> Task<'a> {
    let mut checked = false;
    let mut name = Vec::new();
    let mut description = Vec::new();
    let path = paths.iter().map(|header| &header.text).cloned().collect();
    let mut state = WaitingForTaskListMarker;
    let mut nested = vec![];
    while let Some(event) = parser.next() {
        match (event, state) {
            (TaskListMarker(c), WaitingForTaskListMarker) => {
                checked = c;
                state = ParsingTitle;
            }
            (SoftBreak, ParsingTitle) => state = ParsingDescription,
            (event @ End(Tag::Item), ParsingTitle) => {
                parser.put_back(event);
                state = ParsingDescription;
            }
            (Start(Tag::List(_)), ParsingTitle) | 
            (Start(Tag::List(_)), ParsingDescription)=> {
                parse_tasks(&mut nested, paths, parser);
                state = ParsingDescription;
            }
            (event, ParsingTitle) => name.push(event),
            (event @ End(Tag::Item), ParsingDescription) => {
                parser.put_back(event);
                break;
            }
            (event, ParsingDescription) => description.push(event),
            _ => break,
        }
    }
    Task {
        path,
        name,
        checked,
        description,
        nested,
    }
}

fn parse_header(depth: i32, parser: &mut PutBack<Parser>) -> Header {
    let mut text = String::new();
    while let Some(event) = parser.next() {
        match event {
            Event::Text(t) => text.push_str(&t),
            Event::End(Tag::Header(_)) => break,
            _ => break,
        }
    }
    Header { depth, text }
}

pub fn extract(s: &str) -> (Vec<Task>, Vec<Header>) {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TASKLISTS);

    let mut parser = put_back(Parser::new_ext(s, options));
    let mut out = Vec::new();
    let mut path_stack: Vec<Header> = Vec::new();
    while let Some(event) = parser.next() {
        match event {
            Event::Start(Tag::Header(i)) => {
                let header = parse_header(i, &mut parser);
                while let Some(top_header) = path_stack.pop() {
                    if top_header.depth < header.depth {
                        path_stack.push(top_header);
                        break;
                    }
                }
                path_stack.push(header);
            }
            Event::Start(Tag::List(None)) => {
                parse_tasks(&mut out, &path_stack, &mut parser);
            }
            other => {
                eprintln!("{:?} not implemented", other);
            }
        }
    }

    (out, path_stack)
}
