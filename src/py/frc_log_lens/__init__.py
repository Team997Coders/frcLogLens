# ignore what syntax highlighting tells you-- these are perfectly fine.
# used so that a user can do "import example_lib" instead of "from example_lib import example_lib"
from .frc_log_lens import *

__doc__ = frc_log_lens.__doc__
if hasattr(frc_log_lens, "__all__"):
    __all__ = frc_log_lens.__all__
