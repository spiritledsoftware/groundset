# Adjacent standards and reusable data models

Status: research for `Survey adjacent standards and reusable data models`

Date: 2026-08-26

## Recommendation in one page

Groundset should use a small, profile-based vocabulary rather than adopt one existing graph model wholesale.

1. **Adopt provenance terms and selector concepts directly.** Map `Entity`, `Activity`, and `Agent` to the corresponding W3C PROV concepts. Use Web Annotation's `SpecificResource` and selector ideas for durable source-fragment citations. Keep Groundset's immutable `Snapshot` as the citation anchor.
2. **Adopt JSON Schema as the normative wire contract.** Make ordinary JSON the required interchange form for records and evidence packets. Publish an optional JSON-LD 1.1 representation for linked-data consumers. A client should not need an RDF or JSON-LD processor to consume a packet.
3. **Use SKOS as a vocabulary mapping, not as Groundset's structural model.** Export labels, definitions, broader/narrower, related, and mapping relationships where useful. Keep attributed Groundset Assertions around these terms so competing definitions and relationships remain visible.
4. **Hash exact Snapshot bytes with an algorithm-tagged identifier.** RFC 9530's digest fields are useful for transfer integrity, but they are not a complete content-addressing scheme. Record the hash algorithm, digest, media type, and representation bytes; use RDF Dataset Canonicalization only when signing or identifying an RDF export.
5. **Ingest software manifests through adapters.** Normalize the small set of package facts Groundset needs, such as name, ecosystem, version, digest, license, source, and dependency edges. Import/export SPDX 3.0.1 and CycloneDX 1.7 rather than making either BOM format the canonical corpus model.
6. **Treat signed media provenance as optional.** Import C2PA manifests into Groundset Activities and Assertions when present. Do not require C2PA for documents, source code, or ordinary derived artifacts.
7. **Do not make Verifiable Credentials, DIDs, a universal ontology, a global trust score, or an RDF triple store v1 requirements.** They solve narrower or more operationally expensive problems than the first evidence workflow requires.

## Comparison and decisions

