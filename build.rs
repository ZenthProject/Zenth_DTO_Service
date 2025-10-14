fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .build_server(true)   // Génère le serveur
        .build_client(true)   // Génère le client
        .compile_protos(&["proto/register.proto"], &["proto"])?;
    Ok(())
}
