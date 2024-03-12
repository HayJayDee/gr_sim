use super::solver::{SolveFun, Solver};
use ndarray::Array1;


// Fore more information see https://de.wikipedia.org/wiki/Runge-Kutta-Verfahren

const B_J: [f32; 4] = [
    1.0/6.0,
    1.0/3.0,
    1.0/3.0,
    1.0/6.0,
];

const C_J: [f32; 4] = [
    0.0,
    0.5,
    0.5,
    1.0
];

// TODO: Remove last entry
const A_IJ: [[f32; 4]; 4] = [
    [0.0    , 0.0    , 0.0, 0.0],
    [1.0/2.0, 0.0    , 0.0, 0.0],
    [0.0    , 1.0/2.0, 0.0, 0.0],
    [0.0    , 0.0    , 1.0, 0.0],
];


pub struct RungeKutta4<const D: usize> {
    pub time: f32,
    pub state: Array1<f32>,
}

impl<const D: usize> RungeKutta4<D> {
    pub fn new(time: f32, initial_state: Array1<f32>) -> Self {
        Self {
            time,
            state: initial_state
        }
    }
}

impl<const D: usize> RungeKutta4<D> {
    fn calc_kj(&self, j: usize, fun: &SolveFun<Array1<f32>>, delta_time: f32) -> Array1<f32> {

        let mut inner_sum = Array1::<f32>::zeros(D);

        for l in 0..4 {
            if A_IJ[j][l] == 0.0 {
                continue;
            }
            inner_sum = inner_sum + A_IJ[j][l] * self.calc_kj(l, fun, delta_time);
        }

        fun(self.time + delta_time * C_J[j], &self.state + delta_time * inner_sum)
    }
}

impl<const D: usize> Solver<Array1<f32>> for RungeKutta4<D> {
    
    fn next_step(&mut self, fun: &SolveFun<Array1<f32>>, delta_time: f32) -> &Array1<f32> {
        let mut sum = Array1::<f32>::zeros(D);
        for j in 0..4 {
            sum = sum + B_J[j] * self.calc_kj(j, fun, delta_time);
        }

        self.time += delta_time;
        self.state = &self.state + delta_time * &sum;
        &self.state
    }
}
