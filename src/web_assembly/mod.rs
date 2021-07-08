mod instruction;
mod module;
mod types;
mod values;

pub use instruction::*;
pub use module::*;
pub use types::*;
pub use values::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_module() {
        let module = Module::new();

        assert!(module.types().is_empty());
        assert!(module.functions().is_empty());
        assert!(module.tables().is_empty());
        assert!(module.memories().is_empty());
        assert!(module.globals().is_empty());
        assert!(module.imports().is_empty());
        assert!(module.exports().is_empty());
        assert!(module.data().is_empty());
        assert!(module.elements().is_empty());
        assert!(module.start().is_none());
    }

    #[test]
    fn empty_type() {
        let result_type = ResultType::new(Vec::new());
        let function_type = FunctionType::new(result_type.clone(), result_type.clone());

        assert!(function_type.parameters().is_empty());
        assert!(function_type.results().is_empty());
    }
}
