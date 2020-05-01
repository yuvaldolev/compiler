use std::rc::Rc;

pub struct Ast {}

pub struct Scope {
    parent: Option<Rc<Scope>>,

    stmts: Vec<Rc<Ast>>,
    decls: Vec<Rc<Ast>>,

    // NOTE(ydolev): One of the following:
    // struct / enum / function / statement.
    owner: Option<Rc<Ast>>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            parent: None,
            stmts: Vec::new(),
            decls: Vec::new(),
            owner: None,
        }
    }
}
