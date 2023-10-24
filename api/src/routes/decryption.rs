use rocket::serde::json::Json;
use rocket::http::Status;
use des_cipher::des_decrypt; 

// Define a struct for the incoming data
#[derive(Serialize, Deserialize, Debug)]
pub struct InputData {
    encrypted_text: String,
    key: String,
}

// Define a struct for the response data
#[derive(Serialize, Deserialize, Debug)]
pub struct DecryptedData {
    plaintext: String,
}

#[options("/decrypt")]
pub fn options_decrypt() -> &'static str {
    ""
}

#[post("/", data = "<input_data>")]
pub async fn decrypt(input_data: Json<InputData>) -> Result<Json<DecryptedData>, Status> {
    // Convert the input strings into u64
    let encrypted_num = u64::from_str_radix(&input_data.encrypted_text, 16).map_err(|_| Status::BadRequest)?;
    let key_num = u64::from_str_radix(&input_data.key, 16).map_err(|_| Status::BadRequest)?;

    // Decrypt the data using DES
    let plaintext_num = des_decrypt(encrypted_num, key_num);

    // Convert the decrypted number into a string
    let plaintext_str = format!("{:016x}", plaintext_num);

    Ok(Json(DecryptedData {
        plaintext: plaintext_str,
    }))
}
