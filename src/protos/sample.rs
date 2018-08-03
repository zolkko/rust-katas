// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Test {
    // message fields
    pub id: i32,
    pub name: ::std::string::String,
    pub typ: Test_VarType,
    // message oneof groups
    TestOneof: ::std::option::Option<Test_oneof_TestOneof>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Test {}

#[derive(Clone,PartialEq)]
pub enum Test_oneof_TestOneof {
    yoyo(::std::string::String),
    sub_message(i32),
}

impl Test {
    pub fn new() -> Test {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Test {
        static mut instance: ::protobuf::lazy::Lazy<Test> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Test,
        };
        unsafe {
            instance.get(Test::new)
        }
    }

    // int32 id = 1;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i32) {
        self.id = v;
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &i32 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut i32 {
        &mut self.id
    }

    // string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // .Test.VarType typ = 3;

    pub fn clear_typ(&mut self) {
        self.typ = Test_VarType::CONT;
    }

    // Param is passed by value, moved
    pub fn set_typ(&mut self, v: Test_VarType) {
        self.typ = v;
    }

    pub fn get_typ(&self) -> Test_VarType {
        self.typ
    }

    fn get_typ_for_reflect(&self) -> &Test_VarType {
        &self.typ
    }

    fn mut_typ_for_reflect(&mut self) -> &mut Test_VarType {
        &mut self.typ
    }

    // string yoyo = 4;

    pub fn clear_yoyo(&mut self) {
        self.TestOneof = ::std::option::Option::None;
    }

    pub fn has_yoyo(&self) -> bool {
        match self.TestOneof {
            ::std::option::Option::Some(Test_oneof_TestOneof::yoyo(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_yoyo(&mut self, v: ::std::string::String) {
        self.TestOneof = ::std::option::Option::Some(Test_oneof_TestOneof::yoyo(v))
    }

    // Mutable pointer to the field.
    pub fn mut_yoyo(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(Test_oneof_TestOneof::yoyo(_)) = self.TestOneof {
        } else {
            self.TestOneof = ::std::option::Option::Some(Test_oneof_TestOneof::yoyo(::std::string::String::new()));
        }
        match self.TestOneof {
            ::std::option::Option::Some(Test_oneof_TestOneof::yoyo(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_yoyo(&mut self) -> ::std::string::String {
        if self.has_yoyo() {
            match self.TestOneof.take() {
                ::std::option::Option::Some(Test_oneof_TestOneof::yoyo(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_yoyo(&self) -> &str {
        match self.TestOneof {
            ::std::option::Option::Some(Test_oneof_TestOneof::yoyo(ref v)) => v,
            _ => "",
        }
    }

    // int32 sub_message = 5;

    pub fn clear_sub_message(&mut self) {
        self.TestOneof = ::std::option::Option::None;
    }

    pub fn has_sub_message(&self) -> bool {
        match self.TestOneof {
            ::std::option::Option::Some(Test_oneof_TestOneof::sub_message(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_sub_message(&mut self, v: i32) {
        self.TestOneof = ::std::option::Option::Some(Test_oneof_TestOneof::sub_message(v))
    }

    pub fn get_sub_message(&self) -> i32 {
        match self.TestOneof {
            ::std::option::Option::Some(Test_oneof_TestOneof::sub_message(v)) => v,
            _ => 0,
        }
    }
}

impl ::protobuf::Message for Test {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.typ = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.TestOneof = ::std::option::Option::Some(Test_oneof_TestOneof::yoyo(is.read_string()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.TestOneof = ::std::option::Option::Some(Test_oneof_TestOneof::sub_message(is.read_int32()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.id, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        }
        if self.typ != Test_VarType::CONT {
            my_size += ::protobuf::rt::enum_size(3, self.typ);
        }
        if let ::std::option::Option::Some(ref v) = self.TestOneof {
            match v {
                &Test_oneof_TestOneof::yoyo(ref v) => {
                    my_size += ::protobuf::rt::string_size(4, &v);
                },
                &Test_oneof_TestOneof::sub_message(v) => {
                    my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.id != 0 {
            os.write_int32(1, self.id)?;
        }
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        if self.typ != Test_VarType::CONT {
            os.write_enum(3, self.typ.value())?;
        }
        if let ::std::option::Option::Some(ref v) = self.TestOneof {
            match v {
                &Test_oneof_TestOneof::yoyo(ref v) => {
                    os.write_string(4, v)?;
                },
                &Test_oneof_TestOneof::sub_message(v) => {
                    os.write_int32(5, v)?;
                },
            };
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Test {
    fn new() -> Test {
        Test::new()
    }

    fn descriptor_static(_: ::std::option::Option<Test>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "id",
                    Test::get_id_for_reflect,
                    Test::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Test::get_name_for_reflect,
                    Test::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Test_VarType>>(
                    "typ",
                    Test::get_typ_for_reflect,
                    Test::mut_typ_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "yoyo",
                    Test::has_yoyo,
                    Test::get_yoyo,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor::<_>(
                    "sub_message",
                    Test::has_sub_message,
                    Test::get_sub_message,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Test>(
                    "Test",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Test {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_name();
        self.clear_typ();
        self.clear_yoyo();
        self.clear_sub_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Test {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Test {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Test_VarType {
    CONT = 0,
    DISC = 1,
}

impl ::protobuf::ProtobufEnum for Test_VarType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Test_VarType> {
        match value {
            0 => ::std::option::Option::Some(Test_VarType::CONT),
            1 => ::std::option::Option::Some(Test_VarType::DISC),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Test_VarType] = &[
            Test_VarType::CONT,
            Test_VarType::DISC,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Test_VarType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Test_VarType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Test_VarType {
}

impl ::std::default::Default for Test_VarType {
    fn default() -> Self {
        Test_VarType::CONT
    }
}

impl ::protobuf::reflect::ProtobufValue for Test_VarType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0csample.proto\"\xb0\x01\n\x04Test\x12\x0e\n\x02id\x18\x01\x20\x01(\
    \x05R\x02id\x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04name\x12\x1f\n\x03t\
    yp\x18\x03\x20\x01(\x0e2\r.Test.VarTypeR\x03typ\x12\x14\n\x04yoyo\x18\
    \x04\x20\x01(\tH\0R\x04yoyo\x12!\n\x0bsub_message\x18\x05\x20\x01(\x05H\
    \0R\nsubMessage\"\x1d\n\x07VarType\x12\x08\n\x04CONT\x10\0\x12\x08\n\x04\
    DISC\x10\x01B\x0b\n\tTestOneofb\x06proto3\
";

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
