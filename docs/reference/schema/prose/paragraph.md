# Paragraph

**A paragraph.**

Analogues of `Paragraph` in other schema include:
  - HTML [`<p>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/p)
  - JATS XML [`<p>`](https://jats.nlm.nih.gov/articleauthoring/tag-library/1.2/element/p.html)
  - MDAST [`Paragraph`](https://github.com/syntax-tree/mdast#Paragraph)
  - OpenDocument [`<text:p>`](http://docs.oasis-open.org/office/v1.2/os/OpenDocument-v1.2-os-part1.html#__RefHeading__1415138_253892949)
  - Pandoc [`Para`](https://github.com/jgm/pandoc-types/blob/1.17.5.4/Text/Pandoc/Definition.hs#L220)


**`@id`**: `stencila:Paragraph`

## Properties

The `Paragraph` type has these properties:

| Name      | Aliases | `@id`                                | Type                                                                                              | Description                    | Inherited from                                                                                   |
| --------- | ------- | ------------------------------------ | ------------------------------------------------------------------------------------------------- | ------------------------------ | ------------------------------------------------------------------------------------------------ |
| `id`      | -       | [`schema:id`](https://schema.org/id) | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)   | The identifier for this item.  | [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md) |
| `content` | -       | `stencila:content`                   | [`Inline`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/inline.md)* | The contents of the paragraph. | -                                                                                                |

## Related

The `Paragraph` type is related to these types:

- Parents: [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md)
- Children: none

## Formats

The `Paragraph` type can be encoded (serialized) to, and/or decoded (deserialized) from, these formats:

| Format                                                                                        | Encoding         | Decoding     | Status                 | Notes                                                                                        |
| --------------------------------------------------------------------------------------------- | ---------------- | ------------ | ---------------------- | -------------------------------------------------------------------------------------------- |
| [HTML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/html.md)         | 🟢 No loss        |              | 🚧 Under development    | Encoded as [`<p>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/p)              |
| [JATS](https://github.com/stencila/stencila/blob/main/docs/reference/formats/jats.md)         | 🟢 No loss        | 🟢 No loss    | 🚧 Under development    | Encoded as [`<p>`](https://jats.nlm.nih.gov/articleauthoring/tag-library/1.3/element/p.html) |
| [Markdown](https://github.com/stencila/stencila/blob/main/docs/reference/formats/markdown.md) | 🟢 No loss        | 🟢 No loss    | 🚧 Under development    | Encoded as `{content}\n\n`                                                                   |
| [Plain text](https://github.com/stencila/stencila/blob/main/docs/reference/formats/text.md)   | ⚠️ High loss     |              | ⚠️ Alpha               |                                                                                              |
| [JSON](https://github.com/stencila/stencila/blob/main/docs/reference/formats/json.md)         | 🟢 No loss        | 🟢 No loss    | 🟢 Stable               |                                                                                              |
| [JSON5](https://github.com/stencila/stencila/blob/main/docs/reference/formats/json5.md)       | 🟢 No loss        | 🟢 No loss    | 🟢 Stable               |                                                                                              |
| [YAML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/yaml.md)         | 🟢 No loss        | 🟢 No loss    | 🟢 Stable               |                                                                                              |
| [Debug](https://github.com/stencila/stencila/blob/main/docs/reference/formats/debug.md)       | 🔷 Low loss       |              | 🟢 Stable               |                                                                                              |

## Bindings

The `Paragraph` type is represented in these bindings:

- [JSON-LD](https://stencila.dev/Paragraph.jsonld)
- [JSON Schema](https://stencila.dev/Paragraph.schema.json)
- Python class [`Paragraph`](https://github.com/stencila/stencila/blob/main/python/python/stencila/types/paragraph.py)
- Rust struct [`Paragraph`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/paragraph.rs)
- TypeScript class [`Paragraph`](https://github.com/stencila/stencila/blob/main/typescript/src/types/Paragraph.ts)

## Testing

During property-based (a.k.a generative) testing, the properties of the `Paragraph` type are generated using the following strategies for each complexity level (see the [`proptest` book](https://proptest-rs.github.io/proptest/) for an explanation of the Rust strategy expressions). Any optional properties that are not in this table are set to `None`.

| Property  | Complexity | Description                                                                     | Strategy                                      |
| --------- | ---------- | ------------------------------------------------------------------------------- | --------------------------------------------- |
| `content` | Min+       | Generate a single arbitrary inline node                                         | `vec_inlines(1)`                              |
|           | Low+       | Generate up to two arbitrary inline nodes                                       | `vec_inlines(2)`                              |
|           | High+      | Generate up to four arbitrary inline nodes                                      | `vec_inlines(4)`                              |
|           | Max        | Generate up to eight arbitrary inline nodes without restrictions on their order | `vec(Inline::arbitrary(), size_range(0..=8))` |

## Source

This documentation was generated from [`Paragraph.yaml`](https://github.com/stencila/stencila/blob/main/schema/Paragraph.yaml) by [`docs.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs.rs).