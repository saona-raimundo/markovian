//! Traits with two objectives in mind: use trait objects and implement your own stochastic processes. 


/// Discrete time Markov process.
pub trait MarkovChainTrait<T>: Iterator<Item = T> {
    
    /// Current state of the process. 
    fn state(&self) -> &T;

    /// Change the current state of the process. 
    fn set_state(&mut self, state: T) -> &mut Self;
}

/// Continuous time Markov process.
pub trait CMarkovChainTrait<T>: Iterator<Item = (f64, T)> {

    /// Current state of the process. 
    fn state(&self) -> &T;

    /// Change the current state of the process. 
    fn set_state(&mut self, state: T) -> &mut Self;
}

/// Discrete time Branching process. 
pub trait BranchingTrait<T, I>: MarkovChainTrait<T>
where
    f64: From<T>, // Needed to take powers. This limits the process to u32 or smaller.
    I: IntoIterator<Item = (T, f64)>,
{
    
    /// Returns the density in which the branching process is based. 
    fn density(&self) -> I;

    /// Uses the first approx_level terms from the density to compute an approximation
    /// of the generating function evaluated at z. Formally, it computes
    ///
    /// \sum_{i = 0}^{n*} z^i * density(i)
    ///
    /// where approx_level = |\{ 0, ..., n* \} inter supp(density)|,
    /// i.e. the number of nonzero terms. 
    /// 
    /// This approximation converges to the true value when ``approx_level`` goes to infinity
    /// and is the true value as soon as approx_level = |supp(density)|.
    /// 
    /// # Examples
    /// 
    /// 
    /// ```
    /// let init_state: u32 = 1;
    /// let density = vec![(0, 0.3), (1, 0.4), (2, 0.3)];
    /// let branching_process = markovian::Branching::new(init_state, density);
    /// 
    /// use markovian::traits::BranchingTrait;
    /// // At z = 1.0, it should be 1.0, except for the approx_level. 
    /// assert_eq!(0.3, branching_process.approx_generating_fun(1.0, 1));
    /// assert_eq!(0.7, branching_process.approx_generating_fun(1.0, 2));
    /// assert_eq!(1.0, branching_process.approx_generating_fun(1.0, 3));
    /// assert_eq!(1.0, branching_process.approx_generating_fun(1.0, 20));
    /// 
    /// // At z = 2.0, it should be 1.0 * 0.3 + 2.0 * 0.4 + 4.0 * 0.3. 
    /// assert_eq!(1.0 * 0.3 + 2.0 * 0.4 + 4.0 * 0.3, branching_process.approx_generating_fun(2.0, 3));
    /// ```
    /// 
    fn approx_generating_fun(&self, z: f64, approx_level: usize) -> f64 {
        let mut density = self.density().into_iter();

        let mut result = 0.0;
        for _ in 0..approx_level {
            match density.next() {
                Some((state, prob)) => result += z.powf(f64::from(state)) * prob,
                None => break,
            }
        }

        result
    }
}
