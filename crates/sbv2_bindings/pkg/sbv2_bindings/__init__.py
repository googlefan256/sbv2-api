from typing import cast
from .types_for_python import TTSModel as TTSModelType

__lib = None

try:
    import sbv2_bindings_cpu as __lib
except:
    pass

try:
    import sbv2_bindings_directml as __lib
except:
    pass

try:
    import sbv2_bindings_coreml as __lib
except:
    pass

try:
    import sbv2_bindings_cuda as __lib
except:
    pass

if lib is None:
    raise ImportError("You should have at least one backend package installed")

TTSModel = cast(TTSModelType,__lib.TTSModel)