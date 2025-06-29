
mod requests;

use axum::http::method;
use requests::register::{SecondIdentRequest, FirstMethodRequest, decode_from_base64, format_register_chain, ThirdSaltHashRequest, FourthSaltSecureRequest};
fn main() {
    // Données fictives
    let method = "REGISTER".to_string();
    let identifiant = "JudyJane".to_string();
    let salthash = "saltyhash123".to_string();
    let saltsecure = "securesalt456".to_string();
    let password = "superSecretPwd!".to_string();

    // Création de la chaîne de requêtes
    let register_request = format_register_chain(
        method.clone(),
        identifiant.clone(),
        salthash.clone(),
        saltsecure.clone(),
        password.clone(),
    );

    println!("Requête finale encodée : {:?}", register_request);

    let first_request: FirstMethodRequest = decode_from_base64(&register_request.allrequest);
    println!("Décodage niveau 1 : {:?}", first_request);

    // Décodage niveau 1
    let second_request: SecondIdentRequest = decode_from_base64(&first_request.nextrequest);
    println!("Décodage niveau 2 : {:?}", second_request);

    // Décodage niveau 2
    let third_request: ThirdSaltHashRequest = decode_from_base64(&second_request.nextrequest);
    println!("Décodage niveau 3 : {:?}", third_request);
    // Décodage niveau 3
    let fourth_request: FourthSaltSecureRequest = decode_from_base64(&third_request.nextrequest);
    println!("Décodage niveau 4 : {:?}", fourth_request);

    // Vérification des données
    assert_eq!(first_request.method, method);
    assert_eq!(second_request.identifiant, identifiant);
    assert_eq!(third_request.salthash, salthash);
    assert_eq!(fourth_request.saltsecure, saltsecure);
    assert_eq!(fourth_request.password, password);

    println!("✅ Test réussi : toutes les données correspondent !");
}