extern crate core;

mod closure;
mod courier;
mod error;
mod frame;
mod identifier;
mod machine;
mod number;
mod operation;
mod program;
mod value;

pub use closure::Closure;
pub use courier::Courier;
pub(crate) use frame::CallFrame;
pub use identifier::Identifier;
pub use number::Number;
#[cfg(test)]
pub(crate) use operation::Operations;
pub use program::Program;
pub use value::Value;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
