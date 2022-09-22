use std::fmt::{self, Write};

use my_desire::{interpol, TemplateString, TemplateVisit, TemplateVisitor};

#[test]
fn string() {
    let affect = "frustrated";
    let s = interpol!("I'm a {affect} man" as S);
    assert_eq!(s, "I'm a frustrated man");
}

#[test]
fn shell_escape() {
    let branch = "lol; echo pwned!";
    let s = interpol!("git switch {branch}" as ShellEscape);
    assert_eq!(s, r#"git switch "lol; echo pwned!""#);
}

#[test]
fn sql() {
    let x = 92;
    let y = false;
    let q = interpol!("select '*' from Table where x == {x} and y == {y}" as Sql);
    assert_eq!(
        q,
        Sql {
            query: "select '*' from Table where x == {} and y == {}".to_string(),
            params: vec![SqlParam::Int(92), SqlParam::Bool(false)]
        }
    );
    let q = interpol!("select '*' from Table where x == {x} and y == {y}");
    accepts_sql_template_without_naming_its_type(q);
}

fn accepts_sql_template_without_naming_its_type(_: impl TemplateString<Sql>) {}

struct S {
    buf: String,
}

impl TemplateVisitor for S {
    type Output = String;

    fn new() -> Self {
        S { buf: String::new() }
    }

    fn visit_str(&mut self, s: &'static str) {
        self.buf.push_str(s)
    }

    fn finish(self) -> Self::Output {
        self.buf
    }
}

impl<T: fmt::Display> TemplateVisit<T> for S {
    fn visit(&mut self, value: &T) {
        let _ = write!(self.buf, "{value}");
    }
}

struct ShellEscape {
    buf: String,
}

impl TemplateVisitor for ShellEscape {
    type Output = String;

    fn new() -> Self {
        ShellEscape { buf: String::new() }
    }

    fn visit_str(&mut self, s: &'static str) {
        self.buf.push_str(s)
    }

    fn finish(self) -> Self::Output {
        self.buf
    }
}

impl<T: fmt::Display> TemplateVisit<T> for ShellEscape {
    fn visit(&mut self, value: &T) {
        let value = value.to_string();
        if value.contains(" ") {
            let _ = write!(self.buf, "{value:?}");
        } else {
            let _ = write!(self.buf, "{value}");
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Sql {
    query: String,
    params: Vec<SqlParam>,
}

#[derive(PartialEq, Eq, Debug)]
enum SqlParam {
    Int(i32),
    Bool(bool),
}

impl TemplateVisitor for Sql {
    type Output = Sql;

    fn new() -> Self {
        Sql {
            query: String::new(),
            params: Vec::new(),
        }
    }

    fn visit_str(&mut self, s: &'static str) {
        self.query.push_str(s)
    }

    fn finish(self) -> Self::Output {
        self
    }
}

impl TemplateVisit<i32> for Sql {
    fn visit(&mut self, value: &i32) {
        self.query.push_str("{}");
        self.params.push(SqlParam::Int(*value));
    }
}

impl TemplateVisit<bool> for Sql {
    fn visit(&mut self, value: &bool) {
        self.query.push_str("{}");
        self.params.push(SqlParam::Bool(*value));
    }
}
