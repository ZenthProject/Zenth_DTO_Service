use serde::{Deserialize, Serialize};
use base64::{engine::general_purpose, Engine as _};
use serde_json;


// FIRST REQUEST ALWAYS
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterAllRequest {
    pub allrequest: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FirstMethodRequest{
    pub method: String,
    pub nextrequest: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecondIdentRequest{
    pub identifiant: String,
    pub nextrequest: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThirdSaltHashRequest{
    pub salthash: String,
    pub nextrequest: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FourthSaltSecureRequest{
    pub saltsecure: String,
    pub password: String
}



pub fn encode_to_base64<T: Serialize>(data: &T) -> String {
    let json = serde_json::to_string(data).expect("Erreur de sérialisation JSON");
    general_purpose::STANDARD.encode(json)
}


pub fn decode_from_base64<T: for<'de> Deserialize<'de>>(data: &str) -> T {
    let json = general_purpose::STANDARD
        .decode(data)
        .expect("Erreur de décodage Base64");
    serde_json::from_slice(&json).expect("Erreur de désérialisation JSON")
}


pub fn format_register_chain(
    method: String,  
    identifiant: String,
    salthash: String,
    saltsecure: String,
    password: String,
) -> RegisterAllRequest {
    // Étape 3 : construction de la dernière requête
    let third = FourthSaltSecureRequest {
        saltsecure,
        password,
    };
    let third_b64 = encode_to_base64(&third);

    // Étape 2 : construction de la deuxième requête
    let second = ThirdSaltHashRequest {
        salthash,
        nextrequest: third_b64,
    };
    let second_b64 = encode_to_base64(&second);

    // Étape 1 : construction de la première requête
    let first = SecondIdentRequest {
        identifiant,
        nextrequest: second_b64,
    };
    let first_b64 = encode_to_base64(&first);

    let zero = FirstMethodRequest {
        method,
        nextrequest: first_b64,
    };
    let zero_b64 = encode_to_base64(&zero);


    // Enveloppe globale
    RegisterAllRequest {
        allrequest: zero_b64,
    }
}
