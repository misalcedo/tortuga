use crate::web_assembly::{Expression, Identifier, Offset, ReferenceType, TableUse};

/// Element segments allow for an optional table index to identify the table to initialize.
pub struct Element {
    id: Identifier,
    table: TableUse,
    offset: Offset,
    elements: ElementList,
}

pub struct ElementList {
    reference_type: ReferenceType,
    elements: Vec<ElementExpression>,
}

pub struct ElementExpression(Expression);

pub struct ElementIndex(Identifier);
