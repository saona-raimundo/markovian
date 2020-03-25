//! Simulation of sub-stochastic Markov processes in discrete and continuous time.  
//! 
//!

pub use continuous_time::*;
pub use discrete_time::*;

pub mod continuous_time;

pub mod discrete_time;

pub mod traits;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
