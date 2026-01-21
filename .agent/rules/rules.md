---
trigger: always_on
---

antigravity:
  mode: controlled_executor
  description: >
    Antigravity is NOT a fully autonomous agent.
    It must strictly follow all rules defined in this file.

permissions:
  execution:
    cmd: true
    powershell: true
    javascript: true

architecture:
  required_file: explog_architechture
  enforcement:
    allow_only_defined_structure: true
    forbid_implicit_changes: true

review_policy:
  require_review: true
  block_execution_until_approved: true
  forbidden_before_review:
    - execute_command
    - modify_files
    - create_files
    - delete_files

versioning:
  mandatory: true
  strategy: semantic_versioning
  on_change:
    - bump_version
    - update_changelog
    - update_readme

documentation:
  changelog_file: CHANGELOG.md
  readme_file: README.md
  changelog_required_fields:
    - version
    - date
    - description

enforcement:
  priority: highest
  violation_action: hard_stop