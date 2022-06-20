use chiral_db;

pub fn load_fp_chembl(fpk: &chiral_db::fingerprint::Kind) -> chiral_db::fingerprint::FingerprintDocument {
    chiral_db::fingerprint::FingerprintDocument::new_from_chembl(fpk)
}