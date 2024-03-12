use super::solver::{SolveFun, Solver};


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
    fn next_step(&mut self, fun: &SolveFun<f32>, delta_time: f32) -> &f32 {
        self.time += delta_time;
        self.state += delta_time * fun(self.time, self.state);
        &self.state
    }
}
