# Deno SemVer Redirect

Redirect Deno dependencies from [semantic versions](https://semver.org/) to the newest fitting version on [deno.land/x](https://deno.land/x).
See also this [Deno Issue](https://github.com/denoland/deno_website2/issues/606).

> [!WARNING]
> <https://deno.land> is being replaced by <https://jsr.io>

## How to use

Just change the import URL in your Deno Sources:

```diff
-import { Bot } from 'https://deno.land/x/grammy@v0.3.1/mod.ts'
+import { Bot } from 'https://dsr.edjopato.de/grammy/0.3/mod.ts'
```

## Version Requirements

These are all possible as version requirements:

```plaintext
>=1.0.0
<1.3.0

1.2.3  := >=1.2.3 <1.3.0
1.2    := >=1.2.0 <1.3.0
1      := >=1.0.0 <2.0.0

~1.2.3 := >=1.2.3 <1.3.0
~1.2   := >=1.2.0 <1.3.0
~1     := >=1.0.0 <2.0.0

^1.2.3 := >=1.2.3 <2.0.0
^0.2.3 := >=0.2.3 <0.3.0
^0.0.3 := >=0.0.3 <0.0.4
^0.0   := >=0.0.0 <0.1.0
^0     := >=0.0.0 <1.0.0

0.2.x  := >=0.2.0 <0.3.0
0.x    := >=0.0.0 <1.0.0
```

For the requested versions the [`semver` crate](https://crates.io/crates/semver) is used.
For more details check its documentation.

When the dependency you are using is prefixing its versions with `v` (like `v1.2.3`), the version requirement still has to be provided without the `v`.
The redirect then will include the `v` prefix based on the package.
