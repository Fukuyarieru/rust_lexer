#[macro_export]
macro_rules! arc {
    ($($x:expr),*) => {{
        std::sync::Arc::<[_]>::from(vec![$($x),*])
    }};
}
