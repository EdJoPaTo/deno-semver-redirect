use anyhow::{anyhow, Result};
use semver::{Version, VersionReq};
use serde::Deserialize;

const USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " ",
    env!("CARGO_PKG_REPOSITORY"),
);

#[derive(Debug, Deserialize)]
pub struct Versions {
    pub latest: String,
    pub versions: Vec<String>,
}

pub fn get_existing_versions(package_name: &str) -> Result<Versions> {
    let body = ureq::get(&format!(
        "https://cdn.deno.land/{package_name}/meta/versions.json"
    ))
    .set("User-Agent", USER_AGENT)
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
            return Ok(version_str);
        }
    }

    Err(anyhow!("found no matching version"))
}

#[test]
fn first_matching_works_for_grammy_example() {
    let result = get_first_matching_version("grammy", &VersionReq::parse("0.3").unwrap()).unwrap();
    assert_eq!(result, "v0.3.4");
}
