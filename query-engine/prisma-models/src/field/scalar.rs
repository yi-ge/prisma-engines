use crate::{prelude::*, InternalEnum};
use datamodel::{DefaultValue, FieldArity, NativeTypeInstance};
use once_cell::sync::OnceCell;
use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
    sync::{Arc, Weak},
};

pub type ScalarFieldRef = Arc<ScalarField>;
pub type ScalarFieldWeak = Weak<ScalarField>;

pub struct ScalarField {
    pub name: String,
    pub type_identifier: TypeIdentifier,
    pub is_required: bool,
    pub is_list: bool,
    pub is_id: bool,
    pub is_auto_generated_int_id: bool,
    pub is_autoincrement: bool,
    pub internal_enum: Option<InternalEnum>,
    pub behaviour: Option<FieldBehaviour>,
    pub arity: FieldArity,
    pub db_name: Option<String>,
    pub default_value: Option<DefaultValue>,
    pub native_type: Option<NativeTypeInstance>,

    pub model: ModelWeakRef,
    pub(crate) is_unique: bool,
    pub(crate) read_only: OnceCell<bool>,
}

impl ScalarField {
    pub fn model(&self) -> ModelRef {
        self.model
            .upgrade()
            .expect("Model does not exist anymore. Parent model got deleted without deleting the child.")
    }

    pub fn internal_data_model(&self) -> InternalDataModelRef {
        self.model().internal_data_model()
    }

    pub fn is_id(&self) -> bool {
        self.is_id
    }

    pub fn is_created_at(&self) -> bool {
        matches!(self.behaviour, Some(FieldBehaviour::CreatedAt))
    }

    pub fn is_updated_at(&self) -> bool {
        matches!(self.behaviour, Some(FieldBehaviour::UpdatedAt))
    }

    pub fn unique(&self) -> bool {
        self.is_unique || self.is_id()
    }

    pub fn db_name(&self) -> &str {
        self.db_name.as_deref().unwrap_or_else(|| self.name.as_str())
    }

    pub fn type_identifier_with_arity(&self) -> (TypeIdentifier, FieldArity) {
        (self.type_identifier.clone(), self.arity)
    }

    pub fn is_read_only(&self) -> bool {
        *self.read_only.get_or_init(|| false)
    }

    pub fn is_numeric(&self) -> bool {
        self.type_identifier.is_numeric()
    }
}

impl Debug for ScalarField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScalarField")
            .field("name", &self.name)
            .field("type_identifier", &self.type_identifier)
            .field("is_required", &self.is_required)
            .field("is_list", &self.is_list)
            .field("is_id", &self.is_id)
            .field("is_auto_generated_int_id", &self.is_auto_generated_int_id)
            .field("is_autoincrement", &self.is_autoincrement)
            .field("internal_enum", &self.internal_enum)
            .field("behaviour", &self.behaviour)
            .field("arity", &self.arity)
            .field("db_name", &self.db_name)
            .field("default_value", &self.default_value)
            .field("model", &"#ModelWeakRef#")
            .field("is_unique", &self.is_unique)
            .field("read_only", &self.read_only)
            .finish()
    }
}

impl Eq for ScalarField {}

impl Hash for ScalarField {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.type_identifier.hash(state);
        self.is_required.hash(state);
        self.is_list.hash(state);
        self.is_id.hash(state);
        self.is_auto_generated_int_id.hash(state);
        self.internal_enum.hash(state);
        self.behaviour.hash(state);
        self.is_unique.hash(state);
        self.model().hash(state);
        self.arity.hash(state);
        self.db_name.hash(state);
    }
}

impl PartialEq for ScalarField {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.type_identifier == other.type_identifier
            && self.is_required == other.is_required
            && self.is_list == other.is_list
            && self.is_id == other.is_id
            && self.is_auto_generated_int_id == other.is_auto_generated_int_id
            && self.internal_enum == other.internal_enum
            && self.behaviour == other.behaviour
            && self.default_value == other.default_value
            && self.is_unique == other.is_unique
            && self.model() == other.model()
            && self.arity == other.arity
            && self.db_name == other.db_name
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FieldBehaviour {
    CreatedAt,
    UpdatedAt,
    ScalarList { strategy: ScalarListStrategy },
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ScalarListStrategy {
    Embedded,
    Relation,
}
