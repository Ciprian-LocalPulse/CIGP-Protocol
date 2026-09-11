# Academic Evidence Package

Author: **Ciprian Ștefan Pleșca**  
Protocol version: `0.1`  
Status: Research documentation

This directory turns the CIGP reference implementation into a reviewable research record. It separates claims that can be checked by code from claims that require operational, statistical, legal, or institutional evidence.

| Document | Research function |
| --- | --- |
| [Evidence Graph](evidence-graph.md) | Connects claims to cryptographic artifacts, source modules, tests, and residual risks. |
| [Reproducibility Capsule](reproducibility-capsule.md) | Defines a compact, repeatable procedure for independent verification. |
| [Position Paper](position-paper.md) | States the problem, contribution, boundaries, and next research questions. |
| [Verification Receipt Standard](../standards/verification-receipt.md) | Defines a portable result object for an independent verifier. |

```mermaid
flowchart LR
    C[Research claim] --> E[Evidence artifact]
    E --> V[Independent verification]
    V --> R[Bounded result]
    R --> L[Published limitation]
```

The documents are intentionally conservative. A successful verification result means that supplied evidence is internally consistent with the CIGP reference construction. It is not a statement of real-world fairness, regulatory approval, operator honesty, or financial safety.
