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

### Collection

An attributed grouping assembled for a purpose. A corpus, multimodal example, review set, or vocabulary can be a Collection. Membership says that a curator selected an item. It does not assert that every member has the same semantic type.

### Term

A versioned vocabulary definition. A Term defines either a Category or a Relationship predicate. Terms have namespaced identifiers, labels, definitions, and optional validation rules for allowed subjects and objects.

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

## Change and correction

Snapshots and Assertions are immutable. A correction creates another Assertion and connects it with `supersedes`, `contradicts`, or another attributed relationship. Activities record every transformation so derived Artifacts and search projections can be regenerated.

The canonical corpus does not store a universal truth flag or one global authority score. Assessments name their assessor, criteria, method, Scope, and time. Retrieval policy may calculate task-specific rankings from those assessments.

## Deferred decisions

The first version will not define:

- a universal domain ontology;
- a universal Scope expression language;
- automatic semantic claim deduplication;
- a general inference engine;
- a global source authority score;
- canonical embeddings or chunk boundaries.

These features should be added only after a concrete corpus and retrieval task require them.
