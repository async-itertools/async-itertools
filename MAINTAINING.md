# Maintaining `async-itertools` releases

Presently, the project's releases are maintained using proprietary builds of `tagit`. Open-source
releases are available at <https://github.com/parrrate/opentagit>.

## Updating the version

Just change it in `Cargo.toml`.

## Preparing the changelog

This just puts the links and dates and sections in the right places.

```bash
tagit changelog
```

## Publishing the release

```bash
tagit tag
```
