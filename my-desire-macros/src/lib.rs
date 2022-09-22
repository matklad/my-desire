use proc_macro::{Delimiter, Literal, TokenStream, TokenTree};

#[proc_macro]
pub fn interpol(macro_arg: TokenStream) -> TokenStream {
    let args = macro_arg.into_iter().collect::<Vec<_>>();
    let (l, type_) = match args.as_slice() {
        [l] => (l, None),
        [l, as_, t] if as_.to_string() == "as" => (l, Some(t)),
        _ => panic!(r#"expected syntax: interpol!("template {{string}}" as Type)"#),
    };

    let l = match into_literal(&l) {
        Some(it) => it,
        None => panic!("expected a plain string l"),
    };

    let l = l.to_string();
    if !l.starts_with('"') {
        panic!("expected a plain string literal");
    }
    let mut l = strip_matches(&l, "\"");
    let mut fragments = Vec::new();
    let mut values = Vec::new();
    loop {
        match l.find('{') {
            Some(i) => {
                fragments.push(&l[..i]);
                l = &l[i + 1..];
                let j = l.find('}').unwrap();
                values.push(&l[..j]);
                l = &l[j + 1..];
            }
            None => {
                fragments.push(l);
                break;
            }
        }
    }

    // let buf = r#"{
    //     struct TS<'a, T0>((&'static str, &'static str), (&'a T0,));
    //     impl<'a, T0, V: my_desire::TemplateVisitor + my_desire::TemplateVisit<T0>>
    //         my_desire::TemplateString<V> for TS<'a, T0>
    //     {
    //         fn accept(self) -> V::Output {
    //             let mut v = V::new();
    //             v.visit_str(self.0 .0);
    //             v.visit(self.1 .0);
    //             v.visit_str(self.0 .1);
    //             v.finish()
    //         }
    //     }
    //     my_desire::TemplateString::<S>::accept(TS(("I'm a ", " man"), (&affect,)))
    // }"#;
    let mut p1 = String::new();
    let mut s = String::new();
    let mut p2 = String::new();
    let mut b = String::new();
    let mut visits = String::new();
    let mut f = String::new();
    let mut v = String::new();

    for i in 0..values.len() {
        p1.push_str(&format!("T{i}, "));
        p2.push_str(&format!("&'a T{i}, "));
        b.push_str(&format!(" + my_desire::TemplateVisit<T{i}>"));
        visits.push_str(&format!("\nv.visit(self.1 .{i});"));
        visits.push_str(&format!("\nv.visit_str(self.0 .{});", i + 1));
    }

    for _ in 0..fragments.len() {
        s.push_str(&format!("&'static str, "));
    }

    for frag in fragments {
        f.push_str(&format!("\"{}\", ", frag))
    }

    for val in values {
        v.push_str(&format!("&{val}, "))
    }

    let mut buf = format!(
        "{{
        struct TS<'a, {p1}>(({s}), ({p2}));
        impl<'a, {p1} V: my_desire::TemplateVisitor {b}>
            my_desire::TemplateString<V> for TS<'a, {p1}>
        {{
            fn accept(self) -> V::Output {{
                let mut v = V::new();
                v.visit_str(self.0 .0);
                {visits}
                v.finish()
            }}
        }}
        let ts = TS(({f}), ({v}));
    "
    );

    buf.push_str(&match type_ {
        Some(type_) => format!("my_desire::TemplateString::<{type_}>::accept(ts)}}"),
        None => "ts}".to_string(),
    });

    // eprintln!("{}", buf);

    buf.parse().unwrap()
}

fn into_literal(ts: &TokenTree) -> Option<Literal> {
    match ts {
        TokenTree::Literal(l) => Some(l.clone()),
        TokenTree::Group(g) => match g.delimiter() {
            Delimiter::None => match g.stream().into_iter().collect::<Vec<_>>().as_slice() {
                [TokenTree::Literal(l)] => Some(l.clone()),
                _ => None,
            },
            Delimiter::Parenthesis | Delimiter::Brace | Delimiter::Bracket => None,
        },
        _ => None,
    }
}

fn strip_matches<'a>(s: &'a str, pattern: &str) -> &'a str {
    s.strip_prefix(pattern)
        .unwrap_or(s)
        .strip_suffix(pattern)
        .unwrap_or(s)
}
