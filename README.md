prost-reflect-o3
================

Minimal python bindings for https://github.com/andrewhickman/prost-reflect/

### Usage:

```python
# create a MessageReader instance using the file-descriptor set created via protoc:
from pathlib import Path
from prost_reflect_o3 import MessageReader

file_descriptor_path = "<path>/<to>/<descriptor.bin>"
reader = MessageReader(file_descriptor_path)

# read protobuf bytes from somewhere (web, file, ...)
proto_bytes = Path("some_message.bin").read_bytes()

# deserialize the message into a generic dict
message_dict = reader.decode_message_to_dict("test.MyMessage", proto_bytes)
```
