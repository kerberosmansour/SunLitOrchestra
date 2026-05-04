---
skill: slo-talk-to-users
case: happy-path
case-name: happy-path
category: happy-path
expected-behavior: Produce a confidential UK user-interview artifact with consent and git-warning discipline.
expected_behavior: Produce a confidential UK user-interview artifact with consent and git-warning discipline.
risk: high
---

## Input
~~~text
/slo-talk-to-users pre-interview for a UK founder interviewing a named logistics operator.
~~~

## Expected Behavior
Write the confidential artifact path, include Mom Test questions, cite the UK consent script, and warn when a remote git repo could leak `docs/biz/`.

## Must Not
- Put real interviewee details under `docs/biz-public/`.
- Draft marketing outreach instead of interview preparation.
