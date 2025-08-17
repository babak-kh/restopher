use anyhow::{anyhow, Result};
use graphql_parser::{
    query::Type,
    schema::{
        parse_schema, Definition, Document, Field, InputObjectType, ObjectType, SchemaDefinition,
        TypeDefinition,
    },
};

use std::{error::Error as StdError, fs, path::Path};

/// A wrapper around the GraphQL schema document that provides convenient methods to access
/// schema information
#[derive(Debug, Clone)]
pub struct Schema {
    pub schema: Document<'static, String>,
    query_type_name: String,
    mutation_type_name: String,
}

impl Schema {
    /// Creates a new schema by parsing the schema.graphql file
    pub fn new() -> Result<Self> {
        // Consider making this path configurable in the future
        // Try a few likely locations for the schema file
        let candidates = [
            Path::new("schema.graphql"),
            Path::new("src/graphql/schema.graphql"),
            Path::new("restopher/src/graphql/schema.graphql"),
        ];

        let schema_opt = candidates.iter().find_map(|p| fs::read_to_string(p).ok());

        let schema = match schema_opt {
            Some(s) => s,
            None => {
                return Err(anyhow!(
                    "Schema file not found in expected locations: schema.graphql, src/schema.graphql, tests/src/schema.graphql".to_string()
                ))
            }
        };

        let parsed = parse_schema::<String>(&schema)?;
        let static_data = parsed.into_static();

        let ss = Self {
            query_type_name: get_query_type_name(&static_data),
            mutation_type_name: get_mutation_type_name(&static_data),
            schema: static_data,
        };
        Ok(ss)
    }

    /// Returns the name of the query type
    pub fn get_query_type_name(&self) -> String {
        self.query_type_name.clone()
    }

    pub fn get_fields(&self, object_name: &str) -> Vec<String> {
        self.schema
            .definitions
            .iter()
            .filter_map(|def| match def {
                Definition::TypeDefinition(TypeDefinition::Object(obj)) => {
                    if obj.name == object_name {
                        Some(
                            obj.fields
                                .iter()
                                .map(|f| f.name.clone())
                                .collect::<Vec<_>>(),
                        )
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .flatten()
            .collect()
    }

    /// Returns the name of the mutation type
    pub fn get_mutation_type_name(&self) -> String {
        self.mutation_type_name.clone()
    }

    /// Returns a list of available object type names
    pub fn get_object_types(&self) -> Vec<String> {
        self.schema
            .definitions
            .iter()
            .filter_map(|def| match def {
                Definition::TypeDefinition(TypeDefinition::Object(obj)) => Some(obj.name.clone()),
                _ => None,
            })
            .collect()
    }

    /// Returns a list of available query operation names
    pub fn get_queries(&self) -> Vec<String> {
        self.schema
            .definitions
            .iter()
            .filter_map(|def| match def {
                Definition::TypeDefinition(TypeDefinition::Object(obj)) => {
                    if obj.name == self.query_type_name {
                        Some(
                            obj.fields
                                .iter()
                                .map(|f| f.name.clone())
                                .collect::<Vec<_>>(),
                        )
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .flatten()
            .collect()
    }

    /// Returns a list of available mutation operation names
    pub fn get_mutations(&self) -> Vec<String> {
        self.schema
            .definitions
            .iter()
            .filter_map(|def| match def {
                Definition::TypeDefinition(TypeDefinition::Object(obj)) => {
                    if obj.name == self.mutation_type_name {
                        Some(
                            obj.fields
                                .iter()
                                .map(|f| f.name.clone())
                                .collect::<Vec<_>>(),
                        )
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .flatten()
            .collect()
    }

    /// Finds a type by name in the schema
    pub fn find_type(&self, type_name: &str) -> Option<&TypeDefinition<'static, String>> {
        for def in &self.schema.definitions {
            if let Definition::TypeDefinition(type_def) = def {
                match type_def {
                    TypeDefinition::Object(obj) if obj.name == type_name => return Some(type_def),
                    TypeDefinition::Interface(iface) if iface.name == type_name => {
                        return Some(type_def)
                    }
                    TypeDefinition::InputObject(input) if input.name == type_name => {
                        return Some(type_def)
                    }
                    TypeDefinition::Enum(enum_type) if enum_type.name == type_name => {
                        return Some(type_def)
                    }
                    TypeDefinition::Scalar(scalar) if scalar.name == type_name => {
                        return Some(type_def)
                    }
                    TypeDefinition::Union(union_type) if union_type.name == type_name => {
                        return Some(type_def)
                    }
                    _ => {}
                }
            }
        }
        None
    }

    fn get_operation_as_fields(&self, operation: &str) -> Option<&[Field<'static, String>]> {
        self.schema.definitions.iter().find_map(|def| match def {
            Definition::TypeDefinition(obj) => match obj {
                TypeDefinition::Object(object_type) if object_type.name == operation => {
                    Some(object_type.fields.as_slice())
                }
                _ => None,
            },
            _ => None,
        })
    }

    fn get_return_type_for_operation_and_operation_name(
        &self,
        operation_name: &str,
        operation_type: &str,
    ) -> Option<String> {
        if let Some(fields) = self.get_operation_as_fields(operation_type) {
            fields
                .iter()
                .find(|def| def.name == operation_name)
                .map(|def| def.field_type.to_string())
        } else {
            None
        }
    }

    pub fn get_return_type_for_operation(&self, operation_name: &str) -> Option<String> {
        if let Some(file_type) = self
            .get_return_type_for_operation_and_operation_name(operation_name, &self.query_type_name)
        {
            Some(file_type)
        } else {
            self.get_return_type_for_operation_and_operation_name(
                operation_name,
                &self.mutation_type_name,
            )
        }
    }

    fn get_arguments_for_operation_and_operation_name(
        &self,
        operation_name: &str,
        operation_type: &str,
    ) -> Option<Vec<(String, String)>> {
        if let Some(fields) = self.get_operation_as_fields(operation_type) {
            fields
                .iter()
                .find(|def| def.name == operation_name)
                .map(|args| {
                    args.arguments
                        .iter()
                        .map(|arg| (arg.name.clone(), arg.value_type.to_string().clone()))
                        .collect::<Vec<(String, String)>>()
                })
        } else {
            None
        }
    }

    pub fn get_arguments_for_operation(
        &self,
        operation_name: &str,
    ) -> Option<Vec<(String, String)>> {
        if let Some(args) = self
            .get_arguments_for_operation_and_operation_name(operation_name, &self.query_type_name)
        {
            Some(args)
        } else {
            self.get_arguments_for_operation_and_operation_name(
                operation_name,
                &self.mutation_type_name,
            )
        }
    }
}

/// Extracts the query type name from the schema definition
pub fn get_query_type_name(schema: &Document<'static, String>) -> String {
    for def in &schema.definitions {
        if let Definition::SchemaDefinition(SchemaDefinition { query, .. }) = def {
            if let Some(q) = query {
                return q.clone();
            }
        }
    }
    "Query".to_string() // Default name if not explicitly defined
}

/// Extracts the mutation type name from the schema definition
pub fn get_mutation_type_name(schema: &Document<'static, String>) -> String {
    for def in &schema.definitions {
        if let Definition::SchemaDefinition(SchemaDefinition { mutation, .. }) = def {
            if let Some(m) = mutation {
                return m.clone();
            }
        }
    }
    "Mutation".to_string() // Default name if not explicitly defined
}
