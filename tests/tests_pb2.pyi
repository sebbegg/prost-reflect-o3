from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from collections.abc import Iterable as _Iterable, Mapping as _Mapping
from typing import ClassVar as _ClassVar, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class Person(_message.Message):
    __slots__ = ()
    class PhoneType(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
        __slots__ = ()
        PHONE_TYPE_UNSPECIFIED: _ClassVar[Person.PhoneType]
        PHONE_TYPE_MOBILE: _ClassVar[Person.PhoneType]
        PHONE_TYPE_HOME: _ClassVar[Person.PhoneType]
        PHONE_TYPE_WORK: _ClassVar[Person.PhoneType]

    PHONE_TYPE_UNSPECIFIED: Person.PhoneType
    PHONE_TYPE_MOBILE: Person.PhoneType
    PHONE_TYPE_HOME: Person.PhoneType
    PHONE_TYPE_WORK: Person.PhoneType
    class PhoneNumber(_message.Message):
        __slots__ = ()
        NUMBER_FIELD_NUMBER: _ClassVar[int]
        TYPE_FIELD_NUMBER: _ClassVar[int]
        number: str
        type: Person.PhoneType
        def __init__(
            self,
            number: _Optional[str] = ...,
            type: _Optional[_Union[Person.PhoneType, str]] = ...,
        ) -> None: ...

    class TagsEntry(_message.Message):
        __slots__ = ()
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(
            self, key: _Optional[str] = ..., value: _Optional[str] = ...
        ) -> None: ...

    NAME_FIELD_NUMBER: _ClassVar[int]
    ID_FIELD_NUMBER: _ClassVar[int]
    EMAIL_FIELD_NUMBER: _ClassVar[int]
    PHONES_FIELD_NUMBER: _ClassVar[int]
    TAGS_FIELD_NUMBER: _ClassVar[int]
    ONEOF_STRING_FIELD_NUMBER: _ClassVar[int]
    ONEOF_INT_FIELD_NUMBER: _ClassVar[int]
    ONEOF_PHONE_FIELD_NUMBER: _ClassVar[int]
    name: str
    id: int
    email: str
    phones: _containers.RepeatedCompositeFieldContainer[Person.PhoneNumber]
    tags: _containers.ScalarMap[str, str]
    oneof_string: str
    oneof_int: int
    oneof_phone: Person.PhoneNumber
    def __init__(
        self,
        name: _Optional[str] = ...,
        id: _Optional[int] = ...,
        email: _Optional[str] = ...,
        phones: _Optional[_Iterable[_Union[Person.PhoneNumber, _Mapping]]] = ...,
        tags: _Optional[_Mapping[str, str]] = ...,
        oneof_string: _Optional[str] = ...,
        oneof_int: _Optional[int] = ...,
        oneof_phone: _Optional[_Union[Person.PhoneNumber, _Mapping]] = ...,
    ) -> None: ...

class AddressBook(_message.Message):
    __slots__ = ()
    PEOPLE_FIELD_NUMBER: _ClassVar[int]
    people: _containers.RepeatedCompositeFieldContainer[Person]
    def __init__(
        self, people: _Optional[_Iterable[_Union[Person, _Mapping]]] = ...
    ) -> None: ...
