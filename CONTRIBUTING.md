# Contributing

Thanks for taking the time to contribute to this project!

All changes need to:

- pass basic checks, including tests, formatting and lints,
- be signed-off.

## Basic checks

We are using standard Rust ecosystem tools including `rustfmt` and `clippy` with one minor difference.
Due to a couple of `rustfmt` features being available only in nightly (see the `.rustfmt.toml` file) nightly `rustfmt` is necessary.

All of these details are captured in a `.justfile` and can be checked by running [`just`'](https://just.systems/).

To run all checks locally before sending them to CI you can set your git hooks directory:

```sh
git config core.hooksPath scripts/hooks/
```

## Developer Certificate of Origin

The sign-off is a simple line at the end of the git commit message, which certifies that you wrote it or otherwise have the right to pass it on as a open-source patch.

The rules are pretty simple: if you can [certify the below][DCO]:

```
Developer's Certificate of Origin 1.1

By making a contribution to this project, I certify that:

(a) The contribution was created in whole or in part by me and I
    have the right to submit it under the open source license
    indicated in the file; or

(b) The contribution is based upon previous work that, to the best
    of my knowledge, is covered under an appropriate open source
    license and I have the right under that license to submit that
    work with modifications, whether created in whole or in part
    by me, under the same open source license (unless I am
    permitted to submit under a different license), as indicated
    in the file; or

(c) The contribution was provided directly to me by some other
    person who certified (a), (b) or (c) and I have not modified
    it.

(d) I understand and agree that this project and the contribution
    are public and that a record of the contribution (including all
    personal information I submit with it, including my sign-off) is
    maintained indefinitely and may be redistributed consistent with
    this project or the open source license(s) involved.
```

then you just add a line saying

    Signed-off-by: Random J Developer <random@developer.example.org>

using your name.

If you set your `user.name` and `user.email`, you can sign your commit automatically with [`git commit --signoff`][GSO].

To sign-off your last commit:

    git commit --amend --signoff

[DCO]: https://developercertificate.org
[GSO]: https://git-scm.com/docs/git-commit#git-commit---signoff

If you want to fix multiple commits use:

    git rebase --signoff main

To check if your commits are correctly signed-off locally use `just check-commits`.
