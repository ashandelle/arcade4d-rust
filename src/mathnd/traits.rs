pub trait Sqrt{
    fn sqrt(self) -> Self;
}

macro_rules! impl_sqrt {
    ( $($ty:ty),* ) => {
        $(
            impl Sqrt for $ty {
                fn sqrt(self) -> Self {
                    self.sqrt()
                }
            }
        )*
    };
}

impl_sqrt!(f32, f64);

pub trait Abs{
    fn abs(self) -> Self;
}

macro_rules! impl_abs {
    ( $($ty:ty),* ) => {
        $(
            impl Abs for $ty {
                fn abs(self) -> Self {
                    self.abs()
                }
            }
        )*
    };
}

impl_abs!(
    isize,
    i8, i16, i32, i64,
    f32, f64
);

pub trait Signum{
    fn signum(self) -> Self;
}

macro_rules! impl_signum {
    ( $($ty:ty),* ) => {
        $(
            impl Signum for $ty {
                fn signum(self) -> Self {
                    self.signum()
                }
            }
        )*
    };
}

impl_signum!(
    isize,
    i8, i16, i32, i64,
    f32, f64
);

// pub trait Round{
//     fn round(self) -> Self;
// }

// macro_rules! impl_round {
//     ( $($ty:ty),* ) => {
//         $(
//             impl Round for $ty {
//                 fn round(self) -> Self {
//                     self.round()
//                 }
//             }
//         )*
//     };
// }

// impl_round!(
//     f32, f64
// );

pub trait MinMax{
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
}

macro_rules! impl_minmax {
    ( $($ty:ty),* ) => {
        $(
            impl MinMax for $ty {
                fn min(self, other: Self) -> Self {
                    std::cmp::min(self, other)
                }

                fn max(self, other: Self) -> Self {
                    std::cmp::max(self, other)
                }
            }
        )*
    };
}

macro_rules! impl_minmaxf {
    ( $($ty:ty),* ) => {
        $(
            impl MinMax for $ty {
                fn min(self, other: Self) -> Self {
                    self.min(other)
                }

                fn max(self, other: Self) -> Self {
                    self.max(other)
                }
            }
        )*
    };
}

impl_minmax!(
    isize, usize,
    i8, i16, i32, i64,
    u8, u16, u32, u64
);

impl_minmaxf!(f32, f64);

pub trait Zero{
    fn zero() -> Self;
}

macro_rules! impl_zero {
    ( $( ($ty:ty, $zero:expr) ),* ) => {
        $(
            impl Zero for $ty {
                fn zero() -> Self {
                    $zero
                }
            }
        )*
    };
}

impl_zero!(
    (isize, 0isize),
    (usize, 0usize),
    (i8, 0i8),
    (i16, 0i16),
    (i32, 0i32),
    (i64, 0i64),
    (u8, 0u8),
    (u16, 0u16),
    (u32, 0u32),
    (u64, 0u64),
    (f32, 0f32),
    (f64, 0f64)
);

pub trait One{
    fn one() -> Self;
}

macro_rules! impl_one {
    ( $( ($ty:ty, $one:expr) ),* ) => {
        $(
            impl One for $ty {
                fn one() -> Self {
                    $one
                }
            }
        )*
    };
}

impl_one!(
    (isize, 1isize),
    (usize, 1usize),
    (i8, 1i8),
    (i16, 1i16),
    (i32, 1i32),
    (i64, 1i64),
    (u8, 1u8),
    (u16, 1u16),
    (u32, 1u32),
    (u64, 1u64),
    (f32, 1f32),
    (f64, 1f64)
);

pub trait Two{
    fn two() -> Self;
}

macro_rules! impl_two {
    ( $( ($ty:ty, $two:expr) ),* ) => {
        $(
            impl Two for $ty {
                fn two() -> Self {
                    $two
                }
            }
        )*
    };
}

impl_two!(
    (isize, 2isize),
    (usize, 2usize),
    (i8, 2i8),
    (i16, 2i16),
    (i32, 2i32),
    (i64, 2i64),
    (u8, 2u8),
    (u16, 2u16),
    (u32, 2u32),
    (u64, 2u64),
    (f32, 2f32),
    (f64, 2f64)
);

pub trait MinMaxValue{
    fn minimum() -> Self;
    fn maximum() -> Self;
}

macro_rules! impl_minmaxvalue {
    ( $( $ty:ty ),* ) => {
        $(
            impl MinMaxValue for $ty {
                fn minimum() -> Self {
                    Self::MIN
                }

                fn maximum() -> Self {
                    Self::MAX
                }
            }
        )*
    };
}

impl_minmaxvalue!(
    isize, usize,
    i8, i16, i32, i64,
    u8, u16, u32, u64,
    f32, f64
);

pub trait FromUsize{
    fn fromusize(n: usize) -> Self;
}

macro_rules! impl_fromusize {
    ( $($ty:ty),* ) => {
        $(
            impl FromUsize for $ty {
                fn fromusize(n: usize) -> Self {
                    n as $ty
                }
            }
        )*
    };
}

impl_fromusize!(
    isize, usize,
    i8, i16, i32, i64,
    u8, u16, u32, u64,
    f32, f64
);

pub trait FromFloat32{
    fn fromf32(n: f32) -> Self;
}

macro_rules! impl_fromf32 {
    ( $($ty:ty),* ) => {
        $(
            impl FromFloat32 for $ty {
                fn fromf32(n: f32) -> Self {
                    n as $ty
                }
            }
        )*
    };
}

impl_fromf32!(
    isize, usize,
    i8, i16, i32, i64,
    u8, u16, u32, u64,
    f32, f64
);

pub trait FromFloat64{
    fn fromf64(n: f64) -> Self;
}

macro_rules! impl_fromf64 {
    ( $($ty:ty),* ) => {
        $(
            impl FromFloat64 for $ty {
                fn fromf64(n: f64) -> Self {
                    n as $ty
                }
            }
        )*
    };
}

impl_fromf64!(
    isize, usize,
    i8, i16, i32, i64,
    u8, u16, u32, u64,
    f32, f64
);

pub trait ToFloat64{
    fn tof64(self) -> f64;
}

macro_rules! impl_tof64 {
    ( $($ty:ty),* ) => {
        $(
            impl ToFloat64 for $ty {
                fn tof64(self) -> f64 {
                    self as f64
                }
            }
        )*
    };
}

impl_tof64!(
    isize, usize,
    i8, i16, i32, i64,
    u8, u16, u32, u64,
    f32, f64
);

pub trait FromInt32{
    fn fromi32(n: i32) -> Self;
}

macro_rules! impl_fromi32 {
    ( $($ty:ty),* ) => {
        $(
            impl FromInt32 for $ty {
                fn fromi32(n: i32) -> Self {
                    n as $ty
                }
            }
        )*
    };
}

impl_fromi32!(
    isize, usize,
    i8, i16, i32, i64,
    u8, u16, u32, u64,
    f32, f64
);

pub trait ToInt32{
    fn toi32(self) -> i32;
}

macro_rules! impl_toi32 {
    ( $($ty:ty),* ) => {
        $(
            impl ToInt32 for $ty {
                fn toi32(self) -> i32 {
                    self as i32
                }
            }
        )*
    };
}

impl_toi32!(
    isize, usize,
    i8, i16, i32, i64,
    u8, u16, u32, u64,
    f32, f64
);