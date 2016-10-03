// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct WeatherRequest {
    // message fields
    location: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WeatherRequest {}

impl WeatherRequest {
    pub fn new() -> WeatherRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WeatherRequest {
        static mut instance: ::protobuf::lazy::Lazy<WeatherRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WeatherRequest,
        };
        unsafe {
            instance.get(|| {
                WeatherRequest {
                    location: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string location = 1;

    pub fn clear_location(&mut self) {
        self.location.clear();
    }

    pub fn has_location(&self) -> bool {
        self.location.is_some()
    }

    // Param is passed by value, moved
    pub fn set_location(&mut self, v: ::std::string::String) {
        self.location = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_location(&mut self) -> &mut ::std::string::String {
        if self.location.is_none() {
            self.location.set_default();
        };
        self.location.as_mut().unwrap()
    }

    // Take field
    pub fn take_location(&mut self) -> ::std::string::String {
        self.location.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_location(&self) -> &str {
        match self.location.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for WeatherRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.location));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.location {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.location.as_ref() {
            try!(os.write_string(1, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<WeatherRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WeatherRequest {
    fn new() -> WeatherRequest {
        WeatherRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<WeatherRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "location",
                    WeatherRequest::has_location,
                    WeatherRequest::get_location,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WeatherRequest>(
                    "WeatherRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WeatherRequest {
    fn clear(&mut self) {
        self.clear_location();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for WeatherRequest {
    fn eq(&self, other: &WeatherRequest) -> bool {
        self.location == other.location &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for WeatherRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct WeatherResponse {
    // message fields
    temperature: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WeatherResponse {}

impl WeatherResponse {
    pub fn new() -> WeatherResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WeatherResponse {
        static mut instance: ::protobuf::lazy::Lazy<WeatherResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WeatherResponse,
        };
        unsafe {
            instance.get(|| {
                WeatherResponse {
                    temperature: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 temperature = 1;

    pub fn clear_temperature(&mut self) {
        self.temperature = ::std::option::Option::None;
    }

    pub fn has_temperature(&self) -> bool {
        self.temperature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_temperature(&mut self, v: i32) {
        self.temperature = ::std::option::Option::Some(v);
    }

    pub fn get_temperature(&self) -> i32 {
        self.temperature.unwrap_or(0)
    }
}

impl ::protobuf::Message for WeatherResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.temperature = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.temperature {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.temperature {
            try!(os.write_int32(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<WeatherResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WeatherResponse {
    fn new() -> WeatherResponse {
        WeatherResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<WeatherResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "temperature",
                    WeatherResponse::has_temperature,
                    WeatherResponse::get_temperature,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WeatherResponse>(
                    "WeatherResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WeatherResponse {
    fn clear(&mut self) {
        self.clear_temperature();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for WeatherResponse {
    fn eq(&self, other: &WeatherResponse) -> bool {
        self.temperature == other.temperature &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for WeatherResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0b, 0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x05, 0x68,
    0x65, 0x6c, 0x6c, 0x6f, 0x22, 0x22, 0x0a, 0x0e, 0x57, 0x65, 0x61, 0x74, 0x68, 0x65, 0x72, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x10, 0x0a, 0x08, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x22, 0x26, 0x0a, 0x0f, 0x57, 0x65, 0x61, 0x74,
    0x68, 0x65, 0x72, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x13, 0x0a, 0x0b, 0x74,
    0x65, 0x6d, 0x70, 0x65, 0x72, 0x61, 0x74, 0x75, 0x72, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05,
    0x32, 0x48, 0x0a, 0x07, 0x57, 0x65, 0x61, 0x74, 0x68, 0x65, 0x72, 0x12, 0x3d, 0x0a, 0x0a, 0x67,
    0x65, 0x74, 0x57, 0x65, 0x61, 0x74, 0x68, 0x65, 0x72, 0x12, 0x15, 0x2e, 0x68, 0x65, 0x6c, 0x6c,
    0x6f, 0x2e, 0x57, 0x65, 0x61, 0x74, 0x68, 0x65, 0x72, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x1a, 0x16, 0x2e, 0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x2e, 0x57, 0x65, 0x61, 0x74, 0x68, 0x65, 0x72,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x00, 0x4a, 0x9b, 0x02, 0x0a, 0x06, 0x12,
    0x04, 0x00, 0x00, 0x0e, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08, 0x0d, 0x0a,
    0x0a, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x04, 0x00, 0x06, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x06,
    0x00, 0x01, 0x12, 0x03, 0x04, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x05, 0x02, 0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05,
    0x06, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x05, 0x12, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x05, 0x2b, 0x3a, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x08, 0x00, 0x0a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x08, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x09, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x09, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x09, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x12, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x01, 0x12, 0x04, 0x0c, 0x00, 0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03,
    0x0c, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x02, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0d, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0d, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x11, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x0d, 0x1f, 0x20,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
