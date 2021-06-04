/** `
 * Module for generating Rust language bindings.
 */

/* eslint-disable @typescript-eslint/restrict-template-expressions */

import { pascalCase, snakeCase } from 'change-case'
import fs from 'fs-extra'
import path from 'path'
import { JsonSchema } from '../JsonSchema'
import {
  filterEnumSchemas,
  filterInterfaceSchemas,
  filterUnionSchemas,
  getSchemaProperties,
  readSchemas,
} from '../util/helpers'

/**
 * Run `build()` when this file is run as a Node script
 */
// eslint-disable-next-line @typescript-eslint/no-floating-promises
if (require.main) build()

// Code generation context
interface Context {
  propertyName?: string
  typeName?: string
  propertyTypeName?: string
  anonEnums: Record<string, string>
}

// Attributes to add to properties of some types
const propertyAttributes: Record<string, string[]> = {
  'Date.value': ['#[def = "chrono::Utc::now().to_rfc3339()"]'],
  'PropertyValue.value': [
    '#[def = "PropertyValueValue::String(String::new())"]',
  ],
}

// Manually defined types for properties of some types
const propertyTypes: Record<string, [string, string]> = {
  'Date.value': ['DateValue', 'String'],
}

// Properties that need to use a pointer to prevent circular references
// (the "recursive type has infinite size" error)
const pointerProperties = [
  '*.isPartOf',
  'Organization.parentOrganization',
  'ImageObject.publisher', // recursive because publisher has `logo`
  'ImageObject.thumbnail',
  'ListItem.item',
  'Comment.parentItem',
  'ArrayValidator.contains',
  'ArrayValidator.itemsValidator',
  'ConstantValidator.value',
  'CodeExpression.output',
  'Parameter.default',
  'Parameter.value',
  'Variable.value',
]

/**
 * Generate `../../rust/types.rs` from schemas.
 */
async function build(): Promise<void> {
  const schemas = await readSchemas()

  const context = {
    anonEnums: {},
  }

  const structs = filterInterfaceSchemas(schemas)
    .map((schema) => interfaceSchemaToEnum(schema, context))
    .join('\n')

  const enumEnums = filterEnumSchemas(schemas)
    .map((schema) => enumSchemaToEnum(schema, context))
    .join('\n')

  const unionEnums = filterUnionSchemas(schemas)
    .map((schema) => unionSchemaToEnum(schema, context))
    .join('\n')

  const code = `// Generated by rust.ts; do not edit

#![allow(clippy::large_enum_variant)]

use crate::impl_type;
use crate::prelude::*;

/*********************************************************************
 * Structs for "interface" schemas
 ********************************************************************/

${structs}

/*********************************************************************
 * Types for properties that are manually defined
 ********************************************************************/

${Object.entries(propertyTypes).map(
  ([_key, [name, type]]) => `type ${name} = ${type};\n`
)}

/*********************************************************************
 * Enums for struct properties which use JSON Schema 'enum' or 'anyOf'
 ********************************************************************/

${Object.values(context.anonEnums).join('\n')}

/*********************************************************************
 * Enums for "enum" schemas
 ********************************************************************/

${enumEnums}

/*********************************************************************
 * Enums for "union" schemas
 ********************************************************************/
  
${unionEnums}`

  await fs.writeFile(
    path.join(__dirname, '..', '..', 'rust', 'src', 'types.rs'),
    code
  )
}

/**
 * Generate a doc comments
 */
function docComment(description: string): string {
  return '/// ' + description.trim().replace(/[\n\r]+/g, ' ')
}

/**
 * Generate a Rust `struct` for an "interface" schema.
 *
 * Adds a `type_` property that is intended to help in de-serialization
 * to disambiguate among alternative types in an enum. This is
 * necessary because we can not use `#[serde(tag = "type")]` for enums
 * that involve primitive types. Although we could add that option
 * to each struct it does not help with disambiguation when it comes to
 * deserialization. See https://github.com/serde-rs/serde/issues/760.
 */
export function interfaceSchemaToEnum(
  schema: JsonSchema,
  context: Context
): string {
  const { title = 'Untitled', description = title } = schema
  const { all } = getSchemaProperties(schema)

  const fields = all
    .map(({ name, schema, optional, inherited, override }) => {
      const { description = name, from } = schema

      // Generate a type name for this property (to avoid duplication
      // use the name of the type that this property was defined on)
      context.propertyName = name
      context.typeName = inherited && !override ? from : title
      context.propertyTypeName = pascalCase(
        `${context.typeName} ${context.propertyName}`
      )

      const propertyPath = `${title}.${name}`

      let type =
        propertyTypes[propertyPath]?.[0] ?? schemaToType(schema, context)

      const isPointer =
        pointerProperties.includes(propertyPath) ||
        pointerProperties.includes(`*.${name}`)
      type = isPointer ? `Box<${type}>` : type

      type = optional ? `Option<${type}>` : type

      let attrs = propertyAttributes[propertyPath] ?? []
      if (isPointer) attrs = [...attrs, '#[serde(skip)]']

      return `    ${docComment(description)}
${attrs.map((attr) => `    ${attr}\n`).join('')}    pub ${snakeCase(
        name
      )}: ${type},`
    })
    .join('\n\n')

  const derives = [
    'Clone',
    'Debug',
    'Defaults',
    'Serialize',
    'Deserialize',
  ].join(', ')

  const code = `
${docComment(description)}
#[skip_serializing_none]
#[derive(${derives})]
#[serde(default, rename_all = "camelCase")]
pub struct ${title} {
    /// The name of this type
    #[def = "\\"${title}\\".to_string()"]
    #[serde(rename = "type", deserialize_with = "${title}::deserialize_type")]
    pub type_: String,

${fields}
}
impl_type!(${title});`

  return code
}

