use crate::parser::extract;

mod debug_output {
    use pulldown_cmark::{Options, Parser};

    #[test]
    fn single_tasks_debugout() {
        use pulldown_cmark::{Options, Parser};
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TASKLISTS);
        let input = r#"
- [ ] this is the title 
      this is the description
"#;
        let v: Vec<_> = Parser::new_ext(input, options).collect();

        insta::assert_debug_snapshot!(v);
    }

    #[test]
    fn nested_tasks_with_weird_indentation_test() {
        use pulldown_cmark::{Options, Parser};
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TASKLISTS);
    let input = r#"
- [ ] title of parent
  description of parent
  - [ ] title of child
    description of child
  more description of parent
"#;
        let v: Vec<_> = Parser::new_ext(input, options).collect();

        insta::assert_debug_snapshot!(v);
    }


    #[test]
    fn nested_tasks_debugout() {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TASKLISTS);
        let input = r#"
- [ ] a 
  - [ ] b
    - [ ] c 
  - [ ] d
"#;
        let v: Vec<_> = Parser::new_ext(input, options).collect();

        insta::assert_debug_snapshot!(v);
    }

    #[test]
    fn two_tasks_debugout() {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TASKLISTS);
        let input = r#"
- [ ] a name
      a description
- [ ] b name
      b description
"#;
        let v: Vec<_> = Parser::new_ext(input, options).collect();
        insta::assert_debug_snapshot!(v);
    }
}

#[test]
fn nested_task() {
    let input = r#"
- [ ] title of parent
  - [ ] title of child
"#;
    let (tasks, _headers) = extract(input);
    insta::assert_debug_snapshot!(tasks);
}

#[test]
fn nested_tasks_with_descriptions() {
    let input = r#"
- [ ] title of parent
      description of parent
  - [ ] title of child
        description of child
"#;
    let (tasks, _headers) = extract(input);
    insta::assert_debug_snapshot!(tasks);
}

#[test]
fn deeply_nested_task() {
    let input = r#"
- [ ] a 
  - [ ] b
    - [ ] c 
  - [ ] d
"#;
    let (tasks, _headers) = extract(input);
    insta::assert_debug_snapshot!(tasks);
}

#[test]
fn single_checkbox_with_title_unchecked() {
    let input = r#"
- [ ] this is the title"#;
    let (tasks, _headers) = extract(input);
    insta::assert_debug_snapshot!(tasks);
}

#[test]
fn two_checkbox_with_description() {
    let input = r#"
- [ ] a name
      a description
- [ ] b name
      b description
"#;
    let (tasks, _headers) = extract(input);
    insta::assert_debug_snapshot!(tasks);
}

#[test]
fn two_checkbox_without_description() {
    let input = r#"
- [ ] a name
- [ ] b name
"#;
    let (tasks, _headers) = extract(input);
    insta::assert_debug_snapshot!(tasks);
}

#[test]
fn single_checkbox_with_description() {
    let input = r#"
- [ ] this is the title
      and this is the description
"#;
    let (tasks, _headers) = extract(input);
    insta::assert_debug_snapshot!(tasks);
}

#[test]
fn single_checkbox_with_title_checked() {
    let input = r#"
- [x] this is the title"#;
    let (tasks, _headers) = extract(input);
    insta::assert_debug_snapshot!(tasks);
}

#[test]
fn single_header() {
    let input = r#"
# a "#;
    let (_tasks, headers) = extract(input);
    insta::assert_debug_snapshot!(headers);
}

#[test]
fn task_under_header() {
    let input = r#"
# Header Name
- [ ] Task Name
"#;
    let (tasks, _headers) = extract(input);
    insta::assert_debug_snapshot!(tasks);
}

#[test]
fn chained_header() {
    let input = r#"
# a
## b
### c "#;
    let (_tasks, headers) = extract(input);
    insta::assert_debug_snapshot!(headers);
}
