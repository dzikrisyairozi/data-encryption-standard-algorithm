use rocket::{post, http::Status};
use rocket::serde::json::Json;
use des_cipher::des_encrypt;

// Define a struct for the incoming data
#[derive(Serialize, Deserialize, Debug)]
pub struct InputData {
    plaintext: String,
    key: String,
}

// Define a struct for the response data
#[derive(Serialize, Deserialize, Debug)]
pub struct EncryptedData {
    encrypted_text: String,
}

#[options("/encrypt")]
pub fn options_encrypt() -> &'static str {
    ""
}

#[post("/", data = "<input_data>")]
pub fn encrypt(input_data: Json<InputData>) -> Result<Json<EncryptedData>, Status> {
    // Convert the input strings into u64
    let plaintext_num = u64::from_str_radix(&input_data.plaintext, 16).map_err(|_| Status::BadRequest)?;
    let key_num = u64::from_str_radix(&input_data.key, 16).map_err(|_| Status::BadRequest)?;

    // Encrypt the data using DES
    let encrypted_num = des_encrypt(plaintext_num, key_num);

    // Convert the encrypted number into a string
    let encrypted_str = format!("{:016x}", encrypted_num);

    Ok(Json(EncryptedData {
        encrypted_text: encrypted_str,
    }))
}
