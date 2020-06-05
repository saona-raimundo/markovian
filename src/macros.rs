use crate::distributions::Raw;

/// Creates a [`Raw`] struct by first allocating a `Vec` and passing it to `Raw::new`.
/// 
/// If you need to pass an iterator, use `Raw::new` method.
/// 
/// [`Raw`]: distributions/struct.Raw.html
#[macro_export]
macro_rules! raw_dist {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            Raw::new(temp_vec)
        }
    };
}


#[cfg(test)]
mod tests {
	use super::*;

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