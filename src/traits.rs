pub trait MarkovChainTrait<T>: Iterator<Item = T> {
    fn state(&self) -> &T;

    fn set_state(&mut self, state: T) -> &mut Self;
}

pub trait CMarkovChainTrait<T>: Iterator<Item = (f64, T)> {
    fn state(&self) -> &T;

    fn set_state(&mut self, state: T) -> &mut Self;
}

pub trait BranchingTrait<T, I>: MarkovChainTrait<T>
where
    f64: From<T>, // Needed to take powers. This limits the process to u32 or smaller.
    I: IntoIterator<Item = (T, f64)>,
{
    fn density(&self) -> I;

    /// Uses the first approx_level terms from the density to compute an approximation
    /// of the generating function evaluated at z. Formally, it computes
    ///
    /// \sum_{i = 0}^{n*} z^i * density(i)
    ///
    /// where approx_level = |\{ 0, ..., n* \} inter supp(density)|,
    /// which converges to the true value when approx_level goes to infinity
    /// and is the true value as soon as approx_level = |supp(density)| - 1.
    fn approx_generating_fun(&self, z: f64, approx_level: usize) -> f64 {
        let mut density = self.density().into_iter();

        let mut result = 0.0;
        for _ in 0..=approx_level {
            match density.next() {
                Some((state, prob)) => result += z.powf(f64::from(state)) * prob,
                None => break,
            }
        }

        result
    }
}
