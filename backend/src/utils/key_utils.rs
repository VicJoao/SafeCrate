use::sodiumoxide::hex;

pub fn create_keypair() -> (String, String) {
    let (public_key, private_key) = sodiumoxide::crypto::box_::gen_keypair();
    let public_key = public_key.as_ref().to_vec();
    let private_key = private_key.as_ref().to_vec();
    (hex::encode(public_key), hex::encode(private_key))
}