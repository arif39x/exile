fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .compile(
            &["../shared_protocol/registry.proto"],
            &["../shared_protocol"],
        )?;
    Ok(())
}
