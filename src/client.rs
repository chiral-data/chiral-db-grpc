use tonic; 

pub mod chiral_db_proto {
    tonic::include_proto!("chiraldb");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = chiral_db_proto::chiral_db_client::ChiralDbClient::connect("http://[::1]:10000").await?;    

    println!("\n*** Query Similarity ***");
    let response = client.query_similarity(
        tonic::Request::new(chiral_db_proto::RequestSimilarity { 
            mol: Some(chiral_db_proto::Molecule { smiles: String::from("Cc1cc(NC(=O)c2cc(Cl)cc(Cl)c2O)ccc1Sc1nc2ccccc2s1") }),
            cutoff: 0.2
          })
    ).await?;
    println!("RESPONSE = {:?}", response);

    Ok(())
}