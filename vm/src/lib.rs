extern crate core;

mod closure;
mod courier;
mod error;
mod frame;
mod identifier;
mod machine;
mod value;

pub use closure::Closure;
pub use courier::Courier;
pub(crate) use frame::CallFrame;
pub use identifier::Identifier;
pub use machine::VirtualMachine;
pub use tortuga_executable::{Executable, Function, Number, Text};
pub use value::Value;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
