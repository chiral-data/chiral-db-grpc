use tonic;
use chiral_db;

mod data;
pub mod chiral_db_proto {
    tonic::include_proto!("chiraldb");
}

#[derive(Debug)]
struct ChiralDbService {
    fp_doc: std::sync::Arc<chiral_db::fingerprint::FingerprintDocument>,
}

#[tonic::async_trait]
impl chiral_db_proto::chiral_db_server::ChiralDb for ChiralDbService {
    async fn query_similarity(&self, request: tonic::Request<chiral_db_proto::RequestSimilarity>) -> Result<tonic::Response<chiral_db_proto::ReplySimilarity>, tonic::Status>{
        let smiles = &request.get_ref().mol.as_ref().unwrap().smiles;
        let cut_off = request.get_ref().cutoff;
        let results = chiral_db::similarity::query_similarity_for_smiles(smiles, &self.fp_doc, cut_off);
        Ok(tonic::Response::new(chiral_db_proto::ReplySimilarity { results }))
    }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chiral_db_svc = ChiralDbService {
        fp_doc: std::sync::Arc::new(data::load_fp_chembl(&chiral_db::fingerprint::kind_openbabel_ecfp4(2048)))
    };

    let addr = "[::1]:10000".parse().unwrap();
    let svc = chiral_db_proto::chiral_db_server::ChiralDbServer::new(chiral_db_svc);
    tonic::transport::Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}