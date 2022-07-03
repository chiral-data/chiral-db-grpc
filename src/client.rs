use tonic; 

pub mod chiral_db_proto {
    tonic::include_proto!("chiraldb");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = chiral_db_proto::chiral_db_client::ChiralDbClient::connect("http://[::1]:10000").await?;    
    // let mut client = chiral_db_proto::chiral_db_client::ChiralDbClient::connect("http://0.0.0.0:10000").await?;    

    println!("\n*** Get DB Description ***");
    let response_1 = client.get_description(
        tonic::Request::new(chiral_db_proto::RequestDescription {})
    ).await?;
    println!("RESPONSE = {:?}", response_1);
    println!("{}", response_1.into_inner().desc);

    println!("\n*** Query Similarity ***");
    let response_2 = client.query_similarity(
        tonic::Request::new(chiral_db_proto::RequestSimilarity {
            doc_name: String::from("ChEMBL"),
            mol: Some(chiral_db_proto::Molecule { smiles: String::from("Cc1cc(NC(=O)c2cc(Cl)cc(Cl)c2O)ccc1Sc1nc2ccccc2s1") }),
            cutoff: 0.2
          })
    ).await?;
    println!("RESPONSE = {:?}", response_2);

    Ok(())
}