| Need | Primary standard/model | What it gives Groundset | Decision |
| --- | --- | --- | --- |
| Activity provenance | [W3C PROV-O](https://www.w3.org/TR/prov-o/) and [PROV-DM](https://www.w3.org/TR/prov-dm/) | Common `Entity`, `Activity`, and `Agent` vocabulary; derivation, attribution, use, generation, source, and time relations | **Adopt terms and map fields.** Keep a compact JSON profile and do not require all PROV influence classes or an ontology store. |
| Source-fragment citation | [Web Annotation Data Model](https://www.w3.org/TR/annotation-model/) | `SpecificResource`, `Selector`, text quote/position, media fragments, image regions, and representation state | **Adopt selector concepts.** Require `Snapshot` plus selector in Groundset citations; support quote and position selectors first, then page, time, region, and structured-data selectors. |
| Taxonomies and labels | [SKOS Reference](https://www.w3.org/TR/skos-reference/) | Concepts, concept schemes, collections, preferred/alternate labels, definitions, notes, broader/narrower, related, and mapping properties | **Map/export.** Use SKOS properties in vocabulary profiles, but do not equate SKOS `Collection` with Groundset `Collection` or make SKOS relationships un-attributed truth. |
| Exact content integrity | [RFC 9530 Digest Fields](https://www.rfc-editor.org/rfc/rfc9530) | Standard HTTP `Content-Digest` and `Repr-Digest` fields and registered digest syntax | **Use for transfer verification.** Groundset still defines algorithm-tagged Snapshot IDs over exact bytes; a digest header alone is not a persistent identity. |
| Linked-data exchange | [RDF 1.1 Concepts](https://www.w3.org/TR/rdf11-concepts/) | Triple statements, RDF graphs, and datasets containing a default graph plus named graphs | **Optional export.** A named graph can carry an Assertion's export, but Groundset must define attribution semantics in its profile. Do not force every packet into triples. |
| JSON linked-data representation | [JSON-LD 1.1](https://www.w3.org/TR/json-ld11/) | JSON-based linked-data serialization with an upgrade path from ordinary JSON | **Optional representation.** Keep packet and record JSON self-describing without requiring `@context`; publish a context for clients that want IRIs and graph interchange. |
| Wire validation | [JSON Schema Draft 2020-12](https://json-schema.org/draft/2020-12/json-schema-core.html) | A JSON media type for schemas and a standard vocabulary for validating instances | **Adopt directly.** Version schemas, publish examples, and validate records and packets at the exchange boundary. Avoid encoding semantic truth or inference in the schema. |
| RDF shape validation | [SHACL](https://www.w3.org/TR/shacl/) | Shapes for validating RDF graphs against constraints | **Optional conformance adapter.** Useful for RDF exports and vocabulary profiles, but not needed to validate the primary JSON model. |
| Software package inventory | [SPDX 3.0.1](https://spdx.github.io/spdx-spec/v3.0.1/) | Package/file/dataset models, hashes, versions, package URLs, licenses, and verification-related properties | **Adapter and export.** SPDX's license and package identity fields are useful for software source; its document model is not Groundset's general Artifact/Assertion model. |
| Dependency and component BOM | [CycloneDX 1.7 JSON reference](https://cyclonedx.org/docs/1.7/) | Components, hashes, package URLs, dependency graphs, tools, metadata, and lifecycle context | **Adapter and export.** Preserve dependency edges and provenance in Groundset; do not make `bom-ref` or a BOM document the universal Entity identity. |
| Media provenance/authenticity | [C2PA Specifications](https://spec.c2pa.org/specifications/specifications/2.2/index.html) | Signed media manifests containing provenance assertions, ingredients, and validation information | **Optional import/export.** Map manifests to Activities, Agents, derived Artifacts, and Assertions for supported media. Do not require C2PA for all content. |
| Signed claims | [Verifiable Credentials Data Model v2.0](https://www.w3.org/TR/vc-data-model-2.0/) | Issuer/holder/verifier credential model, claims, and cryptographic securing mechanisms | **Avoid as the core model.** Groundset records attributed Assertions without treating them as credentials. Add a credential envelope only for a concrete interoperable signing requirement. |
| Canonical RDF bytes | [RDF Dataset Canonicalization 1.0](https://www.w3.org/TR/rdf-canon/) | Normalization of RDF datasets for comparison, hashing, and signing | **Use only for RDF export.** Snapshot identity is over captured bytes; canonicalizing every JSON record as RDF would add complexity without improving source fidelity. |

## Provenance: PROV vocabulary, Groundset identity

PROV-O defines an Entity as a physical, digital, conceptual, or other thing with fixed aspects, an Activity as something that occurs over time and acts on or with entities, and an Agent as something bearing responsibility. That maps cleanly to Groundset's `Artifact`/`Snapshot`, `Activity`, and `Agent` vocabulary.

The useful first profile is:

```text
Groundset Snapshot  -> prov:Entity
Groundset Activity  -> prov:Activity
Groundset Agent     -> prov:Agent

produced Snapshot   -> prov:wasGeneratedBy
used Snapshot       -> prov:used
output derived from -> prov:wasDerivedFrom
attributed to       -> prov:wasAttributedTo
primary source      -> prov:hadPrimarySource
activity responsible-> prov:wasAssociatedWith
```

`Artifact` is a logical identity while `Snapshot` is an immutable representation. Both can be represented as PROV entities in an export, but Groundset should distinguish them in its own model. A changing web page therefore has one logical Artifact and multiple Snapshots; a rendering Activity produces a new derived Artifact and Snapshot.

Do not require every record to carry every PROV relation. In v1, acquisition, transformation, rendering, and extraction Activities should carry explicit input and output Snapshot references, responsible Agent, method/tool identity, and time. The remaining PROV vocabulary can be added when a workflow needs it.

## Source-fragment selectors

Web Annotation models an annotation as connected resources, normally a body and target, and provides a `SpecificResource` to describe a constrained use of a source. Its selectors include text quotes, text positions, media fragments, and other resource-specific selection mechanisms. This is a close match for Groundset's evidence rule: an Artifact is not intrinsically Evidence; a source selection becomes Evidence for a task or Assertion.

Groundset should use a citation shape equivalent to:

```json
{
  "snapshot": "urn:groundset:snapshot:sha256:...",
  "selector": {
    "type": "text-quote",
    "exact": "...",
    "prefix": "...",
    "suffix": "..."
  }
}
```

Recommended selector profile:

- `text-quote` for portable exact text, with optional prefix/suffix;
- `text-position` for offsets into a declared text representation;
- `byte-range` for exact binary or source bytes;
- `page`, `region`, and `time` for PDF, image, audio, and video;
- `json-pointer`, `table-cell`, and `code-symbol` for structured or parsed derivatives.

The Snapshot ID must be present even if the selector carries a URL. Dynamic representation state, such as a request header or capture time, belongs to the Snapshot or selector state so a later reader can reproduce what was selected. Generated chunks are projections and must not replace these selectors.

## Vocabularies and taxonomies

SKOS is a good vocabulary interchange layer. `skos:Concept`, `skos:ConceptScheme`, `skos:Collection`, labels, definitions, notes, semantic relations, and mapping properties cover the ordinary needs of a controlled vocabulary.

Groundset should map its `Term` to a SKOS concept or property when a publisher wants SKOS output. It should map `prefLabel`, `altLabel`, `definition`, `scopeNote`, `broader`, `narrower`, `related`, and cross-scheme mapping properties where their meanings fit.

There are two important boundaries:

1. Groundset's `Concept` is an abstract `Entity`, not necessarily a SKOS Concept. It may have competing attributed definitions, examples, exceptions, and applicability patterns. SKOS labels and definitions should be preserved as sourced Assertions rather than collapsed into one canonical text.
2. Groundset's `Collection` is any attributed selection, including multimodal examples and review sets. SKOS `Collection` is a concept grouping construct. They are related ideas, not interchangeable structural kinds.

Do not use SKOS hierarchy as an inference engine. `broader` and `narrower` express vocabulary relationships; they do not establish universal applicability or truth for a retrieved answer.

## Content identity and integrity

RFC 9530 standardizes HTTP digest fields for message content and representations. Groundset can ingest these values and verify that an HTTP transfer or representation matches the source's declared digest. For a Snapshot, Groundset should additionally record:

- the exact bytes captured;
- a cryptographic hash and algorithm identifier;
- media type and representation metadata;
- capture time and source locator;
- publication or version information when known.

A digest is an integrity check, not a complete identity policy. Two different serializations of equivalent JSON do not have the same byte digest. Groundset should therefore keep Snapshot identity tied to the captured representation and only define canonicalization for a representation where the workflow explicitly needs equivalence.

If Groundset exports RDF datasets and needs stable signatures or graph IDs, RDF Dataset Canonicalization 1.0 is the appropriate scoped tool. It should not become a prerequisite for ordinary Snapshot IDs.

C2PA addresses signed provenance and authenticity for media. Its manifests are useful evidence about the history of an image, video, or other supported asset. Groundset should import a C2PA manifest as provenance Activities, Agents, derived Artifacts, and Assertions, preserving the original manifest as an Artifact. C2PA should remain optional because much of Groundset's corpus is text, source, data, or private material.

## Package and dependency manifests

SPDX and CycloneDX answer a narrower question than Groundset: what packages, files, licenses, hashes, tools, and dependencies are present in a software or component inventory? That is useful for the Tailwind example and similar technical corpus items, but a BOM is neither a general source Artifact nor an attributed knowledge Assertion.

An ingestion adapter should normalize package facts into Groundset records:

```text
software package Entity
  -> name, ecosystem, version, package URL, source locator
package Snapshot or manifest Artifact
  -> exact manifest bytes and hash
package dependency Assertion
  -> subject, dependsOn predicate, object, source selector
license Assertion
  -> license term, source, and applicable Scope
```

SPDX provides rich license and package/file identity vocabulary. CycloneDX provides a practical component/dependency graph and lifecycle metadata. Support both adapters. Preserve the original `package.json`, `Cargo.toml`, lockfile, or BOM Snapshot, and never treat a normalized package identity as a replacement for that source.

## Claims and machine-facing exchange

RDF 1.1 describes triples as statements and datasets as a default graph plus named graphs. That is useful for exporting Groundset's relationships and grouping an attributed Assertion's statements. RDF itself does not supply Groundset's source-selection, assessment, or disagreement policy, so those remain profile fields.

The primary Groundset exchange should be plain JSON validated by JSON Schema 2020-12. An Assertion should carry explicit attribution, recording Activity, Scope, and source selections. A relation body can be exported as a triple-like object, but the JSON form should remain legible without graph terminology.

JSON-LD 1.1 is a strong optional representation because it is JSON-based and can provide IRIs and graph interchange without forcing a new packet format. Publish a stable context for records and packets only after field names and extension rules settle. A JSON-LD context must not be the only place where a packet's meaning is defined.

For RDF exports, define a Groundset profile that maps one attributed Assertion to a named graph or equivalent export envelope. Do not rely on a generic RDF consumer to infer that an un-attributed triple is a sourced or verified statement.

SHACL is appropriate for validating RDF exports and vocabulary-specific constraints, such as allowed subject/object categories for a predicate. JSON Schema remains the first validation boundary because Groundset's required interchange is JSON.

## What Groundset should not adopt in v1

- **One universal ontology.** PROV and SKOS provide useful reusable terms, not a complete domain-neutral ontology for every corpus.
- **RDF as the storage requirement.** RDF/JSON-LD export should be possible, but canonical records and evidence packets should remain usable with ordinary JSON and non-RDF storage.
- **A truth or authority score baked into Assertions.** Standards that support signatures or credentials do not justify a universal ranking. Keep attribution, criteria, Scope, and method explicit; compute task-specific ranking at retrieval time.
- **Verifiable Credentials or DIDs as default identity.** VC 2.0 is designed for issuer/holder/verifier credentials and cryptographic securing. Groundset's ordinary attributed Assertions do not need that ceremony. Add it only for a specific federation or signature requirement.
- **A BOM as the canonical model.** SPDX and CycloneDX are excellent software inventory interchange formats but do not model all Groundset artifacts, source selectors, abstract Concepts, or evidence packets.
- **C2PA as universal provenance.** Its signed media focus does not fit every text, code, data, and private-source workflow.
- **Canonical embeddings, chunks, or ranking fields.** These remain replaceable search projections, as required by `docs/data-model.md`.

## Proposed v1 interoperability profile

The v1 specification should publish these profiles and adapters:

1. **Groundset JSON Core**, validated with JSON Schema 2020-12. Defines Entity, Artifact, Snapshot, Assertion, Agent, Activity, Collection, Term, Scope, selectors, and evidence packets.
2. **Groundset Provenance Profile**, mapping the compact Activity model to PROV-O terms.
3. **Groundset Selector Profile**, mapping Snapshot citations to Web Annotation selectors and SpecificResource semantics.
4. **Groundset Vocabulary Profile**, mapping Terms and taxonomy relationships to SKOS where applicable.
5. **Groundset Linked Data Profile**, an optional JSON-LD 1.1 context and RDF export mapping. Define named-graph attribution semantics and use RDF Dataset Canonicalization only for signed/canonical RDF datasets.
6. **Groundset Software Inventory Adapters**, import/export for SPDX 3.0.1 and CycloneDX 1.7, with package-manager-native manifest capture preserved.
7. **Groundset Media Provenance Adapter**, optional C2PA import/export for supported media.

This keeps the durable corpus faithful to sources while giving standards-aware clients useful mappings. It also leaves room to change storage and projections without breaking the packet or source-citation contract.

## Sources consulted

All sources below are first-party specifications or their authoritative project references, checked 2026-08-26.

- W3C, [PROV-O: The PROV Ontology](https://www.w3.org/TR/prov-o/)
- W3C, [PROV-DM: The PROV Data Model](https://www.w3.org/TR/prov-dm/)
- W3C, [Web Annotation Data Model](https://www.w3.org/TR/annotation-model/)
- W3C, [SKOS Simple Knowledge Organization System Reference](https://www.w3.org/TR/skos-reference/)
- W3C, [RDF 1.1 Concepts and Abstract Syntax](https://www.w3.org/TR/rdf11-concepts/)
- W3C, [JSON-LD 1.1](https://www.w3.org/TR/json-ld11/)
- W3C, [Shapes Constraint Language (SHACL)](https://www.w3.org/TR/shacl/)
- W3C, [RDF Dataset Canonicalization 1.0](https://www.w3.org/TR/rdf-canon/)
- W3C, [Verifiable Credentials Data Model v2.0](https://www.w3.org/TR/vc-data-model-2.0/)
- IETF, [RFC 9530: Digest Fields](https://www.rfc-editor.org/rfc/rfc9530)
- JSON Schema, [Draft 2020-12 Core](https://json-schema.org/draft/2020-12/json-schema-core.html)
- SPDX, [SPDX Specification 3.0.1](https://spdx.github.io/spdx-spec/v3.0.1/)
- CycloneDX, [v1.7 JSON Reference](https://cyclonedx.org/docs/1.7/)
- Coalition for Content Provenance and Authenticity, [C2PA Specifications](https://spec.c2pa.org/specifications/specifications/2.2/index.html)
