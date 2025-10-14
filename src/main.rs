mod auth {
    tonic::include_proto!("auth");
}

use auth::RegisterRequest;

fn main() {
    let req = RegisterRequest {
        client_nonce: vec![
            0x01, 
            0x02, 
            0x03
            ],
        timestamp: 1690000000000,
        envelope: None,
    };

    println!(
        "RegisterRequest: {:?}", 
        req
    );
}
