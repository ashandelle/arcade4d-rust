use noisy_float::prelude::*;
use mathnd::traits::*;

impl Sqrt for N64 {
    fn sqrt(self) -> N64 {
        N64::new(self.raw().sqrt())
    }
}
impl Abs for N64 {
    fn abs(self) -> N64 {
        N64::new(self.raw().abs())
    }
}
impl Signum for N64 {
    fn signum(self) -> N64 {
        N64::new(self.raw().signum())
    }
}
impl MinMax for N64 {
    fn min(self, other: N64) -> N64 {
        std::cmp::min(self, other)
    }

    fn max(self, other: N64) -> N64 {
        std::cmp::max(self, other)
    }
}
impl Zero for N64 {
    fn zero() -> N64 {
        n64(0.0)
    }
}
impl One for N64 {
    fn one() -> N64 {
        n64(1.0)
    }
}
impl Two for N64 {
    fn two() -> N64 {
        n64(2.0)
    }
}
impl MinMaxValue for N64 {
    fn minimum() -> N64 {
        n64(f64::MIN)
    }

    fn maximum() -> N64 {
        n64(f64::MAX)
    }
}
impl FromUsize for N64 {
    fn fromusize(n: usize) -> N64 {
        n64(n as f64)
    }
}
impl FromFloat32 for N64 {
    fn fromf32(n: f32) -> N64 {
        n64(n as f64)
    }
}
impl FromFloat64 for N64 {
    fn fromf64(n: f64) -> N64 {
        n64(n)
    }
}
impl ToFloat64 for N64 {
    fn tof64(self) -> f64 {
        self.raw()
    }
}
impl FromInt32 for N64 {
    fn fromi32(n: i32) -> N64 {
        n64(n as f64)
    }
}
impl ToInt32 for N64 {
    fn toi32(self) -> i32 {
        self.raw() as i32
    }
}