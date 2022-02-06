# Notes

Starting out the words _term_ and _expression_ are used interchangably, but the story is a bit more rich. term is typically reserved for a more specialzed sense of phrases representing computations.

Syntax:

In general, describes the language. Syntax can be defined in several ways, BNF grammars, inductively via terms of a powerset, or inference rules, or concretely via set operations.

---
Semantics:

A definition for how the language is evaluated. 

Three basic approaches to formalizing semantics: operational, denotational and axiomatic.

Semantic Styles:

1. Operational - Specifies the behavior of the language by defining an abstract state machine using the language. Machine behavior is defined by a transition function that either gives the next state by performing a simplification step or indicates that the machine has halted.

    Vocab: _meaning_: The meaning of a term can be taken to be the final state the machine reaches when started with the given term as it's initial state.

1. Denotational - Broad generalization: uses abstract mathematical language to describe the evaluation of a language. This involves finding a collection of semantic domains and an appropriate mapping function to map terms into elements of the semantic domains. 

1. Axiomatic - Starts by identifying behaviors of programs and then derives laws from the definitions. The laws are treated _themselves_ as the definition of the language.

---

_stuckness_

A closed term is "stuck", if it is in normal form but not a value.