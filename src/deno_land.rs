use anyhow::{anyhow, Result};
use semver::{Version, VersionReq};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Versions {
    pub latest: String,
    pub versions: Vec<String>,
}

pub fn get_existing_versions(package_name: &str) -> Result<Versions> {
    let body = ureq::get(&format!("https://cdn.deno.land/{}/meta/versions.json", package_name))
        .call()?
        .into_string()?;
    let versions = serde_json::from_str::<Versions>(&body)?;

    Ok(versions)
}

pub fn get_first_matching_version(
    package_name: &str,
    target_version_range: &VersionReq,
) -> Result<String> {
    let versions = get_existing_versions(package_name)?.versions;

    for version_str in versions {
        let raw_version = version_str.strip_prefix('v').unwrap_or(&version_str);
        let version = Version::parse(raw_version)?;

        if target_version_range.matches(&version) {
            return Ok(version_str.to_owned());
        }
    }

    Err(anyhow!("found no matching version"))
}
