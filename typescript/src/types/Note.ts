// Generated file; do not edit. See `../rust/schema-gen` crate.

import { Block } from "./Block.js";
import { Entity } from "./Entity.js";
import { NoteType } from "./NoteType.js";

// Additional content which is not part of the main content of a document.
export class Note extends Entity {
  type = "Note";

  // Determines where the note content is displayed within the document.
  noteType: NoteType;

  // Content of the note, usually a paragraph.
  content: Block[];

  constructor(noteType: NoteType, content: Block[], options?: Note) {
    super();
    if (options) Object.assign(this, options);
    this.noteType = noteType;
    this.content = content;
  }

  static from(other: Note): Note {
    return new Note(other.noteType!, other.content!, other);
  }
}
