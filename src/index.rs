pub const INDEX_HTML: &str = r#"<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8"/>
<title>Deno Semver Redirect</title>
<style>
html, body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", "Roboto", "Oxygen", "Ubuntu", "Cantarell", "Fira Sans", "Droid Sans", "Helvetica Neue", "Helvetica", "Arial", sans-serif;
    line-height: 1.5;
}
header, main, footer {
    margin: 0 auto;
    padding: 0 1rem;
    max-width: 800px;
}
img {
    max-width: 100%;
    display: block;
    margin: 0 auto;
}
a {
    color: #A00;
}
@media (prefers-color-scheme: dark){
    html, body {
        color: whitesmoke;
        background: #222;
    }
}
</style>
</head>
<body>
<header>
<h1>Deno Semver Redirect</h1>
<p>
This service allows you to use <a href="https://deno.land/x">Deno dependencies</a> via semantic versioning.
See also <a href="https://github.com/denoland/deno_website2/issues/606">this Deno Issue on GitHub</a>.
</p>
</header>
<main>
<p>
This service works via redirects.
The requested version range is parsed and the request is redirected to newest fitting version on deno.land/x.
This saves resources for this service and provides you with exactly the sources on deno.land/x.
</p>

<p>
Try the following urls:<br />
<a href="grammy/0.3.0/mod.ts">grammy@0.3.0</a><br />
<a href="std/0.95.0/fs/mod.ts">std@0.95.0</a><br />
</p>
</main>
<footer>
<p>
<a href="https://github.com/EdJoPaTo/deno-semver-redirect/">Source Code</a><br />
<a href="https://github.com/EdJoPaTo/deno-semver-redirect/blob/main/LICENSE">LICENSE (AGPL 3.0 or later)</a><br />
Created by <a href="https://edjopato.de/">EdJoPaTo</a>
</p>
</footer>
</body>
</html>
"#;
