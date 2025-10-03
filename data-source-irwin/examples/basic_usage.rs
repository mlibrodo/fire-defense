use data_source_irwin::{types::environments, IncidentQueryBuilder, IrwinClient, IrwinConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("IRWIN Client Example");
    println!("===================");

    // Example 1: Basic client creation
    println!("\n1. Creating client...");
    let config = IrwinConfig::new(
        environments::TEST.to_string(),
        "your_username".to_string(),
        "your_password".to_string(),
        "your_referer".to_string(),
    );

    let client = IrwinClient::new(config)?;
    println!("✓ Client created successfully");

    // Example 2: Building a query
    println!("\n2. Building incident query...");
    let query = IncidentQueryBuilder::new()
        .where_clause("IsValid=1 AND IncidentTypeKind='FI'")
        .out_fields("IrwinID,IncidentName,UniqueFireIdentifier,ModifiedOnDateTime")
        .return_geometry("true")
        .include_resources(true)
        .include_relationships(true)
        .include_last_sync_date_time(true)
        .build();

    println!("✓ Query built with parameters:");
    println!("  - WHERE: {:?}", query.where_clause);
    println!("  - Fields: {:?}", query.out_fields);
    println!("  - Resources: {}", query.include_resources);
    println!("  - Relationships: {}", query.include_relationships);

    // Example 3: Query by specific IDs
    println!("\n3. Building query for specific incidents...");
    let irwin_ids = vec!["12345".to_string(), "67890".to_string()];
    let id_query = IncidentQueryBuilder::new()
        .where_clause(&format!("IrwinID IN ({})", irwin_ids.join(",")))
        .out_fields("IrwinID,IncidentName,UniqueFireIdentifier")
        .return_geometry("false")
        .build();

    println!("✓ ID-based query built");

    // Example 4: Incremental sync query
    println!("\n4. Building incremental sync query...");
    let last_sync = IrwinClient::current_timestamp_ms() - (24 * 60 * 60 * 1000); // 24 hours ago
    let sync_query = IncidentQueryBuilder::new()
        .where_clause(&format!("ModifiedOnDateTime >= {}", last_sync))
        .out_fields("IrwinID,IncidentName,ModifiedOnDateTime")
        .include_last_sync_date_time(true)
        .build();

    println!("✓ Sync query built for changes since: {}", last_sync);

    // Example 5: Environment information
    println!("\n5. Available environments:");
    println!("  - TEST: {}", environments::TEST);
    println!("  - OAT:  {}", environments::OAT);
    println!("  - PROD: {}", environments::PROD);

    println!("\n✓ Example completed successfully!");
    println!("\nNote: This example shows the client setup and query building.");
    println!("      To make actual API calls, you need valid IRWIN credentials.");

    Ok(())
}
