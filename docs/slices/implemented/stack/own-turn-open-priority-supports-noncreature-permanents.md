# Own-Turn Open-Priority Supports Noncreature Permanents

## Goal

Make explicit that the current `OpenPriorityWindowDuringOwnTurn` corridor is modeled for the currently supported noncreature permanent spell subset, not as isolated one-off behavior for a single card type.

## Implemented behavior

- the current supported noncreature permanent subset for this rule includes `Artifact` and `Enchantment`
- both supported types now exercise the same own-turn open-priority corridor across the currently modeled windows
- documentation and executable coverage describe the rule as a shared supported family

## Out of scope

- extending the family beyond the currently supported noncreature permanent subset
- introducing broader contextual casting rules beyond the current explicit card-face permission
