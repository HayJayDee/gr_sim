use super::solver::Solver;


pub struct EulerSolver {
    pub time: f32,
    pub state: f32,
}

impl EulerSolver {
    pub fn new(time: f32, initial_state: f32) -> Self {
        Self {
            time,
            state: initial_state
        }
    }
}

impl Solver<f32> for EulerSolver {
    fn next_step<F: Fn(f32, f32) -> f32>(self: &mut Self, fun: F, delta_time: f32) -> f32 {
        self.time += delta_time;
        self.state += delta_time * fun(self.time, self.state);
        self.state
    }
}
