# Table Cell

**A cell within a `Table`.**

**`@id`**: `stencila:TableCell`

## Properties

The `TableCell` type has these properties:

| Name         | Aliases                      | `@id`                                    | Type                                                                                                             | Description                        | Inherited from                                                                                   |
| ------------ | ---------------------------- | ---------------------------------------- | ---------------------------------------------------------------------------------------------------------------- | ---------------------------------- | ------------------------------------------------------------------------------------------------ |
| `id`         | -                            | [`schema:id`](https://schema.org/id)     | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)                  | The identifier for this item.      | [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md) |
| `cellType`   | `cell-type`, `cell_type`     | `stencila:cellType`                      | [`TableCellType`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/works/table-cell-type.md) | The type of cell.                  | -                                                                                                |
| `name`       | -                            | [`schema:name`](https://schema.org/name) | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)                  | The name of the cell.              | -                                                                                                |
| `columnSpan` | `column-span`, `column_span` | `stencila:colspan`                       | [`Integer`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/integer.md)                | How many columns the cell extends. | -                                                                                                |
| `rowSpan`    | `row-span`, `row_span`       | `stencila:rowspan`                       | [`Integer`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/integer.md)                | How many columns the cell extends. | -                                                                                                |
| `content`    | -                            | `stencila:content`                       | [`Block`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/block.md)*                  | Contents of the table cell.        | -                                                                                                |

## Related

The `TableCell` type is related to these types:

- Parents: [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md)
- Children: none

## Formats

The `TableCell` type can be encoded (serialized) to, and/or decoded (deserialized) from, these formats:

| Format                                                                                        | Encoding         | Decoding     | Status                 | Notes                                                                             |
| --------------------------------------------------------------------------------------------- | ---------------- | ------------ | ---------------------- | --------------------------------------------------------------------------------- |
| [HTML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/html.md)         | 🔷 Low loss       |              | 🚧 Under development    | Encoded as [`<td>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/td) |
| [JATS](https://github.com/stencila/stencila/blob/main/docs/reference/formats/jats.md)         |                  |              | 🚧 Under development    |                                                                                   |
| [Markdown](https://github.com/stencila/stencila/blob/main/docs/reference/formats/markdown.md) | 🔷 Low loss       |              | 🚧 Under development    |                                                                                   |
| [Plain text](https://github.com/stencila/stencila/blob/main/docs/reference/formats/text.md)   | ⚠️ High loss     |              | ⚠️ Alpha               |                                                                                   |
| [JSON](https://github.com/stencila/stencila/blob/main/docs/reference/formats/json.md)         | 🟢 No loss        | 🟢 No loss    | 🟢 Stable               |                                                                                   |
| [JSON5](https://github.com/stencila/stencila/blob/main/docs/reference/formats/json5.md)       | 🟢 No loss        | 🟢 No loss    | 🟢 Stable               |                                                                                   |
| [YAML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/yaml.md)         | 🟢 No loss        | 🟢 No loss    | 🟢 Stable               |                                                                                   |
| [Debug](https://github.com/stencila/stencila/blob/main/docs/reference/formats/debug.md)       | 🔷 Low loss       |              | 🟢 Stable               |                                                                                   |

## Bindings

The `TableCell` type is represented in these bindings:

- [JSON-LD](https://stencila.dev/TableCell.jsonld)
- [JSON Schema](https://stencila.dev/TableCell.schema.json)
- Python class [`TableCell`](https://github.com/stencila/stencila/blob/main/python/python/stencila/types/table_cell.py)
- Rust struct [`TableCell`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/table_cell.rs)
- TypeScript class [`TableCell`](https://github.com/stencila/stencila/blob/main/typescript/src/types/TableCell.ts)

## Testing

During property-based (a.k.a generative) testing, the properties of the `TableCell` type are generated using the following strategies for each complexity level (see the [`proptest` book](https://proptest-rs.github.io/proptest/) for an explanation of the Rust strategy expressions). Any optional properties that are not in this table are set to `None`.

| Property  | Complexity | Description                             | Strategy            |
| --------- | ---------- | --------------------------------------- | ------------------- |
| `content` | Min+       | Generate a single, arbitrary, paragraph | `vec_paragraphs(1)` |

## Source

This documentation was generated from [`TableCell.yaml`](https://github.com/stencila/stencila/blob/main/schema/TableCell.yaml) by [`docs.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs.rs).