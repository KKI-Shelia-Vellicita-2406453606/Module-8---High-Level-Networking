fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .compile(
            &["proto/services.proto"], // Path to the proto file
            &["proto"],                // Directory where it is located
        )?;
    Ok(())
}