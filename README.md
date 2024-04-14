# Blockchain Commons `bc-server`

<!--Guidelines: https://github.com/BlockchainCommons/secure-template/wiki -->

### _by $major-authors_

**BlockchainCommons Server** (bc-server) is a modular lightweight server codebase for use by BlockchainCommons projects.

The goal of this project is to allow any BlockchainCommons project to expose its API as a HTTP service in a standardized and easy way. A project can either re-use this code or intergrate with it by adding a module that makes use of the project.

BlockchainCommons has several projects as commandline tools which could be made more accessible for testing and useful if a server is running that exposes their functionality.

bc-server will expose its functionality using a JSON-RPC interface.

To this end, we have the following requirements:

- A BlockchainCommons command line tool should be able to describe a simple manifest specifying
  - named endpoints (path and parameters)
  - the command to be executed when an endpoint is called.
- The manifest should be written as a rust library and can contain arbitrary logic to process input and generate output.
  - This library can then be put into a "modules" directory and the server will automatically make its functionality available.

## [Work In Progress] The bc-server API (How to write a module for bc-server)

NB. The functionality described in this section is not yet fully implemented. Consider it a description of how we want the module development process to work.

- The `modules` folder in the root of the bc-server crate contains all modules.
- A module is defined as a sub-crate of bc-server. Each modules corresponds to a bc-server crate `feature` and can be enabled/disabled.
  - Ref. [Specifying Dependencies](cargo/reference/specifying-dependencies.html#specifying-path-dependencies)
    <>
- In the `modules` directory, copy and rename the `example` directory.
  - Add dependencies to your module's Cargo.toml .
  - Ensure that you fill in the following required module functions in `example.rs`:
    - `make_routes()`: This function generates the routes that will be exposed by your module.
    - `start_server()`:
      - Put any startup code required by your module here e.g. initializing a database.
      - Check for and configure any required dependencies here.
- Add your module as a dependeny in `bc-server`'s [Cargo.toml](/Cargo.toml).

## Modules

The following modules are implemented:

- [x] depo:
  - uses [blockchaincommons/bc-depo-rust](https://github.com/blockchaincommons/bc-depo-rust) for secure storage and retrieval of binary objects.

The following APIs are work-in-progress:

- [ ] torgap-demo:
  - Torgap-demo allows a user to sign objects and serve them an onion service.
- [ ] spotbit

## How to test bc-server

```sh
docker compose up
```

OR

```sh
docker compose run -it --service-ports bcserver bash

```

Then verify that you can connect to the API: `curl -vv 127.0.0.1:5332/api/status`

The output log of `docker compose` contains information on what ports and services are running.

## How to deploy bc-server

## References

- [Core Lightning](https://github.com/ElementsProject/lightning) (aka `lightningd`) has a module/plugin system that will server as a model for BC-Server.
  - See [A day in the life of a plugin
    ](https://github.com/ElementsProject/lightning/blob/master/doc/developers-guide/plugin-development/a-day-in-the-life-of-a-plugin.md)

## Bugs

- During compilation you might have to wrestle with: <https://users.rust-lang.org/t/error-e0635-unknown-feature-stdsimd/106445/2>

```

Error[E0635]: unknown feature `stdsimd`
--> /home/nik/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ahash-0.8.3/src/lib.rs:99:4

```

- To fix:
  - `rustup default nightly`
  - `rm Cargo.lock` (because it will have conflicting versions of ahash)

---

<!-- ## Additional Information

The following files contain…

- `$ListOfEssentialDocs`
-->

<!-- ## Installation Instructions -->

## Usage Instructions

- From within the bc-server directory run:

```sh
docker compose up
```

OR

```sh
docker compose run -it --service-ports bcserver bash

```

Then verify that you can connect to the API: `curl -vv 127.0.0.1:5332/api/status`

The output log of `docker compose` contains information on what ports and services are running.

## Gordian Principles

` bc-server` is a reference implementation meant to display the [Gordian Principles`](https://github.com/BlockchainCommons/Gordian#gordian-principles), which are philosophical and technical underpinnings to Blockchain Commons' Gordian technology. This includes:

- **Independence.** `how does it demonstrate independence`
- **Privacy.** `how does it demonstrate privacy`
- **Resilience.** `how does it demonstrate resilience`
- **Openness.** `how does it demonstrate openness`

Blockchain Commons apps do not phone home and do not run ads. Some are available through various app stores; all are available in our code repositories for your usage.

`REMOVE THIS SECTION UNLESS THIS IS A REFERENCE APP MEANT TO DEMONSTRATE GORDIAN PRINCIPLES`

## Status - Alpha

` bc-server` is currently under active development and in the alpha testing phase. It should not be used for production tasks until it has had further testing and auditing. See [Blockchain Commons' Development Phases](https://github.com/BlockchainCommons/Community/blob/master/release-path.md).

### Version History

### Roadmap

## Origin, Authors, Copyright & Licenses

Unless otherwise noted (either in this [/README.md](./README.md) or in the file's header comments) the contents of this repository are Copyright © 2020 by Blockchain Commons, LLC, and are [licensed](./LICENSE) under the [spdx:BSD-2-Clause Plus Patent License](https://spdx.org/licenses/BSD-2-Clause-Patent.html).

In most cases, the authors, copyright, and license for each file reside in header comments in the source code. When it does not, we have attempted to attribute it accurately in the table below.

This table below also establishes provenance (repository of origin, permalink, and commit id) for files included from repositories that are outside of this repo. Contributors to these files are listed in the commit history for each repository, first with changes found in the commit history of this repo, then in changes in the commit history of their repo of their origin.

| File                                        | From                                                                                                 | Commit                                                       | Authors & Copyright (c) | License                              |
| ------------------------------------------- | ---------------------------------------------------------------------------------------------------- | ------------------------------------------------------------ | ----------------------- | ------------------------------------ |
| exception-to-the-rule.c or exception-folder | [https://github.com/community/repo-name/PERMALINK](https://github.com/community/repo-name/PERMALINK) | [https://github.com/community/repo-name/commit/COMMITHASH]() | 2020 Exception Author   | [MIT](https://spdx.org/licenses/MIT) |

### Dependencies

To build `bc-server` you'll need to use the following tools:

- autotools - Gnu Build System from Free Software Foundation ([intro](https://www.gnu.org/software/automake/manual/html_node/Autotools-Introduction.html)).

Other prerequisites include:

...

### Libraries

Libraries used in bc-server are listed in the [Cargo.toml](/Cargo.toml).

### Derived from ...

This `bc-server` project is either derived from:

- [blockchaincommons/bc-depo-rust](https://github.com/blockchaincommons/bc-depo-rust).

<!--
## Subsequent Usage

### Adapted by ...

These are adaptations, conversions, and wrappers that make `bc-server` available for other languages:

- [community/repo-name/](https://github.com/community/repo-name) — Repo that does what, by [developer](https://github.com/developer) or from [community](https://community.com)(language).

### Used by ...

These are other projects that directly use `bc-server`:

- [community/repo-name/](https://github.com/community/repo-name) — Repo that does what, by [developer](https://github.com/developer) or from [community](https://community.com)(use OR fork [version] OR include [version]).

Libraries may be marked as `use` (the current version of our repo is used), `fork` (a specific version of our repo has been forked for usage), or `include` (files from a specific version of our repo have been included).

### Used with ...

These are other projects that work with or leverage `bc-server`:

- [community/repo-name/](https://github.com/community/repo-name) — Repo that does what, by [developer](https://github.com/developer) or from [community](https://community.com).

-->

## Financial Support

`bc-server` is a project of [Blockchain Commons](https://www.blockchaincommons.com/). We are proudly a "not-for-profit" social benefit corporation committed to open source & open development. Our work is funded entirely by donations and collaborative partnerships with people like you. Every contribution will be spent on building open tools, technologies, and techniques that sustain and advance blockchain and internet security infrastructure and promote an open web.

To financially support further development of `bc-server` and other projects, please consider becoming a Patron of Blockchain Commons through ongoing monthly patronage as a [GitHub Sponsor](https://github.com/sponsors/BlockchainCommons). You can also support Blockchain Commons with bitcoins at our [BTCPay Server](https://btcpay.blockchaincommons.com/).

<!--
### Project Sponsors

Thanks to our project sponsors for their support of `bc-server`:

$sponsor-logo-with-link

$sponsor-description
-->

## Contributing

We encourage public contributions through issues and pull requests! Please review [CONTRIBUTING.md](./CONTRIBUTING.md) for details on our development process. All contributions to this repository require a GPG signed [Contributor License Agreement](./CLA.md).

### Discussions

The best place to talk about Blockchain Commons and its projects is in our GitHub Discussions areas.

[**Gordian Developer Community**](https://github.com/BlockchainCommons/Gordian-Developer-Community/discussions). For standards and open-source developers who want to talk about interoperable wallet specifications, please use the Discussions area of the [Gordian Developer Community repo](https://github.com/BlockchainCommons/Gordian-Developer-Community/discussions). This is where you talk about Gordian specifications such as [Gordian Envelope](https://github.com/BlockchainCommons/Gordian/tree/master/Envelope#articles), [bc-shamir](https://github.com/BlockchainCommons/bc-shamir), [Sharded Secret Key Reconstruction](https://github.com/BlockchainCommons/bc-sskr), and [bc-ur](https://github.com/BlockchainCommons/bc-ur) as well as the larger [Gordian Architecture](https://github.com/BlockchainCommons/Gordian/blob/master/Docs/Overview-Architecture.md), its [Principles](https://github.com/BlockchainCommons/Gordian#gordian-principles) of independence, privacy, resilience, and openness, and its macro-architectural ideas such as functional partition (including airgapping, the original name of this community).

[**Gordian User Community**](https://github.com/BlockchainCommons/Gordian/discussions). For users of the Gordian reference apps, including [Gordian Coordinator](https://github.com/BlockchainCommons/iOS-GordianCoordinator), [Gordian Seed Tool](https://github.com/BlockchainCommons/GordianSeedTool-iOS), [Gordian Server](https://github.com/BlockchainCommons/GordianServer-macOS), [Gordian Wallet](https://github.com/BlockchainCommons/GordianWallet-iOS), and [SpotBit](https://github.com/BlockchainCommons/spotbit) as well as our whole series of [CLI apps](https://github.com/BlockchainCommons/Gordian/blob/master/Docs/Overview-Apps.md#cli-apps). This is a place to talk about bug reports and feature requests as well as to explore how our reference apps embody the [Gordian Principles](https://github.com/BlockchainCommons/Gordian#gordian-principles).

[**Blockchain Commons Discussions**](https://github.com/BlockchainCommons/Community/discussions). For developers, interns, and patrons of Blockchain Commons, please use the discussions area of the [Community repo](https://github.com/BlockchainCommons/Community) to talk about general Blockchain Commons issues, the intern program, or topics other than those covered by the [Gordian Developer Community](https://github.com/BlockchainCommons/Gordian-Developer-Community/discussions) or the
[Gordian User Community](https://github.com/BlockchainCommons/Gordian/discussions).

### Other Questions & Problems

As an open-source, open-development community, Blockchain Commons does not have the resources to provide direct support of our projects. Please consider the discussions area as a locale where you might get answers to questions. Alternatively, please use this repository's [issues](./issues) feature. Unfortunately, we can not make any promises on response time.

If your company requires support to use our projects, please feel free to contact us directly about options. We may be able to offer you a contract for support from one of our contributors, or we might be able to point you to another entity who can offer the contractual support that you need.

### Credits

The following people directly contributed to this repository. You can add your name here by getting involved. The first step is learning how to contribute from our [CONTRIBUTING.md](./CONTRIBUTING.md) documentation.

| Name              | Role                | Github                                           | Email                                 | GPG Fingerprint                                   |
| ----------------- | ------------------- | ------------------------------------------------ | ------------------------------------- | ------------------------------------------------- |
| Christopher Allen | Principal Architect | [@ChristopherA](https://github.com/ChristopherA) | \<ChristopherA@LifeWithAlacrity.com\> | FDFE 14A5 4ECB 30FC 5D22 74EF F8D3 6C91 3574 05ED |
| Nicholas Ochiel   | Principal Architect | [@nochiel](https://github.com/ChristopherA)      | \<nochiel@users.noreply.github.com\>  | 45EA 5C81 9B7E E915 C2A2 7C64 4444 1190 7BE8 83D9 |

## Responsible Disclosure

We want to keep all of our software safe for everyone. If you have discovered a security vulnerability, we appreciate your help in disclosing it to us in a responsible manner. We are unfortunately not able to offer bug bounties at this time.

We do ask that you offer us good faith and use best efforts not to leak information or harm any user, their data, or our developer community. Please give us a reasonable amount of time to fix the issue before you publish it. Do not defraud our users or us in the process of discovery. We promise not to bring legal action against researchers who point out a problem provided they do their best to follow the these guidelines.

### Reporting a Vulnerability

Please report suspected security vulnerabilities in private via email to ChristopherA@BlockchainCommons.com (do not use this email for support). Please do NOT create publicly viewable issues for suspected security vulnerabilities.

The following keys may be used to communicate sensitive information to developers:

| Name              | Fingerprint                                       |
| ----------------- | ------------------------------------------------- |
| Christopher Allen | FDFE 14A5 4ECB 30FC 5D22 74EF F8D3 6C91 3574 05ED |

You can import a key by running the following command with that individual’s fingerprint: `gpg --recv-keys "<fingerprint>"` Ensure that you put quotes around fingerprints that contain spaces.

```

```
