use rand::Rng;

pub trait RandomIterator<R>: Iterator 
where
        R: Rng + ?Sized,
{
	#[inline]
    fn sample_next(&self, rng: &mut R) -> Option<Self::Item>;
}
