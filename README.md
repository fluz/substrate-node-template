# Batch Calls

This is a project for experimenting with
[batch calls](https://substrate.dev/rustdocs/v2.0.0-rc4/pallet_utility/enum.Call.html#variant.batch)
on Substrate-based chains.

## Runtime Pallets

The FRAME runtime defined in this repo includes the
[Utility pallet](https://substrate.dev/rustdocs/v2.0.0-rc4/pallet_utility/index.html), which exposes
batch call capabilities, as well as a modified [template pallet](./pallets/template/src/lib.rs),
which makes it easy to invoke a dispatch and know whether it will succeed or result in an error. The
`do_something` dispatchable will succeed when it is called with `true` and fail when called with
`false`.

## Upstream

This project was forked from the
[Substrate Developer Hub Node Template](https://github.com/substrate-developer-hub/substrate-node-template).
