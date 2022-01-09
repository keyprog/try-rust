use mongodb::{bson::doc, sync::Client};

fn main() -> mongodb::error::Result<()> {
    let cn = "mongodb://192.168.0.101:32211";
    let mc = Client::with_uri_str(cn)?;
    mc.database("test")
    .run_command(doc! {"ping": 1}, None)?;
    println!("Connected successfully.");
    
    // List the names of the databases in that cluster
    for db_name in mc.list_database_names(None, None)? {
        println!("{}", db_name);
    }

    Ok(())
}
