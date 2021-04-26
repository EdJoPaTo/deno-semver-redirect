# Deno Semver Redirect

[![Rust](https://github.com/EdJoPaTo/deno-semver-redirect/actions/workflows/rust.yml/badge.svg)](https://github.com/EdJoPaTo/deno-semver-redirect/actions/workflows/rust.yml)
[![Docker Hub Image](https://img.shields.io/docker/image-size/edjopato/deno-semver-redirect)](https://hub.docker.com/r/edjopato/deno-semver-redirect)

Redirect Deno dependencies from semantic versions to the newest fitting version on [deno.land/x](https://deno.land/x).
See also this [Deno Issue](https://github.com/denoland/deno_website2/issues/606).

For the requested versions the [semver crate](https://crates.io/crates/semver) is used.
Check its README for "Requirements" on what it supports.

## How to use

Just change the import URL in your Deno Sources:

```diff
-import { Bot } from 'https://deno.land/x/grammy@0.3.1/mod.ts'
+import { Bot } from 'https://dsr.edjopato.de/grammy/0.3/mod.ts'
```
