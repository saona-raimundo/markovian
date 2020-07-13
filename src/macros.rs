/// Creates a [`Raw`] struct by first allocating a `Vec` and passing it to `Raw::new`.
/// 
/// If you need to pass an iterator, use `Raw::new` method.
/// 
/// [`Raw`]: distributions/struct.Raw.html
/// 
/// # Examples
/// 
/// With help of the `raw_dist` macro, we construct a random variable that samples always a fixed value.
/// ```
/// # use markovian::prelude::*;
/// # use rand::prelude::*;
/// # #[macro_use] extern crate markovian;
/// # fn main() {
/// let value = 0;
/// let dis: Raw<_> = raw_dist![(1.0, value)];
///
/// assert_eq!(value, dis.sample(&mut thread_rng()));
/// }
/// ```
#[macro_export]
macro_rules! raw_dist {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            $crate::distributions::Raw::new(temp_vec)
        }
    };
}


#[cfg(test)]
mod tests {
	// use super::*;
	use crate::distributions::Raw;

	#[test]
	fn construction() {
		let expected = Raw::new(vec![(1.0, 1)]);
		let dis = raw_dist![(1.0, 1)];

		assert_eq!(expected, dis);

		let expected = Raw::new(vec![(0.5, 1), (0.5, 2)]);
		let dis = raw_dist![(0.5, 1), (0.5, 2)];

		assert_eq!(expected, dis);
	}
}