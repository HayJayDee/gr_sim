
pub trait Solver<T> {
    fn next_step<F: Fn(f32, T) -> T>(self: &mut Self, fun: F, delta_time: f32) -> f32;
}
