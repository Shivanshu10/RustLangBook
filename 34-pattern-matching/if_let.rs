fn main() {
    let auth_status: Option<&str> = None;
    let is_admin = false;
    let grp_id: Result<u8, _> = "34".parse();

    // auth status match to status
    if let Some(status) = auth_status {
        println!("Auth Status {}", status);
    } else if is_admin {
        println!("Auth status: admin");
    } else if let Ok(grp_id) = grp_id {
        if grp_id > 30 {
            println!("Auth Status: Privil");
        } else {
            println!("Auth Status: Basic");
        }
    } else {
        println!("Auth Status: Guest");
    }
}