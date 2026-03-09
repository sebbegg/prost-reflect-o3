from prost_reflect_o3 import MessageReader
from tests_pb2 import AddressBook, Person


def msg_to_bytes(msg: AddressBook | Person) -> bytes:
    return msg.SerializeToString()


def test_message_reader(file_descriptor_path: str):
    reader = MessageReader(file_descriptor_path)
    assert reader.options is not None
    assert reader.descriptor_file_path == file_descriptor_path
