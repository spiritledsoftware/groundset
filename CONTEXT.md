# Groundset

Groundset is domain-neutral knowledge infrastructure for humans and machines. It preserves source material, records attributed interpretations, and supplies small evidence packets for particular tasks.

## Language

**Entity**:
A stable identity for a concrete or abstract subject, such as a place, software package, organization, or design principle.
_Avoid_: Knowledge object, record

**Concept**:
An abstract Entity used to connect definitions, examples, indicators, exceptions, and related ideas without declaring one formulation authoritative.
_Avoid_: Claim, category

**Artifact**:
A logical information-bearing item, such as a document, dataset, code example, image, or recording. An Artifact may be acquired or derived and may have multiple Snapshots.
_Avoid_: Evidence, snapshot

**Snapshot**:
An immutable, content-addressed representation of an Artifact at a particular time.
_Avoid_: Artifact, current version

**Agent**:
A person, organization, system, model, or sensor responsible for an Assertion or Activity.
_Avoid_: User, author

**Agent builder**:
A person or team that integrates Groundset into a domain-specific AI agent or application. Agent builders are the first v1 product role.
_Avoid_: User, developer

**Activity**:
An attributed event that acquires, publishes, parses, renders, transforms, evaluates, or verifies material.
_Avoid_: Process metadata

**Assertion**:
An attributed recording of a Claim or semantic Relationship. Recording an Assertion does not make it true.
_Avoid_: Fact, truth

**Claim**:
A proposition expressed by an Assertion that may have support, contradiction, and a stated Scope.
_Avoid_: Fact, assertion

**Term**:
A versioned vocabulary entry that defines a semantic Category or Relationship predicate.
_Avoid_: Tag, concept

**Category**:
A Term that describes what an Entity, Artifact, Assertion, or other record is. Category membership is expressed by an Assertion.
_Avoid_: Collection, structural kind, tag

**Relationship**:
A semantic connection between identified things, recorded as an attributed Assertion with a namespaced Term as its predicate.
_Avoid_: Reference, database relation

**Collection**:
An attributed selection of records assembled for a purpose. Membership does not imply that all members share a Category.
_Avoid_: Category, corpus

**Corpus boundary**:
The storage and authorization boundary containing canonical records and any derived indexes or Evidence packets. Material crosses it only through an explicit publication or export operation. A Corpus boundary is not a canonical record or semantic grouping.
_Avoid_: Collection, record ACL

**Source**:
The origin cited for acquired material or an Assertion. Source is a role played by an Artifact, Snapshot, or Agent rather than a separate kind of knowledge.
_Avoid_: Reference, evidence

**Scope**:
The versions, dates, places, populations, platforms, and other constraints under which an Assertion applies.
_Avoid_: Context, validity metadata

**Evidence**:
A Snapshot selection, Artifact, or Assertion used to support or contradict a conclusion for a particular task.
_Avoid_: Context, search result

**Evidence packet**:
The small set of Evidence supplied to a model for one task.
_Avoid_: Retrieval dump, context packet

**Grounding**:
The task-time selection of applicable, traceable material into an Evidence packet for an Agent.
_Avoid_: Search, ingestion

**Condition**:
The evidence access granted to a model during a benchmark run.
_Avoid_: Mode, variant

**Benchmark protocol**:
The tools, feedback, time budget, and isolation rules applied to every Condition in a benchmark run.
_Avoid_: Product protocol, condition, mode
