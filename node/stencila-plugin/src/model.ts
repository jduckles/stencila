/* eslint-disable @typescript-eslint/no-unused-vars */

import { AuthorRole, InstructionMessage } from "@stencila/types";

/**
 * The type for the name of an model
 */
export type ModelName = string;

/**
 * A model content generation task
 */
export interface ModelTask {
  /**
   * The kind of task
   */
  kind: "message-generation" | "image-generation";

  /**
   * The messages of the task
   */
  messages: InstructionMessage[];

  /**
   * The desired format of the generated content
   */
  format: string;

  /**
   * Other options e.g. temperature
   */
  [key: string]: unknown;
}

/**
 * The output generated by an model
 */
export interface ModelOutput {
  /**
   * The authors of the generated content
   *
   * Should be a `SoftwareApplication` describing the model.
   */
  authors: AuthorRole[];

  /**
   * The kind of the generated content
   *
   * Used by Stencila to determine how to handle the `content` before
   * decoding it into nodes.
   */
  kind: "text" | "url";

  /**
   * The format of the generated content
   *
   * Used by Stencila to decode the generated `content` into a set of
   * Stencila Schema nodes.
   */
  format: string;

  /**
   * The content generated by the model
   */
  content: string;
}

export abstract class Model {
  /**
   * Perform a task
   *
   * @param task The task to perform
   * @return ModelOutput
   */
  async performTask(task: ModelTask): Promise<ModelOutput> {
    throw new Error(
      "Method `performTask` must be implemented by plugins that provide a model"
    );
  }
}
