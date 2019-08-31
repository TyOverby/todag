use crate::binder::*;
use crate::parser::*;

fn run_bind<'a>(s: &'a str) -> Vec<BoundTask<'a>> {
    let (tasks, _headers) = extract(s);
    bind(&tasks[..])
}

#[test]
fn single_task() {
    let bound = run_bind(r#"
- [ ] this is the title 
  this is the description
"#);

    insta::assert_debug_snapshot!(bound);
}

#[test]
fn two_tasks_nested() {
    let bound = run_bind(r#"
- [ ] Title of A
  - [ ] Title of B
"#);

    insta::assert_debug_snapshot!(bound);
}

#[test]
fn two_tasks() {
    let bound = run_bind(r#"
- [ ] Title of A
- [ ] Title of B
"#);

    insta::assert_debug_snapshot!(bound);
}
