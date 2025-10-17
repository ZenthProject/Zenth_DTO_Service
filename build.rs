fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(
            &[
                "proto/requests/register/registertest.proto",
                "proto/requests/register/register.proto",
                "proto/requests/login/login.proto",
                "proto/requests/request.proto",
                "proto/responses/response.proto",
            ],
            &["proto"]
        )?;
    Ok(())
}
