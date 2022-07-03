use tonic;
use chiral_db;

pub mod chiral_db_proto {
    tonic::include_proto!("chiraldb");
}

#[derive(Debug)]
struct ChiralDbService {
    db: std::sync::Arc<chiral_db::ChiralDB>
}

#[tonic::async_trait]
impl chiral_db_proto::chiral_db_server::ChiralDb for ChiralDbService {
    async fn get_description(&self, _request: tonic::Request<chiral_db_proto::RequestDescription>) -> Result<tonic::Response<chiral_db_proto::ReplyDescription>, tonic::Status> {
        let desc = self.db.desc();
        Ok(tonic::Response::new(chiral_db_proto::ReplyDescription { desc }))

    }

    async fn query_similarity(&self, request: tonic::Request<chiral_db_proto::RequestSimilarity>) -> Result<tonic::Response<chiral_db_proto::ReplySimilarity>, tonic::Status>{
        let smiles = &request.get_ref().mol.as_ref().unwrap().smiles;
        let cutoff = request.get_ref().cutoff;
        let doc_name = &request.get_ref().doc_name;
        let results = self.db.query_similarity_for_smiles(doc_name, smiles, cutoff);
        Ok(tonic::Response::new(chiral_db_proto::ReplySimilarity { results }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chiral_db_svc = ChiralDbService { db: std::sync::Arc::new(chiral_db::ChiralDB::new()) };
    let svc = chiral_db_proto::chiral_db_server::ChiralDbServer::new(chiral_db_svc);
   
    // IPv6
    // let addr = "[::1]:10000".parse().unwrap(); 
    // IPv4
    let addr = "0.0.0.0:10000".parse().unwrap();
    tonic::transport::Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}