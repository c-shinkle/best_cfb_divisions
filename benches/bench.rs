#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use best_cfb_divisions::find_closest_divisions;
    use test::Bencher;

    #[bench]
    fn test(b: &mut Bencher) {
        b.iter(|| {
            find_closest_divisions();
        });
    }
}
