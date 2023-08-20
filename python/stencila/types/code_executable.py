# Generated file; do not edit. See the Rust `schema-gen` crate.

from .prelude import *

from .executable import Executable


@dataclass(kw_only=True, frozen=True)
class CodeExecutable(Executable):
    """
    Abstract base type for executable code nodes (e.g. `CodeChunk`).
    """

    type: Literal["CodeExecutable"] = field(default="CodeExecutable", init=False)

    code: str
    """The code."""

    programming_language: str
    """The programming language of the code."""

    guess_language: Optional[bool] = None
    """Whether the programming language of the code should be guessed based on syntax and variables used"""

    media_type: Optional[str] = None
    """Media type, typically expressed using a MIME format, of the code."""
