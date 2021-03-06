use std::prelude::v1::*;

use std::mem;

use super::value::ProtobufValue;

use singular::*;

pub trait ReflectOptional: 'static {
    fn to_option(&self) -> Option<&ProtobufValue>;

    fn set_value(&mut self, value: &ProtobufValue);
}

impl<V : ProtobufValue + Clone + 'static> ReflectOptional for Option<V> {
    fn to_option(&self) -> Option<&ProtobufValue> {
        self.as_ref().map(|v| v as &ProtobufValue)
    }

    fn set_value(&mut self, value: &ProtobufValue) {
        match value.as_any().downcast_ref::<V>() {
            Some(v) => mem::replace(self, Some(v.clone())),
            None => panic!(),
        };
    }
}

impl<V : ProtobufValue + Clone + 'static> ReflectOptional for SingularField<V> {
    fn to_option(&self) -> Option<&ProtobufValue> {
        self.as_ref().map(|v| v as &ProtobufValue)
    }

    fn set_value(&mut self, value: &ProtobufValue) {
        match value.as_any().downcast_ref::<V>() {
            Some(v) => mem::replace(self, SingularField::some(v.clone())),
            None => panic!(),
        };
    }
}

impl<V : ProtobufValue + Clone + 'static> ReflectOptional for SingularPtrField<V> {
    fn to_option(&self) -> Option<&ProtobufValue> {
        self.as_ref().map(|v| v as &ProtobufValue)
    }

    fn set_value(&mut self, value: &ProtobufValue) {
        match value.as_any().downcast_ref::<V>() {
            Some(v) => mem::replace(self, SingularPtrField::some(v.clone())),
            None => panic!(),
        };
    }
}
