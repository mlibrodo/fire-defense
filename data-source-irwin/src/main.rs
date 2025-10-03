use data_source_irwin::{environments, IncidentQueryBuilder, IrwinConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("IRWIN Client Demo");

    // Example configuration (you would need real credentials)
    let config = IrwinConfig::new(
        environments::TEST.to_string(),
        "your_username".to_string(),
        "your_password".to_string(),
        "your_referer".to_string(),
    );

    println!("Configuration created successfully");

    // Example query
    let query = IncidentQueryBuilder::new()
        .where_clause("IsValid=1 AND IncidentTypeKind='FI'")
        .out_fields("IrwinID,IncidentName,UniqueFireIdentifier")
        .return_geometry("true")
        .include_resources(true)
        .include_relationships(true)
        .build();

    println!("Query built: {:?}", query);

    // Note: This would require real credentials to actually work
    println!("Client is ready for use with valid credentials");

    Ok(())
}
