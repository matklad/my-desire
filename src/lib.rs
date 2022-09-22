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
