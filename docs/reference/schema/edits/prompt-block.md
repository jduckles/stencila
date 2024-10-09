# Prompt Block

**A preview of how a prompt will be rendered at a location in the document**

Used on an `Instruction` to render a prompt and display the rendering to the user.
Can also be used standalone to preview how a prompt is rendered at a particular
position in a document.


**`@id`**: `stencila:PromptBlock`

This type is marked as experimental and is likely to change.

## Properties

The `PromptBlock` type has these properties:

| Name                    | Aliases                                                                                                                   | `@id`                                | Type                                                                                                                        | Description                                                       | Inherited from                                                                                          |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------- | ------------------------------------ | --------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| `id`                    | -                                                                                                                         | [`schema:id`](https://schema.org/id) | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)                             | The identifier for this item.                                     | [`Entity`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/other/entity.md)        |
| `executionMode`         | `execution-mode`, `execution_mode`                                                                                        | `stencila:executionMode`             | [`ExecutionMode`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/execution-mode.md)              | Under which circumstances the code should be executed.            | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md) |
| `compilationDigest`     | `compilation-digest`, `compilation_digest`                                                                                | `stencila:compilationDigest`         | [`CompilationDigest`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/compilation-digest.md)      | A digest of the content, semantics and dependencies of the node.  | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md) |
| `compilationMessages`   | `compilation-messages`, `compilation_messages`, `compilationMessage`, `compilation-message`, `compilation_message`        | `stencila:compilationMessages`       | [`CompilationMessage`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/code/compilation-message.md)*   | Messages generated while compiling the code.                      | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md) |
| `executionDigest`       | `execution-digest`, `execution_digest`                                                                                    | `stencila:executionDigest`           | [`CompilationDigest`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/compilation-digest.md)      | The `compilationDigest` of the node when it was last executed.    | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md) |
| `executionDependencies` | `execution-dependencies`, `execution_dependencies`, `executionDependency`, `execution-dependency`, `execution_dependency` | `stencila:executionDependencies`     | [`ExecutionDependency`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/execution-dependency.md)* | The upstream dependencies of this node.                           | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md) |
| `executionDependants`   | `execution-dependants`, `execution_dependants`, `executionDependant`, `execution-dependant`, `execution_dependant`        | `stencila:executionDependants`       | [`ExecutionDependant`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/execution-dependant.md)*   | The downstream dependants of this node.                           | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md) |
| `executionTags`         | `execution-tags`, `execution_tags`, `executionTag`, `execution-tag`, `execution_tag`                                      | `stencila:executionTags`             | [`ExecutionTag`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/execution-tag.md)*               | Tags in the code which affect its execution.                      | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md) |
| `executionCount`        | `execution-count`, `execution_count`                                                                                      | `stencila:executionCount`            | [`Integer`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/integer.md)                           | A count of the number of times that the node has been executed.   | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md) |
| `executionRequired`     | `execution-required`, `execution_required`                                                                                | `stencila:executionRequired`         | [`ExecutionRequired`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/execution-required.md)      | Whether, and why, the code requires execution or re-execution.    | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md) |
| `executionStatus`       | `execution-status`, `execution_status`                                                                                    | `stencila:executionStatus`           | [`ExecutionStatus`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/execution-status.md)          | Status of the most recent, including any current, execution.      | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md) |
| `executionKind`         | `execution-kind`, `execution_kind`                                                                                        | `stencila:executionKind`             | [`ExecutionKind`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/execution-kind.md)              | The kind (e.g. main kernel vs kernel fork) of the last execution. | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md) |
| `executionEnded`        | `execution-ended`, `execution_ended`                                                                                      | `stencila:executionEnded`            | [`Timestamp`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/timestamp.md)                       | The timestamp when the last execution ended.                      | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md) |
| `executionDuration`     | `execution-duration`, `execution_duration`                                                                                | `stencila:executionDuration`         | [`Duration`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/duration.md)                         | Duration of the last execution.                                   | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md) |
| `executionMessages`     | `execution-messages`, `execution_messages`, `executionMessage`, `execution-message`, `execution_message`                  | `stencila:executionMessages`         | [`ExecutionMessage`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/code/execution-message.md)*       | Messages emitted while executing the node.                        | [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md) |
| `prompt`                | -                                                                                                                         | `stencila:prompt`                    | [`String`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/data/string.md)                             | An identifier for the prompt to be rendered                       | -                                                                                                       |
| `content`               | -                                                                                                                         | `stencila:content`                   | [`Block`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/prose/block.md)*                             | The executed content of the prompt                                | -                                                                                                       |

