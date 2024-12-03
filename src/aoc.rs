use regex::Match;

pub trait ToI64 {
    fn toi64(&self) -> i64;
}
impl ToI64 for str {
    fn toi64(&self) -> i64 {
        self.parse().unwrap()
    }
}
impl ToI64 for Match<'_> {
    fn toi64(&self) -> i64 {
        self.as_str().toi64()
    }
}
impl ToI64 for usize {
    fn toi64(&self) -> i64 {
        i64::try_from(*self).unwrap()
    }
}