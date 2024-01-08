#![cfg(feature = "wasmer")]
wai_bindgen_rust::export!("semver.wai");
use semver_upstream;

pub struct Semver;
impl semver::Semver for Semver {

}

pub struct BuildMetadata(semver_upstream::BuildMetadata);
impl semver::BuildMetadata for BuildMetadata {}
