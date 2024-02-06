# Generated file; do not edit. See the Rust `schema-gen` crate.

from .prelude import *

from ._author import Author
from ._automatic_execution import AutomaticExecution
from ._compilation_digest import CompilationDigest
from ._compilation_error import CompilationError
from ._duration import Duration
from ._executable import Executable
from ._execution_dependant import ExecutionDependant
from ._execution_dependency import ExecutionDependency
from ._execution_error import ExecutionError
from ._execution_required import ExecutionRequired
from ._execution_status import ExecutionStatus
from ._execution_tag import ExecutionTag
from ._message import Message
from ._timestamp import Timestamp


@dataclass(init=False)
class Instruction(Executable):
    """
    Abstract base type for a document editing instruction.
    """

    type: Literal["Instruction"] = field(default="Instruction", init=False)

    messages: List[Message]
    """Messages involved in the instruction."""

    candidates: Optional[List[str]] = None
    """A list of candidates for the assignee property."""

    assignee: Optional[str] = None
    """An identifier for the agent assigned to perform the instruction"""

    authors: Optional[List[Author]] = None
    """The authors of the instruction."""

    def __init__(self, messages: List[Message], id: Optional[str] = None, auto_exec: Optional[AutomaticExecution] = None, compilation_digest: Optional[CompilationDigest] = None, compilation_errors: Optional[List[CompilationError]] = None, execution_digest: Optional[CompilationDigest] = None, execution_dependencies: Optional[List[ExecutionDependency]] = None, execution_dependants: Optional[List[ExecutionDependant]] = None, execution_tags: Optional[List[ExecutionTag]] = None, execution_count: Optional[int] = None, execution_required: Optional[ExecutionRequired] = None, execution_status: Optional[ExecutionStatus] = None, execution_actor: Optional[str] = None, execution_ended: Optional[Timestamp] = None, execution_duration: Optional[Duration] = None, execution_errors: Optional[List[ExecutionError]] = None, candidates: Optional[List[str]] = None, assignee: Optional[str] = None, authors: Optional[List[Author]] = None):
        super().__init__(id = id, auto_exec = auto_exec, compilation_digest = compilation_digest, compilation_errors = compilation_errors, execution_digest = execution_digest, execution_dependencies = execution_dependencies, execution_dependants = execution_dependants, execution_tags = execution_tags, execution_count = execution_count, execution_required = execution_required, execution_status = execution_status, execution_actor = execution_actor, execution_ended = execution_ended, execution_duration = execution_duration, execution_errors = execution_errors)
        self.messages = messages
        self.candidates = candidates
        self.assignee = assignee
        self.authors = authors