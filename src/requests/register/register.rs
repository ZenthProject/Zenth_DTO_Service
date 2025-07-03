use serde::{Deserialize, Serialize};
use base64::{engine::general_purpose, Engine as _};
use serde_json;
use zenth_crypto_service::hashs::{
    base64_vecdecode,
    base64decode
};

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
    let third = FourthSaltSecureRequest {
        saltsecure,
        password,
    };
    let third_b64 = encode_to_base64(&third);
    let second = ThirdSaltHashRequest {
        salthash,
        nextrequest: third_b64,
    };
    let second_b64 = encode_to_base64(&second);
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
    let zero: FirstMethodRequest = decode_from_base64(&register.allrequest);
    let first: SecondIdentRequest = decode_from_base64(&zero.nextrequest);
    let second: ThirdSaltHashRequest = decode_from_base64(&first.nextrequest);
    let third: FourthSaltSecureRequest = decode_from_base64(&second.nextrequest);
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

pub fn extraire_method_et_next(payload: String) -> Result<(String, String), String> {
    let register: RegisterAllRequest = serde_json::from_str(&payload)
        .map_err(|e| format!("Erreur JSON initial : {}", e))?;
    let decoded_bytes = base64_vecdecode(&register.allrequest)
        .map_err(|e| format!("Erreur de décodage base64 : {}", e))?;
    let first: FirstMethodRequest = serde_json::from_slice(&decoded_bytes)
        .map_err(|e| format!("Erreur JSON niveau 1 : {}", e))?;

    Ok((first.method, first.nextrequest))
}

pub fn extraire_identifiant_et_next(encoded: &str) -> Result<(String, String), String> {
    let decoded_bytes = base64_vecdecode(encoded)
        .map_err(|e| format!("Erreur de décodage base64 : {}", e))?;
    let second: SecondIdentRequest = serde_json::from_slice(&decoded_bytes)
        .map_err(|e| format!("Erreur JSON dans SecondIdentRequest : {}", e))?;
    Ok((second.identifiant, second.nextrequest))
}


pub fn extraire_salt_et_next(encoded: &str) -> Result<(String, String), String> {
    let decoded_bytes = base64_vecdecode(encoded)
        .map_err(|e| format!("Erreur de décodage base64 : {}", e))?;
    let data_request: ThirdSaltHashRequest = serde_json::from_slice(&decoded_bytes)
        .map_err(|e| format!("Erreur JSON dans DataRequest : {}", e))?;
    Ok((data_request.salthash, data_request.nextrequest))
}


pub fn extraire_salt_et_password(encoded: &str) -> Result<(String, String), String> {
    let decoded_bytes = base64_vecdecode(encoded)
        .map_err(|e| format!("Erreur de décodage base64 : {}", e))?;
    let data_request: FourthSaltSecureRequest = serde_json::from_slice(&decoded_bytes)
        .map_err(|e| format!("Erreur JSON dans SaltPasswordRequest : {}", e))?;
    Ok((data_request.saltsecure, data_request.password))
}
