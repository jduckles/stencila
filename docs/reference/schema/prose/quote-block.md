# Quote Block

**A section quoted from somewhere else.**

**`@id`**: `stencila:QuoteBlock`

## Properties

The `QuoteBlock` type has these properties:

| Name      | Aliases | `@id`                                | Type                                                                                                                                                                                        | Description                   | Inherited from                                                                                   |
| --------- | ------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------- | ------------------------------------------------------------------------------------------------ |
| `id`      | -       | [`schema:id`](https://schema.org/id) | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)                                                                                             | The identifier for this item. | [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md) |
| `cite`    | -       | `stencila:cite`                      | [`Cite`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/cite.md) \| [`Text`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/text.md) | The source of the quote.      | -                                                                                                |
| `content` | -       | `stencila:content`                   | [`Block`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/block.md)*                                                                                             | The content of the quote.     | -                                                                                                |

## Related

The `QuoteBlock` type is related to these types:

- Parents: [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md)
- Children: none

## Formats

The `QuoteBlock` type can be encoded (serialized) to, and/or decoded (deserialized) from, these formats:

| Format                                                                                        | Encoding         | Decoding     | Status                 | Notes |
| --------------------------------------------------------------------------------------------- | ---------------- | ------------ | ---------------------- | ----- |
| [HTML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/html.md)         | 🔷 Low loss       |              | 🚧 Under development    |       |
| [JATS](https://github.com/stencila/stencila/blob/main/docs/reference/formats/jats.md)         |                  |              | 🚧 Under development    |       |
| [Markdown](https://github.com/stencila/stencila/blob/main/docs/reference/formats/markdown.md) | ⚠️ High loss     |              | 🚧 Under development    |       |
| [Plain text](https://github.com/stencila/stencila/blob/main/docs/reference/formats/text.md)   | ⚠️ High loss     |              | ⚠️ Alpha               |       |
| [JSON](https://github.com/stencila/stencila/blob/main/docs/reference/formats/json.md)         | 🟢 No loss        | 🟢 No loss    | 🟢 Stable               |       |
| [JSON5](https://github.com/stencila/stencila/blob/main/docs/reference/formats/json5.md)       | 🟢 No loss        | 🟢 No loss    | 🟢 Stable               |       |
| [YAML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/yaml.md)         | 🟢 No loss        | 🟢 No loss    | 🟢 Stable               |       |
| [Debug](https://github.com/stencila/stencila/blob/main/docs/reference/formats/debug.md)       | 🔷 Low loss       |              | 🟢 Stable               |       |

## Bindings

The `QuoteBlock` type is represented in these bindings:

- [JSON-LD](https://stencila.dev/QuoteBlock.jsonld)
- [JSON Schema](https://stencila.dev/QuoteBlock.schema.json)
- Python class [`QuoteBlock`](https://github.com/stencila/stencila/blob/main/python/python/stencila/types/quote_block.py)
- Rust struct [`QuoteBlock`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/quote_block.rs)
- TypeScript class [`QuoteBlock`](https://github.com/stencila/stencila/blob/main/typescript/src/types/QuoteBlock.ts)

## Testing

During property-based (a.k.a generative) testing, the properties of the `QuoteBlock` type are generated using the following strategies for each complexity level (see the [`proptest` book](https://proptest-rs.github.io/proptest/) for an explanation of the Rust strategy expressions). Any optional properties that are not in this table are set to `None`.

| Property  | Complexity | Description                                                 | Strategy                      |
| --------- | ---------- | ----------------------------------------------------------- | ----------------------------- |
| `content` | Min+       | Generate a single arbitrary paragraph.                      | `vec_paragraphs(1)`           |
|           | Low+       | Generate up to two arbitrary, non-recursive, block nodes.   | `vec_blocks_non_recursive(2)` |
|           | High+      | Generate up to four arbitrary, non-recursive, block nodes.  | `vec_blocks_non_recursive(4)` |
|           | Max        | Generate up to eight arbitrary, non-recursive, block nodes. | `vec_blocks_non_recursive(8)` |

## Source

This documentation was generated from [`QuoteBlock.yaml`](https://github.com/stencila/stencila/blob/main/schema/QuoteBlock.yaml) by [`docs.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs.rs).