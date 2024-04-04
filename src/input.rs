pub trait U: Sized {
    fn f(s: &str) -> Option<Self>;
}

macro_rules! f {
    (($y:ident, $($x:ident), *)) => {
        f!(($($x), *));
        impl<$y: U, $($x: U), *> U for ($y, $($x), *) {
            fn f(s: &str) -> Option<Self> {
                let mut iter = s.split_whitespace();
                Some((
                    <$y as U>::f(&iter.next()?)?,
                    $(
                        <$x as U>::f(&iter.next()?)?
                    ), *,
                ))
            }
        }
    };
    ((H)) => {};
    ($($x:ty) *) => {
        $(
            impl U for $x {
                fn f(s: &str) -> Option<Self> { s.parse().ok() }
            }
        )*
    };
}

f!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize char String);
f!((A, B, C, D, E, F, G, H));

pub fn cin<F: U>() -> F {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    F::f(s.trim()).unwrap()
}
