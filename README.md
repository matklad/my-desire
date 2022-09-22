# My Desire

Small demonstration of generic type & injection safe interpolation without variadic generics.

TL;DR

```rust
pub use my_desire_macros::interpol;

pub trait TemplateString<V: TemplateVisitor> {

    fn accept(self) -> V::Output;
}

pub trait TemplateVisitor {
    type Output;
    fn new() -> Self;
    fn visit_str(&mut self, s: &'static str);
    fn finish(self) -> Self::Output;
}

pub trait TemplateVisit<T> {
    fn visit(&mut self, value: &T);
}

let affect = "frustrated"

let s = interpol!("I'm a {} man" as S);
// macro =>
let s = {
    // need to generate a unique per-invocation type, but doesn't need to be variadic.
    struct TS<'a, T0>((&'static str, &'static str), (&'a T0,));
    impl<'a, T0, V: my_desire::TemplateVisitor + my_desire::TemplateVisit<T0>>
        my_desire::TemplateString<V> for TS<'a, T0>
    {
        fn accept(self) -> V::Output {
            let mut v = V::new();
            v.visit_str(self.0 .0);
            v.visit(self.1 .0);
            v.visit_str(self.0 .1);
            v.finish()
        }
    }
    my_desire::TemplateString::<S>::accept(TS(("I'm a ", " man"), (&affect,)))
};


```