## Related

The `PromptBlock` type is related to these types:

- Parents: [`Executable`](https://github.com/stencila/stencila/blob/main/docs/reference/schema/flow/executable.md)
- Children: none

## Formats

The `PromptBlock` type can be encoded (serialized) to, and/or decoded (deserialized) from, these formats:

| Format                                                                                               | Encoding     | Decoding  | Status              | Notes                              |
| ---------------------------------------------------------------------------------------------------- | ------------ | --------- | ------------------- | ---------------------------------- |
| [DOM HTML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/dom.html.md)        | 🟢 No loss    |           | 🚧 Under development |                                    |
| [HTML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/html.md)                | 🔷 Low loss   |           | 🚧 Under development |                                    |
| [JATS](https://github.com/stencila/stencila/blob/main/docs/reference/formats/jats.md)                |              |           | 🚧 Under development |                                    |
| [Markdown](https://github.com/stencila/stencila/blob/main/docs/reference/formats/markdown.md)        | ⚠️ High loss |           | ⚠️ Alpha            | Encoded using implemented function |
| [MyST](https://github.com/stencila/stencila/blob/main/docs/reference/formats/myst.md)                | ⚠️ High loss |           | ⚠️ Alpha            |                                    |
| [LLM Markdown](https://github.com/stencila/stencila/blob/main/docs/reference/formats/llmd.md)        | ⚠️ High loss |           | ⚠️ Alpha            |                                    |
| [Plain text](https://github.com/stencila/stencila/blob/main/docs/reference/formats/text.md)          | ⚠️ High loss |           | ⚠️ Alpha            |                                    |
| [JSON](https://github.com/stencila/stencila/blob/main/docs/reference/formats/json.md)                | 🟢 No loss    | 🟢 No loss | 🟢 Stable            |                                    |
| [JSON+Zip](https://github.com/stencila/stencila/blob/main/docs/reference/formats/json.zip.md)        | 🟢 No loss    | 🟢 No loss | 🟢 Stable            |                                    |
| [JSON5](https://github.com/stencila/stencila/blob/main/docs/reference/formats/json5.md)              | 🟢 No loss    | 🟢 No loss | 🟢 Stable            |                                    |
| [JSON-LD](https://github.com/stencila/stencila/blob/main/docs/reference/formats/jsonld.md)           | 🟢 No loss    | 🟢 No loss | 🔶 Beta              |                                    |
| [CBOR](https://github.com/stencila/stencila/blob/main/docs/reference/formats/cbor.md)                | 🟢 No loss    | 🟢 No loss | 🟢 Stable            |                                    |
| [CBOR+Zstandard](https://github.com/stencila/stencila/blob/main/docs/reference/formats/cbor.zstd.md) | 🟢 No loss    | 🟢 No loss | 🟢 Stable            |                                    |
| [YAML](https://github.com/stencila/stencila/blob/main/docs/reference/formats/yaml.md)                | 🟢 No loss    | 🟢 No loss | 🟢 Stable            |                                    |
| [Directory](https://github.com/stencila/stencila/blob/main/docs/reference/formats/directory.md)      |              |           | 🚧 Under development |                                    |
| [Stencila Web Bundle](https://github.com/stencila/stencila/blob/main/docs/reference/formats/swb.md)  |              |           | 🚧 Under development |                                    |
| [Debug](https://github.com/stencila/stencila/blob/main/docs/reference/formats/debug.md)              | 🔷 Low loss   |           | 🟢 Stable            |                                    |

## Bindings

The `PromptBlock` type is represented in these bindings:

- [JSON-LD](https://stencila.org/PromptBlock.jsonld)
- [JSON Schema](https://stencila.org/PromptBlock.schema.json)
- Python class [`PromptBlock`](https://github.com/stencila/stencila/blob/main/python/python/stencila/types/prompt_block.py)
- Rust struct [`PromptBlock`](https://github.com/stencila/stencila/blob/main/rust/schema/src/types/prompt_block.rs)
- TypeScript class [`PromptBlock`](https://github.com/stencila/stencila/blob/main/ts/src/types/PromptBlock.ts)

## Source

This documentation was generated from [`PromptBlock.yaml`](https://github.com/stencila/stencila/blob/main/schema/PromptBlock.yaml) by [`docs_type.rs`](https://github.com/stencila/stencila/blob/main/rust/schema-gen/src/docs_type.rs).