# Data model

Status: design draft

Groundset preserves original material and records interpretations around it. It does not translate every source into one supposedly authoritative knowledge graph.

## Data layers

Groundset uses three data models for three different jobs.

| Layer | Purpose | Lifetime |
| --- | --- | --- |
| Canonical corpus | Preserve artifacts, provenance, and attributed assertions | Durable |
| Search projections | Support text, vector, structural, and graph retrieval | Replaceable |
| Evidence packet | Supply the smallest useful result for one task | Request-specific |

Chunks, embeddings, ranking scores, and query-time trust scores belong in search projections or evidence packets. They are not canonical records.

### Corpus boundaries

A Corpus boundary is the minimum storage and authorization boundary. Canonical records, derived indexes, and Evidence packets produced from restricted material remain inside the same boundary. A Collection is a semantic grouping and never an authorization boundary.

Canonical records do not carry per-record access-control rules. Such rules would require every derivation, relationship, index entry, and packet fragment to propagate policy correctly. Instead, material crosses a Corpus boundary only through an explicit publication or export Activity checked by the deployment against source rights. The canonical model records that Activity and its provenance; it does not define the authorization engine.

Public corpus deployments contain no private record, selector, derived Assertion, or opaque pointer that would disclose restricted material.

## Canonical record kinds

### Entity

A stable identity for something assertions can describe. An Entity may be concrete, such as an organization, or abstract, such as YAGNI. Semantic categories distinguish people, products, places, heuristics, and other domain concepts.

### Artifact

A logical information-bearing item. Documents, datasets, code examples, images, screenshots, recordings, and normalized text are Artifacts. An Artifact may come from an external source or from a recorded Activity.

### Snapshot

An immutable representation of an Artifact. A Snapshot records its content hash, media type, byte length, acquisition time, publication time when known, license, and blob location.

Successive captures of a changing Artifact produce new Snapshots. A rendering, transcription, or normalized representation is a derived Artifact with its own Snapshot.

### Assertion

An attributed recording of a claim or semantic relationship. An Assertion has one of two body forms:

```json
{
  "type": "claim",
  "language": "en",
  "text": "Do not implement a capability until a concrete requirement needs it."
}
```

```json
{
  "type": "relation",
  "subject": "urn:entity:yagni",
  "predicate": "https://example.org/decision-knowledge/instanceOf",
  "object": "https://example.org/decision-knowledge/Heuristic"
}
```

An Assertion records who or what it is attributed to, which Activity recorded or derived it, its Scope, and citations to exact Snapshot selections. The corpus does not treat an Assertion as true merely because it exists.

Groundset does not deduplicate semantically equivalent claims in the first version. Later processes may record `exactMatch`, `closeMatch`, or contradiction assertions with their own provenance.

### Agent

A person, organization, model, software system, or sensor responsible for an Assertion or Activity.

### Activity

An acquisition, publication, extraction, transformation, rendering, evaluation, or verification event. An Activity records its inputs, outputs, responsible Agents, time, method, and tool versions.

Activity roles remain distinct from Assertion attribution:

- `attributedTo` identifies the Agent whose claim, position, or observation an Assertion records;
- `generatedBy` identifies the Activity that recorded or derived it;
- the Activity's responsible Agents identify who or what performed that work.

This distinction lets a recording remain attributed to its speaker while separately identifying the interviewer, transcriber, translator, model, or sensor involved in its derivation.

### Collection

An attributed grouping assembled for a purpose. A multimodal example, review set, release selection, or vocabulary can be a Collection. Membership says that a curator selected an item. It does not assert that every member has the same semantic type or access policy.

### Term

A versioned vocabulary definition. A Term defines either a Category or a Relationship predicate. Terms have namespaced identifiers, labels, and definitions. Vocabulary and extension profiles define any extra validation rules outside the core Term shape.

## Canonical wire contract

Groundset Canonical records use ordinary JSON validated by JSON Schema 2020-12. The schema has a versioned identifier. A Corpus release declares that schema once in its manifest; records do not repeat a schema version.

Every record has this closed envelope:

```json
{
  "id": "urn:entity:yagni",
  "type": "Entity",
  "extensions": {
    "https://example.org/profile/v1": {}
  }
}
```

- `id` is an absolute URI or URN.
- `type` is one of the eight Canonical record kinds.
- Each record kind has a closed shape. Unknown fields are rejected.
- `extensions` is optional. Every key is an absolute profile IRI, and that profile owns validation of its value.

Semantic extension does not use `extensions`. Publishers define namespaced Terms and record attributed Assertions. Wire extension profiles are reserved for machine-facing data that the core record schema cannot represent.

### Identifiers and references

Record references are bare identifier strings. A string in a reusable value position always refers to a Canonical record; source strings use typed literals instead. Groundset does not wrap every reference in a `{ "ref": "..." }` object.

Snapshot identifiers encode the digest of the exact captured bytes:

