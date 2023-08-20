# Generated file; do not edit. See the Rust `schema-gen` crate.

from .prelude import *

from .date_time import DateTime
from .entity import Entity


@dataclass(kw_only=True, frozen=True)
class DateTimeValidator(Entity):
    """
    A validator specifying the constraints on a date-time.
    """

    type: Literal["DateTimeValidator"] = field(default="DateTimeValidator", init=False)

    minimum: Optional[DateTime] = None
    """The inclusive lower limit for a date-time."""

    maximum: Optional[DateTime] = None
    """The inclusive upper limit for a date-time."""
