// Traits
use rand::Rng;
use rand_distr::Distribution;

/// A quick and higly error prone struct for quickly define a struct that 
/// implements `Distribution<T>`.
#[derive(Debug)]
pub struct Raw<I> {
	iter: I,
}

impl<I> Raw<I> 
where
	I: IntoIterator,
{
	#[inline]
	pub fn new(iter: I) -> Self {
		Raw { iter }
	}
}

impl<I, T> Distribution<T> for Raw<I> 
where
	I: IntoIterator + Clone,
	(f64, T): From<<I as IntoIterator>::Item>,
{
	#[inline]
	fn sample<R>(&self, rng: &mut R) -> T
    where
        R: Rng + ?Sized
    {
    	let cum_goal: f64 = rng.gen();
    	let mut acc = 0.;
    	for item in self.iter.clone() {
    		let (prob, state) = item.into();
    		acc += prob;
    		if acc >= cum_goal {
    			return state;
    		}
    	}
    	panic!("Sampling was not possible: probabilities did not sum up to one.")
    }
}

impl<I> From<I> for Raw<I> 
where
	I: IntoIterator + Clone,
{
	#[inline]
	fn from(iter: I) -> Self { 
		Raw::new(iter)	 
	}
}

#[cfg(test)]
mod tests {
    use super::Raw;
    use pretty_assertions::assert_eq;
    use rand_distr::Distribution;
    // use approx::abs_diff_eq;

    #[test]
    fn sampling_stability() {
        let mut rng = crate::test::rng(1);
        let expected = 1;
        let dis = Raw::from(vec![(1.0, expected)]);
        let sample = (0..100).map(|_| dis.sample(&mut rng)); 
        for x in sample {
        	assert_eq!(x, expected);
        }

        let dis = Raw::from(vec![(0.5, 1), (0.5, 2)]);
        let sample = (0..100).map(|_| dis.sample(&mut rng)); 
        for x in sample {
        	assert!(x == 1 || x == 2);
        } 
    }

    #[test]
    fn value_stability() {
        let mut rng = crate::test::rng(2);
        let expected = [2, 1, 1, 2];
        let dis = Raw::from(vec![(0.5, 1), (0.5, 2)]);
        let sample = [dis.sample(&mut rng), dis.sample(&mut rng), dis.sample(&mut rng), dis.sample(&mut rng)]; 
        
        assert_eq!(sample, expected);
    }
}