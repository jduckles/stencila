# Code Fragment

**Inline code.**

**`@id`**: `stencila:CodeFragment`

## Properties

The `CodeFragment` type has these properties:

| Name                  | Aliases                                        | `@id`                                                                  | Type                                                                                            | Description                           | Inherited from                                                                                           |
| --------------------- | ---------------------------------------------- | ---------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- | ------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| `id`                  | -                                              | [`schema:id`](https://schema.org/id)                                   | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md) | The identifier for this item.         | [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md)         |
| `code`                | -                                              | `stencila:code`                                                        | [`Cord`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/cord.md)     | The code.                             | [`CodeStatic`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/code/code-static.md) |
| `programmingLanguage` | `programming-language`, `programming_language` | [`schema:programmingLanguage`](https://schema.org/programmingLanguage) | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md) | The programming language of the code. | [`CodeStatic`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/code/code-static.md) |

## Related

The `CodeFragment` type is related to these types:

- Parents: [`CodeStatic`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/code/code-static.md)
- Children: none

## Formats

The `CodeFragment` type can be encoded (serialized) to, and/or decoded (deserialized) from, these formats:

| Format                                                                                        | Encoding         | Decoding     | Status                 | Notes                                                                                              |
| --------------------------------------------------------------------------------------------- | ---------------- | ------------ | ---------------------- | -------------------------------------------------------------------------------------------------- |
| [HTML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/html.md)         | 🟢 No loss        |              | 🚧 Under development    | Encoded as [`<code>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/code)              |
| [JATS](https://github.com/stencila/stencila/blob/main/docs/reference/formats/jats.md)         | 🟢 No loss        | 🟢 No loss    | 🚧 Under development    | Encoded as [`<code>`](https://jats.nlm.nih.gov/articleauthoring/tag-library/1.3/element/code.html) |
| [Markdown](https://github.com/stencila/stencila/blob/main/docs/reference/formats/markdown.md) | 🟢 No loss        | 🟢 No loss    | ⚠️ Alpha               | Encoded using special function                                                                     |
| [Plain text](https://github.com/stencila/stencila/blob/main/docs/reference/formats/text.md)   | ⚠️ High loss     |              | ⚠️ Alpha               |                                                                                                    |
| [JSON](https://github.com/stencila/stencila/blob/main/docs/reference/formats/json.md)         | 🟢 No loss        | 🟢 No loss    | 🟢 Stable               |                                                                                                    |
| [JSON5](https://github.com/stencila/stencila/blob/main/docs/reference/formats/json5.md)       | 🟢 No loss        | 🟢 No loss    | 🟢 Stable               |                                                                                                    |
| [YAML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/yaml.md)         | 🟢 No loss        | 🟢 No loss    | 🟢 Stable               |                                                                                                    |
| [Debug](https://github.com/stencila/stencila/blob/main/docs/reference/formats/debug.md)       | 🔷 Low loss       |              | 🟢 Stable               |                                                                                                    |

## Bindings

The `CodeFragment` type is represented in these bindings:

- [JSON-LD](https://stencila.dev/CodeFragment.jsonld)
- [JSON Schema](https://stencila.dev/CodeFragment.schema.json)
- Python class [`CodeFragment`](https://github.com/stencila/stencila/blob/main/python/python/stencila/types/code_fragment.py)
- Rust struct [`CodeFragment`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/code_fragment.rs)
- TypeScript class [`CodeFragment`](https://github.com/stencila/stencila/blob/main/typescript/src/types/CodeFragment.ts)

## Testing

During property-based (a.k.a generative) testing, the properties of the `CodeFragment` type are generated using the following strategies for each complexity level (see the [`proptest` book](https://proptest-rs.github.io/proptest/) for an explanation of the Rust strategy expressions). Any optional properties that are not in this table are set to `None`.

| Property              | Complexity | Description                                                                                                                    | Strategy                                   |
| --------------------- | ---------- | ------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------ |
| `code`                | Min+       | Generate a simple fixed string of code.                                                                                        | `Cord::new("code")`                        |
|                       | Low+       | Generate a random string of up to 10 alphanumeric characters (exclude whitespace which<br><br>can be problematic in Markdown). | `r"[a-zA-Z0-9]{1,10}".prop_map(Cord::new)` |
|                       | High+      | Generate a random string of up to 100 characters (excluding control characters).                                               | `r"[^\p{C}]{1,100}".prop_map(Cord::new)`   |
|                       | Max        | Generate an arbitrary string.                                                                                                  | `String::arbitrary().prop_map(Cord::new)`  |
| `programmingLanguage` | Min+       | Do not generate a programming language.                                                                                        | `None`                                     |
|                       | Low+       | Generate one of the well known programming language short names.                                                               | `option::of(r"(cpp)\|(js)\|(py)\|(r)\|(ts)")` |
|                       | High+      | Generate a random string of up to 10 alphanumeric characters.                                                                  | `option::of(r"[a-zA-Z0-9]{1,10}")`         |
|                       | Max        | Generate an arbitrary string.                                                                                                  | `option::of(String::arbitrary())`          |

## Source

This documentation was generated from [`CodeFragment.yaml`](https://github.com/stencila/stencila/blob/main/schema/CodeFragment.yaml) by [`docs.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs.rs).