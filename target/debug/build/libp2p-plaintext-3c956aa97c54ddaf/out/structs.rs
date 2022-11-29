#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Exchange {
    #[prost(bytes="vec", optional, tag="1")]
    pub id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="2")]
    pub pubkey: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
