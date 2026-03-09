use prost::DecodeError;
use prost_reflect::DescriptorError;
use pyo3::{PyErr, exceptions::PyValueError};

pub fn decode_error_to_py(err: DecodeError) -> PyErr {
    PyValueError::new_err(format!("Failed to decode data: {}", err))
}

pub fn descriptor_error_to_py(err: DescriptorError) -> PyErr {
    PyValueError::new_err(format!("Failed to decode data: {}", err))
}
