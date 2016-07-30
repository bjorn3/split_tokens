#[macro_use]
extern crate split_tokens;

macro_rules! cb{
    ( ( $($args:tt)* ) ( $($out:tt)* ) ) => {
        //...
    }
}

fn main(){
    split_tokens!( (>>+=-=<<) then cb!() );
}
