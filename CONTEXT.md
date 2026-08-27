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

**Grounding integration**:
The small Agent-runtime adapter that requests Grounding before a model invocation and supplies the resulting Evidence packet as context. It is inside the v1 replacement boundary, but the rest of the Agent runtime is not.
_Avoid_: Agent runtime, prompt template

**Condition**:
The evidence access granted to a model during a benchmark run.
_Avoid_: Mode, variant

**Benchmark protocol**:
The tools, feedback, time budget, and isolation rules applied to every Condition in a benchmark run.
_Avoid_: Product protocol, condition, mode

**Development set**:
The representative design-partner tasks that benchmark participants may inspect and use within a capped tuning budget.
_Avoid_: Evaluation set, training corpus

**Evaluation set**:
The representative design-partner tasks locked before final tuning and withheld until the benchmark Conditions are frozen.
_Avoid_: Development set, live traffic

**Proof of value**:
Evidence that an Agent builder can replace a bespoke RAG path with Groundset while preserving or improving task quality, traceability, and maintenance effort. Model-size substitution and epistemic quality are secondary v1 measurements, in that order.
_Avoid_: Model benchmark, proof of concept

**Bespoke RAG baseline**:
The Agent builder's tuned retrieval system evaluated against Groundset with the same corpus, model, tools, time budget, and task set. Model-only evaluation is diagnostic; naive top-k retrieval is not the primary comparison.
_Avoid_: Naive RAG, model-only baseline

**Proof gate**:
A separate pass condition for task quality, traceability, maintenance effort, or integration burden. Groundset must pass every Proof gate rather than hide a regression inside a composite score.
_Avoid_: Composite score

**Task quality**:
The task-specific correctness or usefulness of an Agent's result. Measure it with executable checks or exact-answer grading where possible, and a blinded expert rubric when judgment is unavoidable.
_Avoid_: LLM judge score, preference alone

**Traceability**:
The ability to resolve an Agent's material claims through cited Evidence to exact source selections and determine whether those selections support the claims. Measure reference resolvability, claim-to-source coverage, and support correctness separately.
_Avoid_: Citation presence, provenance alone

**Maintenance effort**:
The work required to keep both a Corpus and its Agent integration current as material is added, updated, superseded, restricted, or removed. Corpus work and integration-code work are measured separately.
_Avoid_: Integration burden, upkeep score

**Integration burden**:
The hands-on engineering time and Agent-architecture changes required to replace a bespoke RAG path once the proving Corpus is ready.
_Avoid_: Lines of code, Corpus preparation

**Design-partner validation**:
A v1 trial in which one Agent builder replaces a bespoke RAG path for representative tasks, maintains the Corpus without help from Groundset's authors, and records a written choice to keep Groundset after independently running the Evaluation set and maintenance scenarios. Production deployment is not required for v1.
_Avoid_: Integration demo, production validation
