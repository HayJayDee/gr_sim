
pub type SolveFun<T> = dyn Fn(f32, T) -> T;

pub trait Solver<T> {
    fn next_step(&mut self, fun: &SolveFun<T>, delta_time: f32) -> &T;
}
