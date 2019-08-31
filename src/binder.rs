use crate::parser::Task;
use pulldown_cmark::Event;
use std::collections::HashMap;

#[derive(Hash, Debug, Eq, PartialEq, Clone, Copy, PartialOrd, Ord)]
struct Id(usize);

struct IdGen {
    current: usize,
}

impl IdGen {
    fn new() -> IdGen {
        IdGen { current: 0 }
    }
    fn next(&mut self) -> Id {
        let n = self.current;
        self.current += 1;
        Id(n)
    }
}

#[derive(Debug)]
pub struct BoundTask<'a> {
    id: Id,
    name: Vec<Event<'a>>,
    description: Vec<Event<'a>>,
    depends_on: Vec<Id>,
}

impl<'a> BoundTask<'a> {
    fn from_task<'b>(id: Id, task: &Task<'b>) -> BoundTask<'b> {
        BoundTask {
            id,
            name: task.name.clone(),
            description: task.description.clone(),
            depends_on: vec![],
        }
    }
}

fn bind_chain<'a>(
    parent: &mut BoundTask<'a>,
    task: &Task<'a>,
    id_to_task: &mut HashMap<Id, BoundTask<'a>>,
    id_gen: &mut IdGen,
) {
    let id = id_gen.next();
    let mut bound_task = BoundTask::from_task(id, task);
    parent.depends_on.push(id);

    for child in &task.nested {
        bind_chain(&mut bound_task, child, id_to_task, id_gen);
    }

    id_to_task.insert(id, bound_task);
}

pub fn bind<'a>(tasks: &[Task<'a>]) -> Vec<BoundTask<'a>> {
    let mut id_gen = IdGen::new();
    let mut id_to_task = HashMap::new();
    for task in tasks {
        let id = id_gen.next();
        let mut bound_task = BoundTask::from_task(id, task);
        for child in &task.nested {
            bind_chain(&mut bound_task, child, &mut id_to_task, &mut id_gen);
        }
        id_to_task.insert(id, bound_task);
    }

   let mut all_tasks: Vec<_> = id_to_task.into_iter().map(|(_, a)|a).collect();
   all_tasks.sort_by_key(|task| task.id);
   all_tasks
}
