# monto-rs

[![Build Status](https://travis-ci.org/melt-umn/monto-rs.svg?branch=master)](https://travis-ci.org/melt-umn/monto-rs)
[![Build Status](https://ci.appveyor.com/api/projects/status/jsy09v7mqxw2xqex/branch/master?svg=true)](https://ci.appveyor.com/project/remexre/monto-rs/branch/master)

A crate for the Monto protocol. This crate implements version 3.0.0-draft02 of the protocol, which is specified [here](https://melt-umn.github.io/monto-v3-draft/draft02).

## Notes

 - This is currently at a "good enough" stage; all the features fundamentally work, assuming clients and services that comply with the specification.
 - This could also use a large-scale reorganization, and the removal of a lot of "TODO Error Handling"s.
 - HTTP/2 support is blocked on [hyperium/hyper#304](https://github.com/hyperium/hyper/issues/304)
