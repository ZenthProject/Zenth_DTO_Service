fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .build_server(true)   
        .build_client(true)
        .compile_protos(
            &[
                "proto/registertest.proto",
                "proto/register.proto",
                "proto/request.proto",
                "proto/login.proto"
            ], 
            &["proto"]
        )?;
    Ok(())
}
