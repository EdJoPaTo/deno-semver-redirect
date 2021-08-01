use anyhow::anyhow;
use percent_encoding::percent_decode_str;
use semver::VersionReq;
use tide::http::mime;
use tide::utils::After;
use tide::{Redirect, Request, Response, StatusCode};

mod deno_land;

#[cfg(debug_assertions)]
const LISTENER: &str = "localhost:8080";

#[cfg(not(debug_assertions))]
const LISTENER: &str = "[::]:8080"; // Works for both IPv4 and IPv6

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    app.with(After(|mut res: Response| async {
        if let Some(err) = res.error() {
            let msg = format!("Error: {:?}", err);
            // NOTE: You may want to avoid sending error messages in a production server.
            res.set_body(msg);
        }
        Ok(res)
    }));

    app.at("/:package/:version/*path").get(deno_redirect);
    app.at("/index.css").get(|_| async {
        Ok(Response::builder(200)
            .body(include_str!("index.css"))
            .content_type(mime::CSS)
            .build())
    });
    app.at("/").get(|_| async {
        Ok(Response::builder(200)
            .body(include_str!("index.html"))
            .content_type(mime::HTML)
            .header("Content-Security-Policy", "frame-ancestors 'none';base-uri 'none';form-action 'none';default-src 'none';style-src 'self'")
            .build())
    });

    println!("http://localhost:8080/");
    app.listen(LISTENER).await?;
    Ok(())
}

#[allow(clippy::unused_async)]
async fn deno_redirect(req: Request<()>) -> tide::Result<Redirect<String>> {
    let package = req.param("package")?;
    let version = percent_decode_str(req.param("version")?).decode_utf8()?;
    let path = req.param("path")?;

    let meta = format!("package {:15} version {:8} path {}", package, version, path);

    let version_range = VersionReq::parse(&version).map_err(|err| {
        eprintln!("ERROR {}: {}", meta, err);
        tide::Error::new(
            StatusCode::BadRequest,
            anyhow!("could not parse version range"),
        )
    })?;

    let matching_version =
        deno_land::get_first_matching_version(package, &version_range).map_err(|err| {
            eprintln!("ERROR {}: {}", meta, err);
            tide::Error::new(
                StatusCode::NotFound,
                anyhow!("could not get all currently existing version"),
            )
        })?;

    let url = format!(
        "https://deno.land/x/{}@{}/{}",
        package, matching_version, path
    );
    println!("{} -> {}", meta, url);
    Ok(Redirect::temporary(url))
}
