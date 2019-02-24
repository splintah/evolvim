pub mod feed_forward;
pub use feed_forward::Brain;

mod environment;
pub use environment::{Environment, EnvironmentMut};

// pub trait NeuralNet<T: MotorCommands> {
pub trait NeuralNet {
    fn load_input(&mut self, env: &Environment);

    fn run(&mut self);

    fn run_with(&mut self, env: &Environment) {
        self.load_input(env);
        self.run();
    }

    // fn get_output(&self) -> T;
}

pub trait MotorCommands {
    fn execute(&self, env: &EnvironmentMut);
}