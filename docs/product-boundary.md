# Open project and hosted service

Status: design draft

Groundset's open project must perform the complete basic job. The Hosted service makes that job easier and may use stronger components, but it does not own a different Grounding system or a private data format.

## Open and portable

Groundset will publish these assets under permissive licenses:

- Canonical record, Corpus release, Grounding request, and Evidence packet specifications;
- JSON Schemas, core vocabularies, Wire extension rules, and public APIs;
- example releases, validation fixtures, and the official compatibility test suite;
- a Reference implementation for importing material, creating and validating releases, building indexes, running Grounding, and producing Evidence packets;
- usable local components for every job required by the Reference implementation;
- the reproducible Technical proof and its configuration;
- reference and proving Corpus data where upstream rights allow redistribution.

Reference code uses Apache-2.0. Specifications and documentation use a permissive documentation license selected before publication. Upstream Corpus material keeps its own license and attribution requirements.

The Reference implementation works without a Groundset account or required network call after the chosen releases and components are installed. Update checks and telemetry are optional and disabled by default. Local and hosted Grounding use the same public request and response contracts, so an Agent builder can change the endpoint and credentials without rewriting the integration.

## One Grounding engine

The Reference implementation and Hosted service use the same open Grounding engine. This includes task interpretation, eligibility checks, policy application, Evidence selection, disagreement handling, budget handling, and packet assembly.

Deployments may choose different Grounding components. The Hosted service may use stronger embedding or reranking models, search systems, parsers, and computing resources. The Reference implementation supplies usable local defaults and also permits other compatible components.

Every Grounding Activity identifies the engine, policy, and component versions used. Groundset publishes a change to selection behavior in the open engine before deploying that change in the Hosted service. A component may affect candidate quality, but it does not introduce private authority, eligibility, or selection rules.

The public Technical proof runs the Reference implementation with frozen, reproducible components. Hosted results may be published as an additional comparison, not as a replacement for the public run.

## Hosted capabilities

The commercial Hosted service may keep its operational and convenience code private, including:

- tenant and team management;
- scheduling and managed acquisition;
- scaling, monitoring, backup, and recovery;
- administration interfaces;
- premium connectors;
- compatible higher-quality Grounding components;
- maintained public, private, commercial, and Access-only Corpora.

Hosted-only administration APIs may remain private. Data crossing the standard Grounding API or a Portable export uses published schemas. Customer-defined Wire extension profiles export with their schemas; internal scheduling, billing, and search data need not leave the service.

## Corpus access and export

Each Corpus keeps its own license and access rules. Groundset may distribute a maintained Corpus as a downloadable release, provide it through an API, or offer both.

An Access-only Corpus still has a fixed release identity. Before use, the service identifies it as non-portable and states its access, source-resolution, and retention limits. Evidence packets record the exact release used. A consumer may lose the ability to resolve its citations after access ends; Groundset must not present that arrangement as ownership of the source material.

A customer may create a Portable export at any time during service and for a documented period after cancellation. The export contains everything the customer is entitled to possess and needs to continue with the Reference implementation, including:

- source bytes and Corpus releases;
- customer Assertions, Assessments, Activities, Collections, and Agents;
- customer vocabularies and Wire extension profiles;
- Grounding policies and configuration;
- Evidence packets and audit history covered by the customer's retention rights.

Rebuildable indexes are not required in the export. Third-party material remains subject to its license. Groundset verifies that the Reference implementation can import the export rather than supplying an undocumented database dump.

## Public governance

Groundset develops v1 specifications in public through versioned proposals, tests, and releases. The project team has final authority during v1. An independent standards organization is deferred until outside adoption makes the additional governance useful.

Anyone may implement the specifications. An implementation may claim the official compatibility mark only for versions whose public conformance suite it passes. Groundset protects the project name and compatibility mark rather than restricting independent implementations through the code license.
