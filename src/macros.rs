use crate::traits::DistributionOnce;
use rand::Rng;
use rand_distr::Distribution;

Look into https://docs.rs/syn/1.0.18/syn/struct.Generics.html#method.split_for_impl


macro_rules! foo {
    (@$trait_:ident [$($args:ident,)*] where [$($preds:tt)+]) => {
        foo! {
            @as_item
            impl<$($args),*> $trait_<$($args),*>
                where $($args: ::std::any::Any + 'static,)*
                      $($preds)*
            {
                #[allow(non_camel_case_types, dead_code)]
                pub fn bar<__foo_T: $trait_<$($args),*>>(&self) {}
            }
        }
    };
    (@as_item $i:item) => { $i };

    (
        $trait_:ident < $($args:ident),* $(,)* >
        where $($preds:tt)+
    ) => {
        foo! { @$trait_ [$($args,)*] where [$($preds)*] }
    };
}

macro_rules! implement_distribution_once {
    impl<T> markovian::traits::DistributionOnce<T> for $input 
	{
	    fn sample_once<R: Rng + ?Sized>(self, rng: &mut R) -> T {
	        self.sample(rng)
	    }
	}
}
	