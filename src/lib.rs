mod errors;

#[pyo3::pymodule]
mod prost_reflect_o3 {
    use std::{default::Default, fs::File, io::Read};

    use bytes::Bytes;
    use prost_reflect::{
        DescriptorPool, DynamicMessage, FieldDescriptor, ReflectMessage, SerializeOptions, Value,
    };
    use pyo3::{
        PyResult, Python,
        exceptions::PyTypeError,
        prelude::*,
        types::{PyBool, PyBytes, PyDict, PyFloat, PyInt, PyList, PyString},
    };
    use titlecase::Titlecase;

    use crate::errors::{decode_error_to_py, descriptor_error_to_py};

    #[pyclass(from_py_object)]
    #[derive(Debug, Clone)]
    pub struct Options {
        enums_as_strings: bool,
        render_defaults: bool,
        use_json_names: bool,
    }

    impl Default for Options {
        fn default() -> Self {
            Self {
                enums_as_strings: true,
                render_defaults: false,
                use_json_names: false,
            }
        }
    }

    #[pymethods]
    impl Options {
        #[new]
        fn new(enums_as_strings: bool, render_defaults: bool, use_json_names: bool) -> Self {
            Self {
                enums_as_strings,
                render_defaults,
                use_json_names,
            }
        }

        pub fn __str__(&self) -> String {
            format!(
                "Options(enums_as_strings={}, render_defaults={}, use_json_names={})",
                format!("{}", self.enums_as_strings).titlecase(),
                format!("{}", self.render_defaults).titlecase(),
                format!("{}", self.use_json_names).titlecase()
            )
        }
    }

    #[pyclass]
    struct MessageReader {
        descriptor_pool: DescriptorPool,

        #[pyo3(get)]
        descriptor_file_path: String,

        #[pyo3(get, set)]
        options: Options,
    }

    #[pymethods]
    impl MessageReader {
        #[new]
        #[pyo3(signature = (descriptor_file_path, options=Options::default()))]
        fn new(descriptor_file_path: &str, options: Options) -> PyResult<Self> {
            let mut desc_file = File::open(descriptor_file_path)?;
            let mut buf = vec![];
            desc_file.read_to_end(&mut buf)?;

            let descriptor_pool =
                DescriptorPool::decode(Bytes::from(buf)).map_err(descriptor_error_to_py)?;
            Ok(Self {
                descriptor_file_path: descriptor_file_path.to_owned(),
                descriptor_pool,
                options: options.clone(),
            })
        }

        fn __str__(&self) -> String {
            format!(
                "MessageReader(descriptor_file_path='{}')",
                self.descriptor_file_path
            )
        }

        fn decode_message_to_dict<'py>(
            &self,
            py: Python<'py>,
            message_name: &str,
            data: Vec<u8>,
        ) -> PyResult<Bound<'py, PyDict>> {
            let msg = self.try_decode_message_by_name(message_name, data)?;

            let msg_value = Value::Message(msg);
            let result: Bound<'py, PyDict> = self
                .proto_value_to_python(py, &msg_value, None)
                .cast_into()
                .unwrap();
            Ok(result)
        }

        fn decode_message_to_json(&self, message_name: &str, data: Vec<u8>) -> PyResult<String> {
            let msg = self.try_decode_message_by_name(message_name, data)?;

            let serialize_options = SerializeOptions::new()
                .use_enum_numbers(!self.options.enums_as_strings)
                .skip_default_fields(!self.options.render_defaults)
                .use_proto_field_name(!self.options.use_json_names);
            let json = msg
                .serialize_with_options(serde_json::value::Serializer, &serialize_options)
                .unwrap();
            Ok(serde_json::to_string(&json).unwrap())
        }
    }

    impl MessageReader {
        fn try_decode_message_by_name(
            &self,
            message_name: &str,
            data: Vec<u8>,
        ) -> PyResult<DynamicMessage> {
            let descriptor = self
                .descriptor_pool
                .get_message_by_name(message_name)
                .ok_or(PyTypeError::new_err(format!(
                    "Message '{}' not found.",
                    message_name
                )))?;
            DynamicMessage::decode(descriptor, Bytes::from(data)).map_err(decode_error_to_py)
        }

        fn proto_value_to_python<'py>(
            &self,
            py: Python<'py>,
            value: &Value,
            field_descriptor: Option<&FieldDescriptor>,
        ) -> Bound<'py, PyAny> {
            match value {
                Value::Bool(b) => PyBool::new(py, b.clone()).to_owned().into_any(),
                Value::I32(i) => PyInt::new(py, i).into_any(),
                Value::I64(i) => PyInt::new(py, i).into_any(),
                Value::U32(i) => PyInt::new(py, i).into_any(),
                Value::U64(i) => PyInt::new(py, i).into_any(),
                Value::F32(f) => PyFloat::new(py, f.clone().into()).into_any(),
                Value::F64(d) => PyFloat::new(py, d.clone().into()).into_any(),
                Value::String(s) => PyString::new(py, s).into_any(),
                Value::Bytes(bytes) => PyBytes::new(py, bytes).into_any(),
                Value::EnumNumber(e) => {
                    if self.options.enums_as_strings {
                        let enum_descriptor = field_descriptor
                            .unwrap()
                            .kind()
                            .as_enum()
                            .unwrap()
                            .get_value(e.clone())
                            .unwrap();
                        PyString::new(py, enum_descriptor.name()).into_any()
                    } else {
                        PyInt::new(py, e).into_any()
                    }
                }
                Value::List(values) => PyList::new(
                    py,
                    values
                        .iter()
                        .map(|v| self.proto_value_to_python(py, v, None)),
                )
                .unwrap()
                .into_any(),
                Value::Map(hash_map) => {
                    let value_descriptor = field_descriptor
                        .unwrap()
                        .kind()
                        .as_message()
                        .unwrap()
                        .map_entry_value_field();
                    let msg_dict = PyDict::new(py);

                    for (key, val) in hash_map.iter() {
                        let py_value = self.proto_value_to_python(py, val, Some(&value_descriptor));
                        let py_key = match key {
                            prost_reflect::MapKey::Bool(b) => {
                                PyBool::new(py, b.clone()).to_owned().into_any()
                            }
                            prost_reflect::MapKey::I32(i) => PyInt::new(py, i).into_any(),
                            prost_reflect::MapKey::I64(i) => PyInt::new(py, i).into_any(),
                            prost_reflect::MapKey::U32(i) => PyInt::new(py, i).into_any(),
                            prost_reflect::MapKey::U64(i) => PyInt::new(py, i).into_any(),
                            prost_reflect::MapKey::String(s) => PyString::new(py, s).into_any(),
                        };
                        msg_dict.set_item(py_key, py_value).unwrap();
                    }
                    msg_dict.into_any()
                }
                Value::Message(message) => {
                    let msg_dict = PyDict::new(py);
                    for field in message.descriptor().fields() {
                        let has_field = message.has_field(&field);
                        if self.options.render_defaults || has_field {
                            let field_name = match self.options.use_json_names {
                                true => field.json_name(),
                                false => field.name(),
                            };
                            let field_value = match field.kind() {
                                prost_reflect::Kind::Message(_) if !has_field => {
                                    py.None().into_any().into_bound(py)
                                }
                                _ => self.proto_value_to_python(
                                    py,
                                    &message.get_field(&field),
                                    Some(&field),
                                ),
                            };
                            msg_dict.set_item(field_name, field_value).unwrap();
                        }
                    }
                    msg_dict.into_any()
                }
            }
        }
    }
}
