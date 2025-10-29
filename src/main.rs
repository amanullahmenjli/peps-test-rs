use serde::{Deserialize, Serialize};
use supabase::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the client
    let client = Client::new(
        "https://ozlzgyaemrjbwgtxwrzt.supabase.co",
        "sb_secret_t0jGFNCXM9VlVn01iOEwmw_q57rRVL0",
    )?;

    // Authenticate user
    let auth_response = client
        .auth()
        .sign_in_with_email_and_password("menjliamanullah@egg.tn", "samir123")
        .await?;

    println!("User signed in: {:?}", auth_response.user);

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct Activitiy {
        id: u32,
        name: String,
    }

    // Query database
    let posts = client
        .database()
        .from("activities")
        .select("*")
        .execute::<Activitiy>()
        .await?;

    println!();
    println!("Posts: {:?}", posts);

    Ok(())
}
