use serde::{Deserialize, Serialize};
use base64::{engine::general_purpose, Engine as _};
use serde_json;
use zenth_crypto_service::hashs::base64_vecdecode;

// FIRST REQUEST ALWAYS
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegisterAllRequest {
    pub allrequest: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FirstMethodRequest{
    pub method: String,
    pub nextrequest: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SecondIdentRequest{
    pub identifiant: String,
    pub nextrequest: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThirdSaltHashRequest{
    pub salthash: String,
    pub nextrequest: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegisterChainDecoded {
    pub method: String,
    pub identifiant: String,
    pub salthash: String,
    pub saltsecure: String,
    pub password: String,
}

pub fn deformater(register: RegisterAllRequest) -> RegisterChainDecoded {
    // Étape 0 : décoder la première enveloppe
    let zero: FirstMethodRequest = decode_from_base64(&register.allrequest);

    // Étape 1 : décoder la seconde enveloppe
    let first: SecondIdentRequest = decode_from_base64(&zero.nextrequest);

    // Étape 2 : décoder la troisième enveloppe
    let second: ThirdSaltHashRequest = decode_from_base64(&first.nextrequest);

    // Étape 3 : décoder la quatrième enveloppe
    let third: FourthSaltSecureRequest = decode_from_base64(&second.nextrequest);

    // Construire la structure complète
    RegisterChainDecoded {
        method: zero.method,
        identifiant: first.identifiant,
        salthash: second.salthash,
        saltsecure: third.saltsecure,
        password: third.password,
    }
}



pub fn deformater_first(register: RegisterAllRequest) -> (String, String) {
    let zero: FirstMethodRequest = decode_from_base64(&register.allrequest);
    (zero.method, zero.nextrequest)
}


pub fn encode_fourth_layer(
    saltsecure: String,
    password: String,
    encode_fn: impl Fn(&[u8]) -> Result<Vec<u8>, String>,
) -> Result<String, String> {
    let data = FourthSaltSecureRequest { saltsecure, password };
    let json = serde_json::to_vec(&data).map_err(|e| format!("Erreur JSON : {}", e))?;
    let encodeed = encode_fn(&json)?;
    Ok(base64::engine::general_purpose::STANDARD.encode(&encodeed))
}


pub fn encode_third_layer(
    salthash: String,
    next_encodeed: String,
    encode_fn: impl Fn(&[u8]) -> Result<Vec<u8>, String>,
) -> Result<String, String> {
    let data = ThirdSaltHashRequest {
        salthash,
        nextrequest: next_encodeed,
    };
    let json = serde_json::to_vec(&data).map_err(|e| format!("Erreur JSON : {}", e))?;
    let encodeed = encode_fn(&json)?;
    Ok(base64::engine::general_purpose::STANDARD.encode(&encodeed))
}


pub fn encode_second_layer(
    identifiant: String,
    next_encodeed: String,
    encode_fn: impl Fn(&[u8]) -> Result<Vec<u8>, String>,
) -> Result<String, String> {
    let data = SecondIdentRequest {
        identifiant,
        nextrequest: next_encodeed,
    };
    let json = serde_json::to_vec(&data).map_err(|e| format!("Erreur JSON : {}", e))?;
    let encodeed = encode_fn(&json)?;
    Ok(base64::engine::general_purpose::STANDARD.encode(&encodeed))
}

pub fn encode_first_layer(
    method: String,
    next_encodeed: String,
    encode_fn: impl Fn(&[u8]) -> Result<Vec<u8>, String>,
) -> Result<RegisterAllRequest, String> {
    let data = FirstMethodRequest {
        method,
        nextrequest: next_encodeed,
    };
    let json = serde_json::to_vec(&data).map_err(|e| format!("Erreur JSON : {}", e))?;
    let encodeed = encode_fn(&json)?;
    let encoded = base64::engine::general_purpose::STANDARD.encode(&encodeed);
    Ok(RegisterAllRequest { allrequest: encoded })
}



pub fn decode_first_layer(
    encoded: &str,
    decode_fn: impl Fn(&[u8]) -> Result<Vec<u8>, String>,
) -> Result<FirstMethodRequest, String> {
    let encodeed_bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .map_err(|e| format!("Base64 error: {}", e))?;
    let decodeed = decode_fn(&encodeed_bytes)?;
    serde_json::from_slice(&decodeed).map_err(|e| format!("JSON error: {}", e))
}


pub fn decode_second_layer(
    encoded: &str,
    decode_fn: impl Fn(&[u8]) -> Result<Vec<u8>, String>,
) -> Result<SecondIdentRequest, String> {
    let encodeed_bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .map_err(|e| format!("Base64 error: {}", e))?;
    let decodeed = decode_fn(&encodeed_bytes)?;
    serde_json::from_slice(&decodeed).map_err(|e| format!("JSON error: {}", e))
}


pub fn decode_third_layer(
    encoded: &str,
    decode_fn: impl Fn(&[u8]) -> Result<Vec<u8>, String>,
) -> Result<ThirdSaltHashRequest, String> {
    let encodeed_bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .map_err(|e| format!("Base64 error: {}", e))?;
    let decodeed = decode_fn(&encodeed_bytes)?;
    serde_json::from_slice(&decodeed).map_err(|e| format!("JSON error: {}", e))
}


pub fn decode_fourth_layer(
    encoded: &str,
    decode_fn: impl Fn(&[u8]) -> Result<Vec<u8>, String>,
) -> Result<FourthSaltSecureRequest, String> {
    let encodeed_bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .map_err(|e| format!("Base64 error: {}", e))?;
    let decodeed = decode_fn(&encodeed_bytes)?;
    serde_json::from_slice(&decodeed).map_err(|e| format!("JSON error: {}", e))
}


pub fn extraire_method_et_next(payload: String) -> Result<(String, String), String> {
    let register: RegisterAllRequest = serde_json::from_str(&payload)
        .map_err(|e| format!("Erreur JSON initial : {}", e))?;
    let decoded_bytes = base64_vecdecode(&register.allrequest)
        .map_err(|e| format!("Erreur de décodage base64 : {}", e))?;
    let first: FirstMethodRequest = serde_json::from_slice(&decoded_bytes)
        .map_err(|e| format!("Erreur JSON niveau 1 : {}", e))?;

    Ok((first.method, first.nextrequest))
}