```text
urn:sha256:<64 lowercase hexadecimal characters>
```

The Snapshot does not repeat that digest in another field. The Corpus release manifest handles transfer-integrity metadata. A Snapshot keeps its content locator and upstream source locator separate from its immutable identifier.

### Selections and typed literals

A Snapshot selection contains the Snapshot identifier and one closed selector:

```json
{
  "snapshot": "urn:sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
  "selector": {
    "type": "text-quote",
    "exact": "Do not implement a capability until a concrete requirement needs it."
  }
}
```

The core selector profile defines text quote, text position, and byte range. A domain selector uses an absolute profile IRI as its type and places its profile-validated payload under `value`. Short custom selector names are rejected.

A typed literal always records a scalar value and datatype. A language tag is valid only for a string; a unit is valid only for a number.

```json
{
  "value": 21.4,
  "datatype": "http://www.w3.org/2001/XMLSchema#decimal",
  "unit": "https://example.org/unit/degree-celsius"
}
```

JSON Schema checks those structural combinations. Datatype and unit meaning belongs to vocabulary and Corpus conformance rules.

### Assertion contract

An Assertion body is either a textual Claim or a semantic Relationship. Claim bodies require text and language. Relationship bodies use the shared record-reference, Snapshot-selection, and typed-literal value forms.

Every Assertion requires:

- at least one `attributedTo` Agent;
- one `generatedBy` Activity.

`scope` and `citations` are optional, but each must be nonempty when present. Their absence means that no Scope or citation was recorded. It never means universally applicable, verified, or true. Scope values may reference records or typed literals, but not source selections.

### Activity contract

An Activity records its start time and at least one role binding:

```json
{
  "id": "urn:activity:render-example",
  "type": "Activity",
  "startedAt": "2026-08-27T10:00:00Z",
  "roles": [
    {
      "role": "https://example.org/groundset/used",
      "value": "urn:sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    },
    {
      "role": "https://example.org/render/viewportWidth",
      "value": {
        "value": 1280,
        "datatype": "http://www.w3.org/2001/XMLSchema#integer",
        "unit": "https://example.org/unit/css-pixel"
      }
    }
  ]
}
```

Namespaced role bindings represent inputs, outputs, responsible Agents, methods, tools, and domain parameters without adding scenario-specific Activity fields. The core vocabulary defines reusable provenance roles. Domain profiles define parameter roles.

### Validation boundary

Groundset has two validation tiers:

1. Record schema validation checks one Canonical record's closed structural shape.
2. Corpus conformance checks identifier uniqueness, reference closure and referenced kinds, Snapshot digest-to-byte agreement, selector bounds, datatype semantics, time ordering, and Activity role semantics.

JSON Schema cannot perform the second tier reliably. A record may pass its schema while belonging to a nonconforming Corpus.

### JSON-LD projection

Canonical records do not contain `@context`. Groundset may export a deterministic JSON-LD representation that wraps records in a published context and graph. The projection maps Groundset provenance, selector, and vocabulary concepts to standards such as PROV, Web Annotation, and SKOS without creating a second Canonical record shape.

## Reusable values

Relationship endpoints, Activity parameters, Scope constraints, and citations reuse three value forms:

1. a canonical record identifier;
2. a Snapshot selection;
3. a typed literal with a datatype and, when applicable, a unit or language.

Illustrative typed literal:

```json
{
  "type": "literal",
  "value": 21.4,
  "datatype": "http://www.w3.org/2001/XMLSchema#decimal",
  "unit": "https://example.org/unit/degree-celsius"
}
```

Typed literals preserve source values. Confidence, probability, severity, authenticity, and authority do not become universal record fields; a domain vocabulary may use typed literals in attributed Assertions or assessments.

## Source selections

A citation points to an immutable Snapshot and identifies the relevant part with a selector. Selectors may address text quotes, byte ranges, pages, timestamps, image regions, table rows, JSON pointers, headings, or code symbols.

```json
{
  "snapshot": "urn:sha256:abc123",
  "selector": {
    "type": "text-quote",
    "exact": "Do not implement a capability until a concrete requirement needs it."
  }
}
```

Generated chunks are search projections. They do not replace Snapshot selectors as durable citations.

Selector types are extensible and namespaced. Domain profiles may define selectors for geometry features, legal sections, formal-language symbols, media tracks, dataset cells, or other source-native structures without changing the core record kinds.

## Scope constraints

Scope states when and where an Assertion applies. It is a list of predicate/value constraints whose predicates are versioned Terms and whose values are canonical record identifiers or typed literals.

```json
{
  "scope": [
    {
      "predicate": "https://example.org/groundset/validFrom",
      "object": {
        "type": "literal",
        "value": "2025-07-01",
        "datatype": "http://www.w3.org/2001/XMLSchema#date"
      }
    },
    {
      "predicate": "https://example.org/law/jurisdiction",
      "object": "urn:place:example-state"
    }
  ]
}
```

