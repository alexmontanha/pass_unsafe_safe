use bcrypt::{hash, verify};

fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, bcrypt::DEFAULT_COST)
}

fn verify_password(password: &str, hashed_password: &str) -> bool {
    match verify(password, hashed_password) {
        Ok(result) => result,
        Err(_) => false,
    }
}

fn main() {
    let password = "senha123";

    // Hash da senha
    let hashed_password = match hash_password(password) {
        Ok(hashed) => hashed,
        Err(err) => {
            eprintln!("Erro ao gerar hash: {}", err);
            return;
        }
    };

    println!("Senha: {}", password);
    println!("Hash: {}", hashed_password);

    // Verificar senha
    let is_valid = verify_password(password, &hashed_password);
    println!("Senha válida: {}", is_valid);
}