/**
 * Generate a Rust `enum` from a "enum" schema.
 */
export function enumSchemaToEnum(
  schema: JsonSchema,
  _context: Context
): string {
  const { title = '', description = title, anyOf } = schema

  const variants = anyOf
    ?.map((schema) => {
      const { description = '', const: const_ = '' } = schema
      return `    /// ${description}\n    ${const_ as string},\n`
    })
    .join('')

  return `${docComment(description)}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ${title} {\n${variants}}\n`
}

/**
 * Generate a Rust `enum` from a "union" schema.
 *
 * Needs to use `serde(untagged)` because the union may include
 * primitive types such as `Number` and `String` which can not
 * be tagged. Tagging is done within structs.
 */
export function unionSchemaToEnum(
  schema: JsonSchema,
  context: Context
): string {
  const { title = '', description = title, anyOf } = schema

  const variants = anyOf
    ?.map((schema) => {
      const name = schemaToType(schema, context)
      return name === 'Null' ? `    ${name},\n` : `    ${name}(${name}),\n`
    })
    .join('')

  return `${docComment(description)}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ${title} {\n${variants}}\n`
}

/**
 * Convert a schema definition to a Rust type
 */
function schemaToType(schema: JsonSchema, context: Context): string {
  const { type, anyOf, allOf, $ref } = schema

  if ($ref !== undefined) return `${$ref.replace('.schema.json', '')}`
  if (anyOf !== undefined) return anyOfToEnum(anyOf, context)
  if (allOf !== undefined) return allOfToType(allOf, context)
  if (schema.enum !== undefined) return enumToEnum(schema.enum, context)

  if (type === 'null') return 'Null'
  if (type === 'boolean') return 'Boolean'
  if (type === 'number') return 'Number'
  if (type === 'integer') return 'Integer'
  if (type === 'string') return 'String'
  if (type === 'array') return arrayToType(schema, context)
  if (type === 'object') return 'Object'

  throw new Error(`Unhandled schema: ${JSON.stringify(schema)}`)
}

/**
 * Convert the `anyOf` property of a JSON schema to a Rust `enum`.
 *
 * Needs to use `serde(untagged)` because the property may allow for
 * primitive types such as `Number` and `String` which can not
 * be tagged. Tagging is done within structs.
 */
function anyOfToEnum(anyOf: JsonSchema[], context: Context): string {
  const variants = anyOf
    .map((schema) => {
      const type = schemaToType(schema, context)
      const name = type.replace('<', '').replace('>', '')
      return type === 'Null' ? name : `    ${name}(${type}),\n`
    })
    .join('')

  const name = context.propertyTypeName ?? ''
  const definition = `/// Types permitted for the \`${context.propertyName}\` property of a \`${context.typeName}\` node.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ${name} {\n${variants}}\n`
  context.anonEnums[name] = definition

  return name
}

/**
 * Convert the values of an `enum` property of a JSON schema to a Rust `enum`.
 */
export function enumToEnum(enu: (string | number)[], context: Context): string {
  const lines = enu
    .map((variant) => {
      variant = typeof variant === 'string' ? variant : `V${variant}`
      return `    ${variant},\n`
    })
    .join('')

  const name = context.propertyTypeName ?? ''
  const definition = `#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ${name} {\n${lines}}\n`
  context.anonEnums[name] = definition

  return name
}

/**
 * Convert a schema with the `allOf` property to a type.
 */
function allOfToType(allOf: JsonSchema[], context: Context): string {
  if (allOf.length === 1) return schemaToType(allOf[0], context)
  else return schemaToType(allOf[allOf.length - 1], context)
}

/**
 * Convert a schema with the `array` property to an `Array` type checker.
 */
function arrayToType(schema: JsonSchema, context: Context): string {
  const items = Array.isArray(schema.items)
    ? anyOfToEnum(schema.items, context)
    : schema.items !== undefined
    ? schemaToType(schema.items, context)
    : 'ANY'
  return items === 'ANY' ? 'Array' : `Vec<${items}>`
}
