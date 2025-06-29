use zenth_dto_serice::requests::register::register::{
    FirstMethodRequest,
    SecondIdentRequest,
    ThirdSaltHashRequest,
    FourthSaltSecureRequest,
    decode_from_base64,
    format_register_chain,
};



#[test]
fn test_register_chain() {
    let method = "REGISTER".to_string();
    let identifiant = "JudyJane".to_string();
    let salthash = "saltyhash123".to_string();
    let saltsecure = "securesalt456".to_string();
    let password = "superSecretPwd!".to_string();

    let register_request = format_register_chain(
        method.clone(),
        identifiant.clone(),
        salthash.clone(),
        saltsecure.clone(),
        password.clone(),
    );


    let first_request: FirstMethodRequest = decode_from_base64(&register_request.allrequest);
    let second_request: SecondIdentRequest = decode_from_base64(&first_request.nextrequest);
    let third_request: ThirdSaltHashRequest = decode_from_base64(&second_request.nextrequest);
    let fourth_request: FourthSaltSecureRequest = decode_from_base64(&third_request.nextrequest);

    assert_eq!(first_request.method, method); // ou la valeur que tu attends
    assert_eq!(second_request.identifiant, identifiant);
    assert_eq!(third_request.salthash, salthash);
    assert_eq!(fourth_request.saltsecure, saltsecure);
    assert_eq!(fourth_request.password, password);

}
