/// Build a new `VecMap` efficiently.
#[macro_export]
macro_rules! vec_map {
	(@single $($x:tt)*) => (());
	(@count $($rest:expr),*) => (<[()]>::len(&[$(vec_map!(@single $rest)),*]));

	($($key:expr => $value:expr,)+) => { vec_map!($($key => $value),+) };
	($($key:expr => $value:expr),*) => {
		{
			let _cap = vec_map!(@count $($key),*);
			let mut _map = ::vec_map::VecMap::with_capacity(_cap);
			$(
				_map.insert($key, $value);
			)*
			_map
		}
	};
}
