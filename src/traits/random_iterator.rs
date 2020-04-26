use rand::Rng;

pub trait RandomIterator<R>: Iterator 
where
        R: Rng + ?Sized,
{
    fn sample_next(&self, rng: &mut R) -> Option<Self::Item>;
}
