/// Represents a complete VORTEX model
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VortexModel {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub version: i32,
    #[prost(message, repeated, tag="3")]
    pub layers: ::prost::alloc::vec::Vec<VortexLayer>,
}
/// Represents a single layer in the VORTEX model
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VortexLayer {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(int32, tag="3")]
    pub input_dim: i32,
    #[prost(int32, tag="4")]
    pub output_dim: i32,
    #[prost(float, repeated, tag="5")]
    pub weights: ::prost::alloc::vec::Vec<f32>,
    #[prost(float, repeated, tag="6")]
    pub biases: ::prost::alloc::vec::Vec<f32>,
}
