mod protos {
    tonic::include_proto!("mod");
}
pub use protos::*;
pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("kvproto");
