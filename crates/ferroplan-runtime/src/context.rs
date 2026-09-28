pub fn bounded<T:Clone>(x:&[T],n:usize)->Vec<T>{x.iter().rev().take(n).cloned().collect::<Vec<_>>().into_iter().rev().collect()}
