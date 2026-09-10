# Merkle Proofs

Ledger round hashes can be batched into a binary Merkle tree. Odd levels duplicate the final node. The root can be published as a batch commitment; an inclusion path then proves membership without disclosing other rounds.

Blockchain anchoring is a future extension and is not required by CIGP v0.1.