The core vocabulary defines only broadly reusable temporal predicates such as `validFrom` and `validUntil`. Domain vocabularies define constraints such as jurisdiction, population, platform, package version range, axiom system, equipment, or forecast target interval.

Scope is deliberately not a universal rule language. Complex exceptions remain separate attributed Assertions. Publication, acquisition, and observation times belong to Snapshots and Activities; they must not be substituted for the time during which a Claim applies.

## Relationships

Structural links required to parse a record are fields in the core model. Examples include the Artifact belonging to a Snapshot and the Activity that generated a derived Artifact.

Semantic relationships are Assertions with a namespaced predicate Term:

```text
subject -> predicate -> object
```

The subject and object may identify a canonical record, a Snapshot selection, or a typed literal such as a date, measurement, or version range.

The initial core vocabulary should stay small:

```text
instanceOf
subclassOf
partOf
memberOf
derivedFrom
cites
supports
contradicts
supersedes
```

Third parties can publish domain vocabularies without changing the core model. Groundset may borrow established semantics such as W3C PROV for provenance and SKOS for taxonomies without requiring an RDF store or general-purpose ontology reasoner.

When a semantic relationship needs more than two endpoints, its event or situation becomes an Entity and attributed Relationship Assertions connect participants to it by named roles. Groundset does not add a different core record kind for every domain event.

## Categories, collections, and tags

These classifications are intentionally different:

```text
Structural kind: Artifact
Semantic category: ResearchArticle
Collection: 2026 Climate Review Corpus
Local tag: Needs Human Review
```

Structural kind controls the record shape. Semantic category membership is an attributed `instanceOf` Assertion. Collection membership records curation. A tag is a local label and carries no global inference rules.

## Abstract concepts

An abstract idea has a stable Entity as its meeting point. Assertions connect that Entity to competing definitions, recommendations, problem patterns, rationale, indicators, diagnostic questions, exceptions, examples, and counterexamples.

```text
YAGNI
├── instanceOf -> Heuristic
├── hasDefinition -> attributed Assertion
├── hasIndicator -> attributed Assertion
├── hasException -> attributed Assertion
├── exemplifiedBy -> Collection or Artifact
└── relatedTo -> premature generalization
```

Predicates such as `hasIndicator` belong to a domain vocabulary rather than the core vocabulary. Several sources may define YAGNI differently. Groundset preserves those differences instead of manufacturing one universal definition.

Retrieval matches an unfamiliar situation against problem patterns, examples, and applicability conditions. The resulting statement that a concept may apply is a query-time interpretation, not a permanent fact.

## Compound artifacts

A Collection can group multiple representations of an example. A Tailwind centering example might contain an HTML Artifact, a rendered screenshot Artifact, and layout measurements. A rendering Activity connects the screenshot to the exact HTML Snapshot, dependency versions, browser, and viewport that produced it.

```text
Tailwind centering example
├── HTML Artifact and Snapshot
├── extracted symbol annotations
├── rendering Activity
├── PNG Artifact and Snapshot
├── layout measurements
└── Assertions describing what the example demonstrates
```

The source Snapshot remains preserved. Symbol annotations and measurements are derived Assertions linked to source selections. The screenshot is a derived Artifact, not metadata attached to the HTML.

The same pattern covers transcripts, translations, vectorized maps, edited photographs, formal proofs, evaluation logs, and published aggregates. Each durable representation is an Artifact; the transformation is an Activity; interpretations and measurements are Assertions.

Large datasets and observation streams remain Artifact content. Groundset does not require a canonical Assertion for every row or sample. A task may promote selected values to Assertions while citing the original Snapshot with a row, cell, time, or structural selector.

## Change and correction

Snapshots and Assertions are immutable. A correction creates another Assertion and connects it with `supersedes`, `contradicts`, or another attributed relationship. Activities record every transformation so derived Artifacts and search projections can be regenerated.

The canonical corpus does not store a universal truth flag or one global authority score. Assessments name their assessor, criteria, method, Scope, and time. Retrieval policy may calculate task-specific rankings from those assessments.

Retractions, legal amendments, forecast revisions, taxonomic changes, vulnerability updates, and conflicting testimony all follow this rule. A later Assertion may retract, narrow, contradict, or supersede an earlier one without deleting the earlier source or claim.

## Deferred decisions

The first version will not define:

- a universal domain ontology;
- a universal Scope expression language;
- the published schema URI and permanent vocabulary IRIs;
- the complete Corpus conformance and extension-profile specifications;
- corpus-boundary authorization and export-policy enforcement;
- the repository and downloadable corpus-release format;
- automatic semantic claim deduplication;
- a general inference engine;
- a global source authority score;
- canonical embeddings or chunk boundaries.

These features should be added only after a concrete corpus and retrieval task require them.
