pub struct Stuff {
    n: u64,
    v: String,
}

impl Stuff {
    pub fn new() -> Self {
        Self {
            n: 0,
            v: "things".to_owned(),
        }
    }

    pub fn with_n(mut self, n: u64) -> Self {
        self.n = n;
        self
    }

    pub fn output(&self) -> String {
        format!("{} {}", self.n, self.v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = Stuff::new().with_n(42);
        assert_eq!(s.output(), "42 things");
    }
}
