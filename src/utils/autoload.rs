use crate::utils::crypto::jwt_create;

pub fn init() {
    // load env
    dotenvy::dotenv().ok();


    // testing JWT
    let token = jwt_create(
        "1".to_string(),
        "johndoe".to_string(),
        "John Doe".to_string(),
        vec!["admin".to_string(), "editor".to_string()],
    )
    .ok();

    println!("token {:?}", token);
}
