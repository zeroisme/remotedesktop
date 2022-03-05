/// 客户端请求，基本为事件（键盘事件，鼠标事件）
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
    #[prost(oneof="operation::Event", tags="1")]
    pub event: ::core::option::Option<operation::Event>,
}
/// Nested message and enum types in `Operation`.
pub mod operation {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag="1")]
        Resize(super::Resize),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resize {
    #[prost(uint32, tag="1")]
    pub width: u32,
    #[prost(uint32, tag="2")]
    pub height: u32,
}
/// 服务端传输图片流
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Image {
    #[prost(uint32, tag="1")]
    pub width: u32,
    #[prost(uint32, tag="2")]
    pub height: u32,
    #[prost(enumeration="image::Type", tag="3")]
    pub typ: i32,
    #[prost(enumeration="image::Mode", tag="4")]
    pub mode: i32,
    #[prost(bytes="bytes", tag="5")]
    pub data: ::prost::bytes::Bytes,
}
/// Nested message and enum types in `Image`.
pub mod image {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// 正常图片
        Nomal = 0,
        /// 与前一个capture的图片异或的图片
        Exclusive = 1,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Mode {
        Rgb = 0,
        Rgba = 1,
    }
}
