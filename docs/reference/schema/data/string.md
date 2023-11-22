# String

**A value comprised of a string of characters.**

**`@id`**: [`schema:Text`](https://schema.org/Text)

## Formats

The `String` type can be encoded (serialized) to, and/or decoded (deserialized) from, these formats:

| Format                                                                                             | Encoding      | Decoding     | Status                 | Notes |
| -------------------------------------------------------------------------------------------------- | ------------- | ------------ | ---------------------- | ----- |
| [HTML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/html.md)              | 🟢 No loss     |              | 🚧 Under development    |       |
| [JATS](https://github.com/stencila/stencila/blob/main/docs/reference/formats/jats.md)              | 🟢 No loss     | 🟢 No loss    | 🚧 Under development    |       |
| [Markdown](https://github.com/stencila/stencila/blob/main/docs/reference/formats/markdown.md)      | 🟢 No loss     | 🟢 No loss    | ⚠️ Alpha               |       |
| [Plain text](https://github.com/stencila/stencila/blob/main/docs/reference/formats/text.md)        | 🟢 No loss     |              | ⚠️ Alpha               |       |
| [JSON](https://github.com/stencila/stencila/blob/main/docs/reference/formats/json.md)              | 🟢 No loss     | 🟢 No loss    | 🟢 Stable               |       |
| [JSON5](https://github.com/stencila/stencila/blob/main/docs/reference/formats/json5.md)            | 🟢 No loss     | 🟢 No loss    | 🟢 Stable               |       |
| [YAML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/yaml.md)              | 🟢 No loss     | 🟢 No loss    | 🟢 Stable               |       |
| [CBOR](https://github.com/stencila/stencila/blob/main/docs/reference/formats/cbor.md)              | 🟢 No loss     | 🟢 No loss    | 🟢 Stable               |       |
| [CBOR+Zstandard](https://github.com/stencila/stencila/blob/main/docs/reference/formats/cborzst.md) | 🟢 No loss     | 🟢 No loss    | 🟢 Stable               |       |
| [Debug](https://github.com/stencila/stencila/blob/main/docs/reference/formats/debug.md)            | 🔷 Low loss    |              | 🟢 Stable               |       |

## Bindings

The `String` type is represented in these bindings:

- [JSON-LD](https://stencila.dev/String.jsonld)
- [JSON Schema](https://stencila.dev/String.schema.json)
- Python type [`String`](https://github.com/stencila/stencila/blob/main/python/python/stencila/types/string.py)
- Rust type [`String`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/string.rs)
- TypeScript type [`String`](https://github.com/stencila/stencila/blob/main/typescript/src/types/String.ts)

## Source

This documentation was generated from [`String.yaml`](https://github.com/stencila/stencila/blob/main/schema/String.yaml) by [`docs.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs.rs).