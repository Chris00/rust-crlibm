Crlibm
======

This crate is a binding to [CRlibm][], an efficient and proved
correctly-rounded mathematical library.
For the user convenience, this module embeds the relevant C code from
the [CRlibm Git repository][crlibm-git].

Note that the C code of CRlibm is mature but superseded by
[MetaLibm][] ([repository][MetaLibm-git]) and will therefore not
receive updates.

## Example

```rust
use crlibm::*;
let x = sinpi_rd(2.);
let y = sinpi_rn(2.);
let z = sinpi_ru(2.);
```

## Features

Be default, the crate uses portable implementations of the logarithm.
The feature `log_double_extended` enables implementations optimized
for processors with double-extended hardware (if in addition the
correct platform is detected).  These versions may not be faster.


[CRlibm]: https://web.archive.org/web/20161027224938/http://lipforge.ens-lyon.fr/www/crlibm
[crlibm-git]: https://scm.gforge.inria.fr/anonscm/git/metalibm/crlibm.git
[MetaLibm]: http://www.metalibm.org/
[MetaLibm-git]: https://github.com/metalibm/metalibm
