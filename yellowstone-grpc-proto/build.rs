fn main() -> anyhow::Result<()> {
    const PROTOC_ENVAR: &str = "PROTOC";
    if std::env::var(PROTOC_ENVAR).is_err() {
        #[cfg(not(windows))]
        std::env::set_var(PROTOC_ENVAR, protobuf_src::protoc());
    }
    tonic_build::compile_protos("proto/geyser.proto")?;
    Ok(())
}
