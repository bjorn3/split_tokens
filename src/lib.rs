#![cfg_attr(test, feature(trace_macros))]

#[cfg(test)]
trace_macros!(true);


/// Split compound tokens in their individual parts.
/// 
/// #Warning
/// tokentrees are not traversed!
/// 
/// #Usage
/// ```rust,ignore
/// # #[macro_use] extern crate split_tokens;
/// macro_rules! cb{
///     ( ( $($args:tt)* ) ( $($out:tt)* ) ) => {
///         //...
///     }
/// }
/// split_tokens!( (>>+=-=<<) then cb!( /*args*/ ) );
/// ```
///
/// #Current split tokens
/// * >>
/// * <<
/// * +=
/// * -=
#[macro_export]
macro_rules! split_tokens{
    (@inner () ( $($out:tt)* ) ( $m:ident $($args:tt)* ) ) => {
        $m ! ( ( $($args)* ) ( $($out)* ) );
    };
    
    () => {};
    
    (@inner ( >> $($input:tt)* ) ( $($out:tt)* ) ( $m:ident $($args:tt)* ) ) => {
        split_tokens!(@inner ( $($input)* ) ( $($out)* > > ) ( $m $($args)* ) );
    };
    
    (@inner ( << $($input:tt)* ) ( $($out:tt)* ) ( $m:ident $($args:tt)* ) ) => {
        split_tokens!(@inner ( $($input)* ) ( $($out)* < < ) ( $m $($args)* ) );
    };
    
    (@inner ( += $($input:tt)* ) ( $($out:tt)* ) ( $m:ident $($args:tt)* ) ) => {
        split_tokens!(@inner ( $($input)* ) ( $($out)* + = ) ( $m $($args)* ) );
    };
    
    (@inner ( -= $($input:tt)* ) ( $($out:tt)* ) ( $m:ident $($args:tt)* ) ) => {
        split_tokens!(@inner ( $($input)* ) ( $($out)* - = ) ( $m $($args)* ) );
    };
    
    () => {};
    
    (@inner ( $a:tt $($input:tt)* ) ( $($out:tt)* ) ( $m:ident $($args:tt)* ) ) => {
        split_tokens!(@inner ( $($input)* ) ( $($out)* $a ) ( $m $($args)* ) );
    };
    
    ( ( $($input:tt)* ) then $m:ident ! ($($args:tt)*) ) => {
        split_tokens!(@inner ( $($input)* ) () ( $m $($args)* ) );
    };
}

#[test]
fn test(){
    split_tokens!{(>>+=-=<<) then stringify!()};
}
