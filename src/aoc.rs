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

pub trait Vec2<T> {
    fn index(&self, x: i64, y: i64) -> Option<&T>;
    fn indexp(&self, (x,y): (i64, i64)) -> Option<&T> {
        self.index(x, y)
    }
}

impl<T> Vec2<T> for Vec<Vec<T>> {
    fn index(&self, x: i64, y: i64) -> Option<&T> {
        match self.get(y as usize) {
            None => None,
            Some(row) => row.get(x as usize)
        }
    }
}