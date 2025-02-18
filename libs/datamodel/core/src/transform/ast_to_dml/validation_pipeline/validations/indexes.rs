use crate::{
    diagnostics::{DatamodelError, Diagnostics},
    transform::ast_to_dml::db::{walkers::IndexWalker, ConstraintName, ParserDatabase},
};

/// Different databases validate index and unique constraint names in a certain namespace.
/// Validates index and unique constraint names against the database requirements.
pub(crate) fn has_a_unique_constraint_name(
    db: &ParserDatabase<'_>,
    index: IndexWalker<'_, '_>,
    diagnostics: &mut Diagnostics,
) {
    let name = index.final_database_name();
    let model = index.model();

    for violation in db.scope_violations(model.model_id(), ConstraintName::Index(name.as_ref())) {
        let message = format!(
            "The given constraint name `{}` has to be unique in the following namespace: {}. Please provide a different name using the `map` argument.",
            name,
            violation.description(model.name()),
        );

        let span = index
            .ast_attribute()
            .map(|a| {
                let from_arg = a.span_for_argument("map").or_else(|| a.span_for_argument("name"));
                from_arg.unwrap_or(a.span)
            })
            .unwrap_or_else(|| model.ast_model().span);

        diagnostics.push_error(DatamodelError::new_attribute_validation_error(
            &message,
            index.attribute_name(),
            span,
        ));
    }
}
