# Professional AWS Organization Setup — Startup Infrastructure (AI-First Runbook v2)

> **Purpose**: Stand up a production-grade, multi-account AWS Organization with defense-in-depth security, fault-tolerant availability zones, IaC via Pulumi, hardened CI/CD, ransomware-resilient backups, and software supply chain security — all sized for a small startup.
> **Audience**: AI coding agents first, humans second. This document is written to reduce ambiguity, prevent scope drift, and improve code quality with the same model capability.
> **How to use**: Work through milestones sequentially. Before starting any milestone, read its full section and the Global Execution Rules. After completing it, follow the Global Exit Rules. Never skip ahead. Never silently widen scope.
> **Prerequisite reading**: [Pulumi AWS Get Started](https://www.pulumi.com/docs/iac/get-started/aws/configure/), [AWS Organizations Best Practices](https://docs.aws.amazon.com/organizations/latest/userguide/orgs_best-practices.html), [NIST SP 800-218 SSDF](https://csrc.nist.gov/publications/detail/sp/800-218/final), [DoD Enterprise DevSecOps Fundamentals](https://dodcio.defense.gov/Library/), [NSA/CISA Securing the Software Supply Chain](https://media.defense.gov/2022/Sep/01/2003068942/-1/-1/0/ESF_SECURING_THE_SOFTWARE_SUPPLY_CHAIN_DEVELOPERS.PDF)

---

## Runbook Metadata

- **Runbook ID**: `aws-org`
- **Prefix for test files and lessons files**: `aws-org`
- **Primary stack**: `Pulumi (TypeScript) + AWS`
- **Primary package/app names**: `infra-org`, `infra-network`, `infra-security`, `infra-eks`, `infra-cicd`, `infra-backup`
- **Default test commands**:
  - IaC unit tests: `cd infra && npm test`
  - IaC preview: `pulumi preview --stack <stack>`
  - IaC deploy: `pulumi up --stack <stack> --yes`
  - E2E validation: `cd infra && npm run test:e2e`
  - Build/boot: `pulumi up --stack org-master --yes && pulumi up --stack network-prod --yes`
- **Allowed new dependencies by default**: `none`
- **Schema/config migration allowed by default**: `no`
- **Public interfaces that must remain stable unless explicitly listed otherwise**:
  - AWS account IDs and OU structure
  - VPC CIDR allocations
  - DNS zone delegations
  - IAM role trust policies across accounts
  - EKS cluster endpoints
  - CI/CD pipeline definitions

---

## Milestone Tracker

Update this table as each milestone is completed. This is the single source of truth for progress.

| # | Milestone | Status | Started | Completed | Lessons File | Completion Summary |
|---|---|---|---|---|---|---|
| 1 | AWS Organization & Account Structure | `not_started` | | | | |
| 2 | Identity, Governance & Guardrails | `not_started` | | | | |
| 3 | Network Architecture — VPCs, Zones & DNS | `not_started` | | | | |
| 4 | Security Baseline — Logging, Detection & Response | `not_started` | | | | |
| 5 | CI/CD Pipeline — Hardened & Least-Privilege | `not_started` | | | | |
| 6 | EKS Runtime Environment | `not_started` | | | | |
| 7 | Software Supply Chain Security | `not_started` | | | | |
| 8 | Backup, Recovery & Ransomware Resilience | `not_started` | | | | |
| 9 | Operational Readiness & Validation | `not_started` | | | | |

<!-- Status values: not_started | in_progress | blocked | done -->
<!-- Lessons files go in docs/lessons/aws-org-m<N>.md -->
<!-- Completion summaries go in docs/completion/aws-org-m<N>.md -->

---

## Global Execution Rules

These rules apply to every milestone without exception.

### 1) Stay inside scope

- Only change files listed in the current milestone unless a listed step explicitly requires one additional file.
- Do not refactor unrelated infrastructure.
- Do not rename existing AWS resources, IAM roles, DNS zones, or Pulumi stack names unless the milestone explicitly says so.
- Do not introduce a new Pulumi provider or npm dependency unless the milestone explicitly allows it.
- Do not change existing account structures, CIDR allocations, or IAM trust policies unless the milestone explicitly includes migration work and rollback tests.

### 2) Tests define the contract

- Write Pulumi unit tests (using `@pulumi/pulumi/runtime/mocks`) before production Pulumi code.
- Write E2E validation scripts (using AWS SDK calls to verify deployed resources) before deploying.
- Confirm new tests fail for the right reason before implementing.
- A milestone is not done when `pulumi preview` succeeds. It is done when the declared contract is satisfied and evidence is recorded.

### 3) No placeholders in production paths

The following are not allowed unless explicitly permitted in the milestone:

- TODO or placeholder logic in Pulumi programs
- Silent fallbacks that hide deployment errors
- Swallowed errors without structured logging or user-visible handling
- Hard-coded secrets, AWS access keys, or default passwords in source control
- Overly permissive IAM policies (`*` actions or `*` resources) left in place after tests pass
- Security groups with `0.0.0.0/0` ingress unless explicitly required and documented
- Commented-out dead code or temporary mocks in production stacks

### 4) Preserve backwards compatibility

Every milestone must explicitly verify that previously deployed resources, DNS records, IAM roles, and network routes still function unless the milestone explicitly replaces them.

### 5) Prefer smallest safe change

- Prefer narrow, targeted Pulumi resource changes over broad stack rewrites.
- Prefer extending existing patterns (e.g., adding a new OU) over inventing new abstractions.
- Prefer deleting unused resources over adding compatibility shims.
- If a refactor is required, keep it minimal and directly justified by the milestone goal.

### 6) Record evidence, not claims

All meaningful checks must be recorded in the milestone Evidence Log:

- command run
- relevant file or resource
- expected result
- actual result
- pass/fail
- notes

---

## Global Entry Rules (Pre-Milestone Protocol)

Do this before every milestone.

1. Read the lessons file from the previous milestone, if one exists. Apply any design corrections, naming rules, and failure-mode coverage it calls for before writing new code.
2. Read the current milestone fully: goal, context, contract block, out-of-scope block, file list, BDD scenarios, regression tests, E2E tests, smoke tests, and definition of done.
3. Run the full existing test suite and confirm it passes. Record the baseline in the Evidence Log.
   ```
   cd infra && npm test
   pulumi preview --stack <relevant-stack>
   ```
   If any tests fail before you start, stop and fix the baseline first. Do not begin a milestone on a red baseline.
4. Read the files listed in "Files Allowed To Change" and "Files To Read Before Changing Anything". Understand their current shape before editing.
5. Update the Milestone Tracker in this file: set the current milestone status to `in_progress` and record the Started date.
6. Create Pulumi unit test files first.
7. Create E2E runtime validation test stubs first.
8. Copy the milestone's Evidence Log template into working notes and begin filling it out as work happens.
9. Re-state the milestone constraints in your own words before coding:
   - goal
   - allowed files
   - forbidden changes
   - compatibility requirements
   - tests that must pass

---

## Global Exit Rules (Post-Milestone Protocol)

Do this after every milestone.

1. Run the full test suite. Every pre-existing test must still pass. Every new test must pass.
   ```
   cd infra && npm test
   ```
2. Run the milestone E2E runtime validation tests.
   ```
   cd infra && npm run test:e2e
   ```
3. Verify the stack deploys cleanly.
   ```
   pulumi up --stack <stack> --yes
   ```
4. Run the smoke tests listed in the milestone. Check off each item in the runbook.
5. Verify backward compatibility for all items listed in the milestone Compatibility Checklist.
6. Complete the Self-Review Gate.
7. Update ARCHITECTURE.md following the Documentation Update Table.
8. Update README.md if user-facing capabilities changed.
9. Write a lessons-learned file at `docs/lessons/aws-org-m<N>.md`.
10. Write a completion summary at `docs/completion/aws-org-m<N>.md`.
11. Update the Milestone Tracker in this file: set status to `done`, record Completed date, and fill in the lessons and completion summary paths.
12. Re-read the next milestone with fresh eyes and record any assumption changes in the lessons file.

---

## Background Context

### Current State

The startup has no existing AWS infrastructure. Development has been done on local machines. There is no formalized cloud account structure, no IaC, no centralized logging, and no incident response tooling. DNS is managed manually via a registrar. Deployments are manual or ad-hoc.

### Problem

1. **No account isolation**: A single AWS account (or none) means a blast radius that encompasses everything — dev, staging, prod, security logs, and billing. A compromised workload can access billing, delete logs, or pivot to production data.
2. **No network segmentation**: Without VPCs, subnets, NACLs, and security groups designed intentionally, workloads share flat networks. Internal services are exposed. There is no separation between public-facing and internal traffic.
3. **No identity governance**: Without SSO, MFA enforcement, SCPs, and least-privilege IAM, any developer with root credentials can destroy the organization. No audit trail for who did what.
4. **No CI/CD hardening**: Without a locked-down pipeline, attackers can inject malicious code, exfiltrate secrets, or deploy backdoors. No SBOM generation, no image signing, no artifact provenance.
5. **No DNS strategy**: Without split-horizon DNS, internal service discovery leaks to the internet. Without DNSSEC, DNS responses can be spoofed.
6. **No backup/recovery for ransomware**: Without immutable backups, vault locks, cross-account replication, and tested restore procedures, a ransomware attack means total data loss.
7. **No software supply chain security**: Without SBOM, image scanning, dependency pinning, signed artifacts, and admission control, the software supply chain is an open attack surface (ref: NIST SSDF SP 800-218, NSA/CISA Securing the Software Supply Chain, DoD Enterprise DevSecOps Fundamentals).
8. **No observability or detection**: Without CloudTrail, GuardDuty, Security Hub, Config, and centralized logging, breaches go undetected for months.

### Target Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        AWS ORGANIZATION (Management)                        │
│  ┌─────────────────┐  ┌──────────────────┐  ┌───────────────────────────┐  │
│  │ SCPs & Policies  │  │ AWS SSO / IAM IC │  │ CloudTrail (Org-wide)     │  │
│  └─────────────────┘  └──────────────────┘  └───────────────────────────┘  │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌──────────────────── Organizational Units ──────────────────────────┐     │
│  │                                                                    │     │
│  │  ┌─────────┐ ┌──────────┐ ┌──────────┐ ┌─────────┐ ┌──────────┐ │     │
│  │  │Security │ │  Shared   │ │   Dev    │ │ Staging │ │  Prod    │ │     │
│  │  │  Acct   │ │ Services │ │  Acct    │ │  Acct   │ │  Acct    │ │     │
│  │  └────┬────┘ └────┬─────┘ └────┬─────┘ └────┬────┘ └────┬─────┘ │     │
│  │       │            │            │             │           │        │     │
│  └───────┼────────────┼────────────┼─────────────┼───────────┼────────┘     │
│          │            │            │             │           │              │
│  ┌───────▼────┐ ┌─────▼──────┐    │             │    ┌──────▼───────┐      │
│  │GuardDuty   │ │Transit GW  │    │             │    │ EKS Cluster  │      │
│  │SecurityHub │ │Route 53    │    │             │    │ (Multi-AZ)   │      │
│  │CloudTrail  │ │ECR (shared)│    │             │    │ ┌──────────┐ │      │
│  │Config      │ │CI/CD       │    │             │    │ │Workloads │ │      │
│  │Detective   │ │Backup Vault│    │             │    │ └──────────┘ │      │
│  └────────────┘ └────────────┘    │             │    └──────────────┘      │
│                                   │             │                          │
│  ┌─────────────────── Network Topology ──────────────────────────────┐     │
│  │                                                                    │     │
│  │  Region: primary (e.g. us-east-1)    Region: DR (e.g. us-west-2) │     │
│  │  ┌──────────────────────┐            ┌──────────────────────┐     │     │
│  │  │ VPC (prod)           │            │ VPC (DR)             │     │     │
│  │  │ ┌───────┐ ┌────────┐ │            │ ┌───────┐ ┌────────┐│     │     │
│  │  │ │Public │ │Private │ │◄──TGW────►│ │Public │ │Private ││     │     │
│  │  │ │Subnet │ │Subnet  │ │            │ │Subnet │ │Subnet  ││     │     │
│  │  │ │(AZ-a) │ │(AZ-a)  │ │            │ │(AZ-a) │ │(AZ-a)  ││     │     │
│  │  │ ├───────┤ ├────────┤ │            │ ├───────┤ ├────────┤│     │     │
│  │  │ │Public │ │Private │ │            │ │Public │ │Private ││     │     │
│  │  │ │Subnet │ │Subnet  │ │            │ │Subnet │ │Subnet  ││     │     │
│  │  │ │(AZ-b) │ │(AZ-b)  │ │            │ │(AZ-b) │ │(AZ-b)  ││     │     │
│  │  │ ├───────┤ ├────────┤ │            │ ├───────┤ ├────────┤│     │     │
│  │  │ │Public │ │Private │ │            │ │Public │ │Private ││     │     │
│  │  │ │Subnet │ │Subnet  │ │            │ │Subnet │ │Subnet  ││     │     │
│  │  │ │(AZ-c) │ │(AZ-c)  │ │            │ │(AZ-c) │ │(AZ-c)  ││     │     │
│  │  │ └───────┘ └────────┘ │            │ └───────┘ └────────┘│     │     │
│  │  └──────────────────────┘            └──────────────────────┘     │     │
│  │                                                                    │     │
│  └────────────────────────────────────────────────────────────────────┘     │
│                                                                             │
│  ┌─────────────────── CI/CD Pipeline ────────────────────────────────┐     │
│  │  GitHub Actions ──► Build ──► SAST/SCA ──► SBOM ──► Sign ──►     │     │
│  │  ──► ECR Push ──► Admission Control ──► EKS Deploy               │     │
│  └────────────────────────────────────────────────────────────────────┘     │
│                                                                             │
│  ┌─────────────────── Backup & DR ───────────────────────────────────┐     │
│  │  AWS Backup Vault (Locked) ──► Cross-Account Replication ──►      │     │
│  │  ──► Cross-Region Copy ──► Immutable Retention ──► Tested Restore │     │
│  └────────────────────────────────────────────────────────────────────┘     │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Key Design Principles

1. **Account-per-environment isolation**: Each environment (dev, staging, prod) gets its own AWS account. Security tooling lives in a dedicated security account. Shared services (CI/CD, DNS, ECR) live in a shared services account. The management account does nothing except run Organizations and SSO.
2. **Least privilege everywhere**: Every IAM role, SCP, security group, and NACL starts with deny-all and opens the minimum required. CI/CD roles can only deploy to their target account. Developers cannot access prod directly.
3. **Multi-AZ by default**: Every stateful resource (RDS, EKS nodes, ElastiCache) spans at least 2 AZs in the primary region. Critical workloads span 3 AZs.
4. **Immutable infrastructure**: No SSH into instances. All changes go through IaC (Pulumi) and CI/CD. EKS nodes are managed node groups with automatic replacement.
5. **Defense in depth**: Security is layered — SCPs at the org level, IAM policies at the account level, security groups at the VPC level, network policies at the Kubernetes level, and admission control at the workload level.
6. **Software supply chain integrity**: All container images are scanned, signed, and tracked with SBOMs. Only signed images from the internal ECR are admitted to EKS. Dependencies are pinned and audited (aligned with NIST SP 800-218 SSDF and DoD DevSecOps Fundamentals).
7. **Ransomware-resilient backups**: Backups use AWS Backup Vault Lock (compliance mode), cross-account replication to the security account, and cross-region copies. Restore procedures are tested quarterly.
8. **IaC is the source of truth**: Pulumi manages all infrastructure. No ClickOps. Pulumi state is stored in Pulumi Cloud (or S3 + DynamoDB with encryption). All changes go through PR review.

### What to Keep

- Any existing DNS registrar configuration (we will delegate zones, not transfer domains)
- Any existing GitHub repositories and team structure
- Developer laptop setups (we add AWS SSO CLI profiles, not replace workflows)

### What to Change

- **AWS Organization** — create from scratch with OU hierarchy
- **AWS Accounts** — provision security, shared-services, dev, staging, prod accounts
- **Networking** — create VPCs, subnets, Transit Gateway, NACLs, security groups
- **DNS** — create Route 53 hosted zones (public and private), delegate from registrar
- **IAM / SSO** — configure AWS IAM Identity Center, permission sets, MFA
- **Security tooling** — enable CloudTrail, GuardDuty, Security Hub, Config, Detective
- **CI/CD** — GitHub Actions with OIDC federation, ECR, image scanning, SBOM
- **EKS** — multi-AZ cluster with managed node groups, network policies, admission control
- **Backups** — AWS Backup with vault lock, cross-account/cross-region replication

### Global Red Lines

These are forbidden unless explicitly overridden inside a milestone.

- No unrelated refactors
- No new dependencies without explicit milestone approval
- No schema migrations
- No config key renames
- No public API/event/route renames
- No production placeholders
- No silent error swallowing
- No secrets in source control
- No IAM policies with `Action: "*"` and `Resource: "*"`
- No security groups with `0.0.0.0/0` ingress on non-443/80 ports
- No unencrypted storage (S3, EBS, RDS, backups)
- No resources deployed outside designated regions without explicit approval

---

## Pulumi Project Structure

```
infra/
├── package.json                  # Shared dependencies
├── tsconfig.json
├── Pulumi.yaml                   # Monorepo workspace
├── stacks/
│   ├── 01-org/                   # AWS Organization, OUs, accounts
│   │   ├── Pulumi.yaml
│   │   ├── Pulumi.prod.yaml
│   │   ├── index.ts
│   │   └── __tests__/
│   ├── 02-identity/              # SSO, permission sets, MFA
│   │   ├── Pulumi.yaml
│   │   ├── index.ts
│   │   └── __tests__/
│   ├── 03-network/               # VPCs, subnets, TGW, NACLs
│   │   ├── Pulumi.yaml
│   │   ├── Pulumi.dev.yaml
│   │   ├── Pulumi.staging.yaml
│   │   ├── Pulumi.prod.yaml
│   │   ├── index.ts
│   │   └── __tests__/
│   ├── 04-dns/                   # Route 53 public + private zones
│   │   ├── Pulumi.yaml
│   │   ├── index.ts
│   │   └── __tests__/
│   ├── 05-security/              # CloudTrail, GuardDuty, SecHub, Config
│   │   ├── Pulumi.yaml
│   │   ├── index.ts
│   │   └── __tests__/
│   ├── 06-cicd/                  # ECR, OIDC provider, IAM roles
│   │   ├── Pulumi.yaml
│   │   ├── index.ts
│   │   └── __tests__/
│   ├── 07-eks/                   # EKS cluster, node groups, addons
│   │   ├── Pulumi.yaml
│   │   ├── Pulumi.dev.yaml
│   │   ├── Pulumi.staging.yaml
│   │   ├── Pulumi.prod.yaml
│   │   ├── index.ts
│   │   └── __tests__/
│   └── 08-backup/                # AWS Backup vaults, plans, replication
│       ├── Pulumi.yaml
│       ├── index.ts
│       └── __tests__/
├── lib/                          # Shared Pulumi component resources
│   ├── vpc.ts
│   ├── eks-cluster.ts
│   ├── backup-vault.ts
│   └── tagging.ts
├── test/
│   └── e2e/                      # E2E validation scripts (AWS SDK)
│       ├── org.e2e.test.ts
│       ├── network.e2e.test.ts
│       ├── security.e2e.test.ts
│       ├── eks.e2e.test.ts
│       └── backup.e2e.test.ts
└── .github/
    └── workflows/
        ├── infra-preview.yml     # PR preview
        └── infra-deploy.yml      # Merge deploy
```

---

## BDD and Runtime Validation Rules

Every milestone follows these rules.

### Write Tests Before Production Code

For each milestone:
1. Read the BDD acceptance table.
2. Create the test file(s) first.
3. Confirm the tests fail for the expected reason.
4. Write production code to make the tests pass.
5. Re-run tests after any refactor.

### Required Test Coverage Categories

Every milestone must explicitly cover the categories that apply:

- happy path (resource created with correct configuration)
- invalid input (bad CIDR, missing required tags, invalid account email)
- empty state / first-run state (stack with no prior resources)
- dependency failure / partial failure (cross-account assume-role failure, API throttling)
- backward compatibility behavior (existing resources not mutated)

### Scenario Structure

Every BDD scenario uses Given/When/Then:

```typescript
it("descriptive test name", () => {
  // Given: [precondition]
  // When: [action]
  // Then: [expected outcome]
});
```

### Test File Naming

| Layer | Convention | Location |
|---|---|---|
| Pulumi unit tests | `*.test.ts` | `stacks/<stack>/__tests__/` |
| Shared library tests | `*.test.ts` | `lib/__tests__/` |
| E2E runtime validation | `*.e2e.test.ts` | `test/e2e/` |

### End-to-End Runtime Validation

Every milestone must include E2E tests that go beyond `pulumi preview` and verify that deployed resources are correctly configured. These tests prove:

1. Resources exist and are reachable
2. Security configurations are enforced (encryption, access policies, security groups)
3. Cross-account and cross-region connectivity works
4. Failing states are handled safely

---

## Dependency, Migration, and Refactor Policy

### Dependency policy

A new dependency is allowed only if the milestone explicitly includes:

- package name and version
- why existing dependencies are insufficient
- security and maintenance rationale
- tests covering the new integration

### Migration policy

Any change to deployed resources that would cause replacement (e.g., renaming an S3 bucket, changing a VPC CIDR) requires:

- migration plan with `pulumi preview` diff
- backward compatibility strategy
- rollback strategy
- tested restore procedure

### Refactor budget

Each milestone must state one of the following:

- `No refactor permitted beyond direct implementation`
- `Minimal local refactor permitted in listed files only`
- `Targeted refactor permitted for [specific reason]`

---

## Evidence Log Template

Copy this table into each milestone section and fill it in during execution.

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `npm test` | all pre-existing tests green | | | |
| BDD tests created | `[files]` | compile or fail for expected reason | | | |
| E2E stubs created | `[files]` | compile or fail for expected reason | | | |
| Implementation | `[summary]` | contract satisfied | | | |
| Full tests | `npm test` | green | | | |
| E2E runtime | `npm run test:e2e` | green | | | |
| Pulumi preview | `pulumi preview` | clean preview | | | |
| Pulumi deploy | `pulumi up` | deploys cleanly | | | |
| Smoke tests | `[steps]` | all checked | | | |
| Compatibility checks | `[checks]` | no regressions | | | |

---

## Self-Review Gate

Before marking a milestone done, answer every question.

- Did I change only allowed files?
- Did I avoid unrelated refactors?
- Did I preserve all listed public interfaces and compatibility requirements?
- Did I add tests for failure modes, not just happy paths?
- Did I remove temporary debug code, mocks, placeholders, and commented-out dead code?
- Did I update documentation to match the implementation?
- Is every IAM policy least-privilege and justified?
- Is every security group restricted to the minimum required ports and sources?
- Is every S3 bucket, EBS volume, and database encrypted?
- Is every assumption either verified or explicitly documented as unresolved?
- Is the milestone truly done according to its Definition of Done?

If any answer is "no", the milestone is not complete.

---

## Lessons-Learned File Template

Path: `docs/lessons/aws-org-m<N>.md`

```md
# Lessons Learned — aws-org Milestone <N>

## What changed
- [summary]

## Design decisions and why
- [decision] — [reason]

## Mistakes made
- [mistake]

## Root causes
- [root cause]

## What was harder than expected
- [note]

## Naming conventions established
- [resources, stacks, tags]

## Test patterns that worked well
- [pattern]

## Missing tests that should exist now
- [test]

## Rules for the next milestone
- [rule]

## Template improvements suggested
- [improvement]
```

---

## Completion Summary Template

Path: `docs/completion/aws-org-m<N>.md`

```md
# Completion Summary — aws-org Milestone <N>

## Goal completed
- [what capability now exists]

## Files changed
- [file]

## Tests added
- [test file]

## Runtime validations added
- [e2e file]

## Compatibility checks performed
- [check]

## Documentation updated
- [doc and section]

## Deferred follow-ups
- [follow-up]

## Known non-blocking limitations
- [limitation]
```

---

## Milestone Plan

---

### Milestone 1 — AWS Organization & Account Structure

**Goal**: Create the AWS Organization, define the OU hierarchy, and provision the five core accounts (Security, Shared Services, Dev, Staging, Prod) using Pulumi, establishing the foundational account isolation that all subsequent milestones build upon.

**Context**: Today there is a single AWS root account (or nothing at all). All resources, billing, and credentials are co-mingled. This milestone creates the multi-account structure that is the bedrock of AWS security best practices (ref: AWS Well-Architected Framework, Security Pillar). The management account will do nothing except run Organizations and SSO. All workloads will live in member accounts.

**Important design rule**: The management account must never host workloads. It exists only for Organizations, SSO, and consolidated billing. Every workload account must be in an OU with appropriate SCPs from day one.

**Refactor budget**: `No refactor permitted beyond direct implementation`

#### Contract Block

| Field | Value |
|---|---|
| Inputs | AWS root account credentials (bootstrap only), desired account emails, region |
| Outputs | Organization ID, OU IDs, account IDs for all 5 member accounts, Pulumi stack outputs |
| Interfaces touched | AWS Organizations API, Pulumi state |
| Files allowed to change | `infra/stacks/01-org/index.ts`, `infra/stacks/01-org/Pulumi.yaml`, `infra/stacks/01-org/Pulumi.prod.yaml` |
| Files to read before changing anything | Pulumi AWS Organizations provider docs |
| New files allowed | `infra/stacks/01-org/__tests__/org.test.ts`, `infra/test/e2e/org.e2e.test.ts`, `infra/package.json`, `infra/tsconfig.json`, `infra/Pulumi.yaml`, `infra/lib/tagging.ts` |
| New dependencies allowed | `@pulumi/pulumi`, `@pulumi/aws`, `@pulumi/awsx`, `vitest` (or `jest`), `@aws-sdk/client-organizations`, `@aws-sdk/client-sts` |
| Migration allowed | `no` (greenfield) |
| Compatibility commitments | N/A — greenfield |
| Forbidden shortcuts | Hard-coded account IDs in source, root account API keys in state, `AdministratorAccess` policies on member accounts |

#### Out of Scope / Must Not Do

- Do not configure SSO or IAM roles (Milestone 2)
- Do not create VPCs or networking (Milestone 3)
- Do not enable security services (Milestone 4)
- Do not create more than the 5 listed accounts

#### Pre-Flight

1. Complete the Global Entry Rules.
2. Ensure you have management account root credentials (for initial bootstrap only).
3. Configure Pulumi AWS provider: `export AWS_PROFILE=management` or set env vars per [Pulumi AWS configuration](https://www.pulumi.com/docs/iac/get-started/aws/configure/).
4. Verify access: `aws sts get-caller-identity` — must show management account.
5. Copy the Evidence Log template.

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `infra/package.json` | NEW: Node project with Pulumi and AWS SDK dependencies |
| `infra/tsconfig.json` | NEW: TypeScript configuration |
| `infra/Pulumi.yaml` | NEW: Workspace-level Pulumi config |
| `infra/lib/tagging.ts` | NEW: Standard tagging function for all resources |
| `infra/stacks/01-org/Pulumi.yaml` | NEW: Stack definition for organization |
| `infra/stacks/01-org/Pulumi.prod.yaml` | NEW: Config values (account emails, region) |
| `infra/stacks/01-org/index.ts` | NEW: Organization, OUs, and accounts |
| `infra/stacks/01-org/__tests__/org.test.ts` | NEW: Unit tests |
| `infra/test/e2e/org.e2e.test.ts` | NEW: E2E validation |

#### Step-by-Step

1. Initialize the `infra/` project with `package.json`, `tsconfig.json`, and dependencies.
2. Create `infra/lib/tagging.ts` — a helper that applies standard tags (`Project`, `Environment`, `ManagedBy: pulumi`, `CostCenter`) to every resource.
3. Write unit tests in `infra/stacks/01-org/__tests__/org.test.ts` for:
   - Organization is created with `ALL` features enabled
   - Five OUs are created: `Security`, `SharedServices`, `Workloads/Dev`, `Workloads/Staging`, `Workloads/Prod`
   - Five accounts are created with correct names and emails, placed in correct OUs
   - All resources have standard tags
4. Write E2E test stubs in `infra/test/e2e/org.e2e.test.ts`.
5. Implement `infra/stacks/01-org/index.ts`:
   - Create `aws.organizations.Organization` with all features and all policy types enabled (SCP, Tag Policy, Backup Policy)
   - Create OUs: `Security`, `SharedServices`, `Workloads` (parent), `Workloads/Dev`, `Workloads/Staging`, `Workloads/Prod`
   - Create accounts: `startup-security`, `startup-shared`, `startup-dev`, `startup-staging`, `startup-prod`
   - Export: org ID, OU IDs, account IDs, account ARNs
6. Run unit tests — all pass.
7. Run `pulumi preview --stack org-prod` — clean preview, no errors.
8. Deploy: `pulumi up --stack org-prod --yes`.
9. Run E2E tests — verify all accounts exist and are in correct OUs.

#### BDD Acceptance Scenarios

**Feature: AWS Organization and Account Structure**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Organization created | happy path | No org exists | Pulumi deploys 01-org stack | Organization exists with ALL features, SCP + tag + backup policies enabled |
| OUs in correct hierarchy | happy path | Organization exists | OUs are queried | Security, SharedServices, Workloads, Workloads/Dev, Workloads/Staging, Workloads/Prod all exist |
| Accounts in correct OUs | happy path | OUs exist | Accounts are queried | Each account is in its designated OU |
| Account emails are unique | invalid input | Two accounts share an email | Pulumi runs | Error is surfaced, deployment fails cleanly |
| Duplicate deploy is idempotent | empty state | Stack already deployed | `pulumi up` runs again | No changes detected |
| Standard tags applied | happy path | Resources deployed | Tags are queried | All resources have Project, Environment, ManagedBy tags |

#### Regression Tests

- N/A — greenfield, no pre-existing resources

#### Compatibility Checklist

- [ ] N/A — greenfield

#### E2E Runtime Validation

**File**: `infra/test/e2e/org.e2e.test.ts`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `org_exists_with_all_features` | Organization is real and configured | `describeOrganization` returns `ALL` features, policy types include SCP |
| `ous_match_expected_hierarchy` | OUs are real and correctly nested | `listOrganizationalUnitsForParent` returns expected OU tree |
| `accounts_in_correct_ous` | Accounts exist and are in the right OU | `listAccountsForParent` returns correct accounts per OU |
| `accounts_have_standard_tags` | Tagging convention is enforced | All accounts have required tags |

#### Smoke Tests

- [ ] `aws organizations describe-organization` returns org with ALL features
- [ ] `aws organizations list-accounts` shows 5 member accounts + management account
- [ ] `aws organizations list-organizational-units-for-parent` shows correct OU tree
- [ ] Each account can be assumed into via `aws sts assume-role`
- [ ] `pulumi stack output` shows all account IDs and OU IDs

#### Evidence Log

| Step | Command / Check | Expected Result | Actual Result | Pass/Fail | Notes |
|---|---|---|---|---|---|
| Baseline tests | `npm test` | green (or no tests yet) | | | |
| BDD tests created | `org.test.ts` | fail for expected reason | | | |
| E2E stubs created | `org.e2e.test.ts` | fail for expected reason | | | |
| Implementation | `01-org/index.ts` | contract satisfied | | | |
| Full tests | `npm test` | green | | | |
| E2E runtime | `npm run test:e2e -- org` | green | | | |
| Pulumi preview | `pulumi preview --stack org-prod` | clean | | | |
| Pulumi deploy | `pulumi up --stack org-prod` | deploys cleanly | | | |
| Smoke tests | manual checks | all checked | | | |

#### Definition of Done

The milestone is done only when all of the following are true:

- Organization exists with ALL features enabled
- All 5 OUs exist in the correct hierarchy
- All 5 member accounts exist in the correct OUs
- All resources are tagged with standard tags
- Unit tests pass
- E2E runtime validation passes
- Smoke tests are checked off
- No secrets or hard-coded account IDs in source
- Lessons file is written
- Completion summary is written
- Milestone Tracker is updated

#### Post-Flight

Complete the Global Exit Rules above. Key documentation updates:

- **ARCHITECTURE.md**: Add "AWS Organization Structure" section with account IDs and OU hierarchy
- **README.md**: Add "Infrastructure" section with Pulumi setup instructions

#### Notes

- Account creation can take several minutes per account. Budget time accordingly.
- Account emails must be unique and receivable — use `+` aliasing (e.g., `infra+security@startup.com`).
- The management account should never be used for workloads. Install a guardrail SCP in Milestone 2.

---

### Milestone 2 — Identity, Governance & Guardrails

**Goal**: Configure AWS IAM Identity Center (SSO) with MFA enforcement, create least-privilege permission sets, deploy Service Control Policies (SCPs) on all OUs, and establish the governance foundation that prevents account misuse.

**Context**: Milestone 1 created the account structure but with no access controls. Currently, only the management account root user can access anything. This milestone establishes who can do what and hard guardrails that cannot be overridden by any individual account. SCPs are the most powerful preventive control in AWS — they apply org-wide and cannot be bypassed by account-level admin.

**Important design rule**: SCPs should deny dangerous actions (deny-list approach for critical actions like leaving the org, disabling CloudTrail, or creating resources outside allowed regions) AND permission sets should grant only what is needed (allow-list approach for day-to-day access). MFA must be enforced for all human access.

**Refactor budget**: `No refactor permitted beyond direct implementation`

#### Contract Block

| Field | Value |
|---|---|
| Inputs | Organization and account IDs from Milestone 1, SSO instance, user/group definitions |
| Outputs | SSO portal URL, permission set ARNs, SCP IDs, ability for developers to assume roles in member accounts via SSO |
| Interfaces touched | AWS IAM Identity Center, AWS Organizations SCPs, Pulumi state |
| Files allowed to change | `infra/stacks/02-identity/index.ts`, `infra/stacks/02-identity/Pulumi.yaml`, `infra/stacks/02-identity/Pulumi.prod.yaml` |
| Files to read before changing anything | Milestone 1 outputs, AWS SCP documentation |
| New files allowed | `infra/stacks/02-identity/__tests__/identity.test.ts`, `infra/stacks/02-identity/scps/`, E2E test file |
| New dependencies allowed | `@aws-sdk/client-sso-admin`, `@aws-sdk/client-identitystore` |
| Migration allowed | `no` |
| Compatibility commitments | Organization and account structure from Milestone 1 unchanged |
| Forbidden shortcuts | `AdministratorAccess` permission set for developers, SCP that allows all actions, disabled MFA |

#### Out of Scope / Must Not Do

- Do not modify the Organization or account structure
- Do not create VPCs or networking
- Do not enable security services beyond SCPs
- Do not create service-linked roles for EKS or other services (later milestones)

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `infra/stacks/02-identity/Pulumi.yaml` | NEW: Stack definition |
| `infra/stacks/02-identity/Pulumi.prod.yaml` | NEW: Config (group names, MFA settings) |
| `infra/stacks/02-identity/index.ts` | NEW: SSO, permission sets, SCPs |
| `infra/stacks/02-identity/scps/deny-leave-org.json` | NEW: SCP JSON |
| `infra/stacks/02-identity/scps/deny-root-actions.json` | NEW: SCP JSON |
| `infra/stacks/02-identity/scps/deny-outside-regions.json` | NEW: SCP JSON |
| `infra/stacks/02-identity/scps/deny-disable-security.json` | NEW: SCP JSON |
| `infra/stacks/02-identity/scps/deny-unencrypted-storage.json` | NEW: SCP JSON |
| `infra/stacks/02-identity/__tests__/identity.test.ts` | NEW: Unit tests |
| `infra/test/e2e/identity.e2e.test.ts` | NEW: E2E tests |

#### Step-by-Step

1. Write unit tests for all SCPs, permission sets, and SSO configuration.
2. Write E2E stubs.
3. Define SCPs as JSON files:
   - **deny-leave-org**: Prevents any account from leaving the organization.
   - **deny-root-actions**: Blocks root user actions in member accounts (except break-glass).
   - **deny-outside-regions**: Restricts resource creation to allowed regions only (e.g., `us-east-1`, `us-west-2`). Global services (IAM, CloudFront, Route53) are exempted.
   - **deny-disable-security**: Prevents disabling CloudTrail, GuardDuty, Config, Security Hub, or modifying their configurations.
   - **deny-unencrypted-storage**: Blocks creation of unencrypted S3 buckets, EBS volumes, and RDS instances.
4. Implement SSO configuration:
   - Enable IAM Identity Center (if not already enabled)
   - Create groups: `Admins`, `Developers`, `ReadOnly`, `SecurityAuditors`
   - Create permission sets:
     - `AdminAccess` — full admin, attached to `Admins` group on Shared Services account only
     - `DeveloperAccess` — power user minus IAM/Org/billing, attached to `Developers` group on Dev account
     - `ReadOnlyAccess` — read-only, attached to `ReadOnly` group on all workload accounts
     - `SecurityAudit` — SecurityAudit managed policy, attached to `SecurityAuditors` group on Security account
     - `ProdDeployAccess` — scoped deployment role, attached to CI/CD only (no human group), on Prod account
   - Enforce MFA for all permission sets via session policy
5. Attach SCPs to OUs:
   - Root OU: `deny-leave-org`, `deny-root-actions`
   - All OUs: `deny-outside-regions`, `deny-disable-security`, `deny-unencrypted-storage`
6. Run unit tests.
7. Deploy and validate.

#### BDD Acceptance Scenarios

**Feature: Identity and Governance**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| SSO portal accessible | happy path | Identity stack deployed | User navigates to SSO portal URL | SSO login page renders, MFA is required |
| Developer can access dev | happy path | Developer SSO login | Assumes DeveloperAccess in dev account | Session is active with power-user permissions |
| Developer cannot access prod | access control | Developer SSO login | Attempts to assume any role in prod | Access denied |
| SCP blocks region violation | guardrail | SCP deployed | Account attempts `ec2:RunInstances` in `eu-west-1` | Action denied by SCP |
| SCP blocks CloudTrail disable | guardrail | SCP deployed | Account attempts `cloudtrail:StopLogging` | Action denied by SCP |
| SCP blocks unencrypted S3 | guardrail | SCP deployed | Account attempts `s3:CreateBucket` without encryption | Action denied by SCP |
| SCP blocks org leave | guardrail | SCP deployed | Account attempts `organizations:LeaveOrganization` | Action denied by SCP |
| Root user blocked in member | guardrail | SCP deployed | Root user attempts action in member account | Action denied by SCP |
| MFA required for SSO | security | SSO configured | User attempts login without MFA | Access denied |

#### E2E Runtime Validation

**File**: `infra/test/e2e/identity.e2e.test.ts`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `sso_instance_exists` | SSO is enabled | `listInstances` returns an instance |
| `permission_sets_exist` | All permission sets are created | Expected permission sets found |
| `scps_attached_to_ous` | SCPs are enforced | `listPoliciesForTarget` returns expected SCPs per OU |
| `region_restriction_enforced` | Cannot create resources in blocked region | API call to blocked region fails with access denied |

#### Smoke Tests

- [ ] SSO portal URL is accessible
- [ ] Developer can log in via SSO with MFA and access dev account
- [ ] Developer cannot access prod account via SSO
- [ ] `aws ec2 run-instances --region eu-west-1` fails with SCP denial in any workload account
- [ ] `aws cloudtrail stop-logging` fails in any workload account
- [ ] `pulumi stack output` shows SSO portal URL and permission set ARNs

#### Definition of Done

- SSO enabled with MFA enforcement
- 4 groups and 5 permission sets created
- 5 SCPs deployed and attached to correct OUs
- Developers have access to dev but not prod
- All guardrails verified via E2E tests
- Smoke tests checked off
- No `AdministratorAccess` for developer groups
- Lessons file written, Milestone Tracker updated

---

### Milestone 3 — Network Architecture — VPCs, Zones & DNS

**Goal**: Deploy the complete network topology — VPCs with public/private/isolated subnets across 3 AZs per environment, Transit Gateway for cross-VPC connectivity, NACLs for subnet-level firewalling, NAT Gateways for private subnet egress, and Route 53 hosted zones for both public and private DNS — establishing the network foundation for all workloads.

**Context**: Accounts exist (M1) and are governed (M2), but have no networking. This milestone creates the network layer that all services will use. The design follows AWS best practices: public subnets for ALBs/NLBs only, private subnets for compute (EKS nodes, RDS), isolated subnets for databases with no internet route. Transit Gateway connects VPCs cross-account. Private hosted zones enable internal service discovery without leaking DNS to the internet.

**Important design rule**: CIDR blocks must be non-overlapping and sized for growth. Use a structured allocation scheme (e.g., `10.0.0.0/16` for dev, `10.1.0.0/16` for staging, `10.2.0.0/16` for prod, `10.3.0.0/16` for shared services). Each VPC gets `/16`, each subnet gets `/20` (4,091 IPs per subnet — plenty for a startup with room to grow).

**Refactor budget**: `No refactor permitted beyond direct implementation`

#### Contract Block

| Field | Value |
|---|---|
| Inputs | Account IDs from M1, CIDR allocation plan, region, AZ list, domain name |
| Outputs | VPC IDs, subnet IDs, route table IDs, TGW ID, NAT Gateway IPs, Route 53 zone IDs, NS records for delegation |
| Interfaces touched | AWS VPC, Subnets, Route Tables, NACLs, Transit Gateway, NAT Gateway, Route 53, Pulumi state |
| Files allowed to change | `infra/stacks/03-network/index.ts`, `infra/stacks/04-dns/index.ts` |
| Files to read before changing anything | M1 outputs, CIDR plan |
| New files allowed | Stack configs, tests, `infra/lib/vpc.ts` component |
| New dependencies allowed | `@aws-sdk/client-ec2`, `@aws-sdk/client-route-53` |
| Migration allowed | `no` |
| Compatibility commitments | Org and identity from M1/M2 unchanged |
| Forbidden shortcuts | `0.0.0.0/0` ingress on any security group, overlapping CIDRs, public subnets with auto-assign public IP, DNS zones without DNSSEC |

#### Out of Scope / Must Not Do

- Do not deploy any compute resources (EC2, EKS, RDS)
- Do not configure VPN or Direct Connect
- Do not create application-specific security groups (those come with EKS/workloads)

#### CIDR Allocation Plan

| Account | VPC CIDR | Public Subnets | Private Subnets | Isolated Subnets |
|---|---|---|---|---|
| Dev | `10.0.0.0/16` | `10.0.0.0/20`, `10.0.16.0/20`, `10.0.32.0/20` | `10.0.48.0/20`, `10.0.64.0/20`, `10.0.80.0/20` | `10.0.96.0/20`, `10.0.112.0/20`, `10.0.128.0/20` |
| Staging | `10.1.0.0/16` | `10.1.0.0/20`, `10.1.16.0/20`, `10.1.32.0/20` | `10.1.48.0/20`, `10.1.64.0/20`, `10.1.80.0/20` | `10.1.96.0/20`, `10.1.112.0/20`, `10.1.128.0/20` |
| Prod | `10.2.0.0/16` | `10.2.0.0/20`, `10.2.16.0/20`, `10.2.32.0/20` | `10.2.48.0/20`, `10.2.64.0/20`, `10.2.80.0/20` | `10.2.96.0/20`, `10.2.112.0/20`, `10.2.128.0/20` |
| Shared | `10.3.0.0/16` | `10.3.0.0/20`, `10.3.16.0/20`, `10.3.32.0/20` | `10.3.48.0/20`, `10.3.64.0/20`, `10.3.80.0/20` | — |
| DR (Prod) | `10.4.0.0/16` | `10.4.0.0/20`, `10.4.16.0/20`, `10.4.32.0/20` | `10.4.48.0/20`, `10.4.64.0/20`, `10.4.80.0/20` | `10.4.96.0/20`, `10.4.112.0/20`, `10.4.128.0/20` |

#### DNS Architecture

| Zone | Type | Account | Purpose |
|---|---|---|---|
| `startup.com` | Public | Shared Services | External-facing DNS (web, API) |
| `internal.startup.com` | Private (VPC-associated) | Shared Services | Internal service discovery |
| `dev.internal.startup.com` | Private | Dev | Dev-specific internal DNS |
| `staging.internal.startup.com` | Private | Staging | Staging-specific internal DNS |
| `prod.internal.startup.com` | Private | Prod | Prod-specific internal DNS |

#### BDD Acceptance Scenarios

**Feature: Network Architecture**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| VPCs created per account | happy path | Network stack deployed | VPCs queried | 4 VPCs exist with correct CIDRs (dev, staging, prod, shared) |
| Subnets span 3 AZs | happy path | VPC exists | Subnets queried | Each VPC has 3 public, 3 private, 3 isolated subnets across AZs a, b, c |
| Private subnets have NAT route | happy path | Subnets exist | Route tables queried | Private subnet route tables have `0.0.0.0/0` via NAT GW |
| Isolated subnets have no internet | happy path | Subnets exist | Route tables queried | Isolated subnet route tables have NO `0.0.0.0/0` route |
| TGW connects all VPCs | happy path | TGW deployed | TGW attachments queried | All VPCs are attached to TGW |
| Cross-VPC routing works | happy path | TGW routes propagated | Shared VPC pings prod VPC CIDR | Route exists (actual ICMP test in E2E) |
| NACLs restrict traffic | security | NACLs deployed | NACL rules queried | Public subnets allow 80/443 inbound only; private subnets allow only VPC CIDR inbound |
| Public hosted zone exists | happy path | DNS stack deployed | Zone queried | `startup.com` public zone exists with NS records |
| Private hosted zone exists | happy path | DNS stack deployed | Zone queried | `internal.startup.com` private zone exists and is associated with all VPCs |
| DNSSEC enabled on public zone | security | Public zone exists | DNSSEC status queried | DNSSEC signing is enabled |
| No overlapping CIDRs | invalid input | CIDR plan defined | CIDRs compared | All VPC CIDRs are unique and non-overlapping |

#### E2E Runtime Validation

**File**: `infra/test/e2e/network.e2e.test.ts`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `vpcs_exist_with_correct_cidrs` | VPCs deployed correctly | `describeVpcs` returns expected CIDRs per account |
| `subnets_span_three_azs` | Multi-AZ deployment | Each VPC has subnets in 3 distinct AZs |
| `private_subnets_route_via_nat` | Private subnet egress works | Route table has NAT GW route |
| `isolated_subnets_no_internet` | DB subnets are air-gapped from internet | No `0.0.0.0/0` route in isolated subnet route tables |
| `tgw_attachments_exist` | Cross-VPC connectivity | TGW has attachments for all VPCs |
| `public_zone_has_dnssec` | DNS integrity | `getDnssec` returns SIGNING status |
| `private_zones_associated` | Internal DNS resolution | Private zones associated with correct VPCs |

#### Smoke Tests

- [ ] `aws ec2 describe-vpcs` in each account shows expected VPC
- [ ] `aws ec2 describe-subnets` shows 9 subnets per workload VPC (3 public + 3 private + 3 isolated)
- [ ] `aws ec2 describe-nat-gateways` shows NAT GW in each VPC's public subnets
- [ ] `aws ec2 describe-transit-gateway-attachments` shows all VPC attachments
- [ ] `aws route53 list-hosted-zones` shows public and private zones
- [ ] `aws route53 get-dnssec` on public zone shows signing enabled
- [ ] `nslookup internal.startup.com` from a VPC resolves correctly

#### Definition of Done

- 4 VPCs with correct CIDRs deployed across 4 accounts
- DR VPC in secondary region deployed
- 9 subnets per workload VPC spanning 3 AZs
- NAT Gateways in public subnets, routes in private subnets
- Isolated subnets with no internet route
- Transit Gateway connecting all VPCs
- NACLs restricting subnet-level traffic
- Public Route 53 zone with DNSSEC
- Private Route 53 zones associated with VPCs
- All unit and E2E tests pass
- Lessons file written, Milestone Tracker updated

---

### Milestone 4 — Security Baseline — Logging, Detection & Response

**Goal**: Deploy the full AWS security stack — CloudTrail (org-wide), GuardDuty (org-wide), Security Hub (org-wide), AWS Config (org-wide), VPC Flow Logs, and centralized log aggregation in the Security account — establishing the detection and audit layer.

**Context**: The organization has accounts (M1), governance (M2), and networking (M3), but no visibility into what is happening. Without logging and detection, compromises go unnoticed. This milestone deploys the "security cameras and alarms" layer. All logs flow to the Security account, where they are stored immutably and cannot be tampered with by workload accounts (enforced by the SCP from M2 that prevents disabling these services).

**Important design rule**: All security logs must be centralized in the Security account. Workload accounts must not be able to delete or modify security logs. CloudTrail must log all management events and S3 data events. Every log bucket must have object lock enabled.

**Refactor budget**: `No refactor permitted beyond direct implementation`

#### Contract Block

| Field | Value |
|---|---|
| Inputs | Account IDs, VPC IDs from M1/M3, Security account credentials |
| Outputs | CloudTrail trail ARN, GuardDuty detector IDs, Security Hub ARN, Config recorder ARNs, log bucket ARN |
| Interfaces touched | CloudTrail, GuardDuty, Security Hub, AWS Config, S3, VPC Flow Logs, KMS, Pulumi state |
| Files allowed to change | `infra/stacks/05-security/index.ts`, config files |
| New files allowed | Stack configs, tests, E2E tests |
| New dependencies allowed | `@aws-sdk/client-cloudtrail`, `@aws-sdk/client-guardduty`, `@aws-sdk/client-securityhub`, `@aws-sdk/client-config-service` |
| Migration allowed | `no` |
| Compatibility commitments | Org, identity, and network from M1-M3 unchanged |
| Forbidden shortcuts | Unencrypted log buckets, log buckets without object lock, CloudTrail without data events, disabled multi-region trails |

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `infra/stacks/05-security/Pulumi.yaml` | NEW: Stack definition |
| `infra/stacks/05-security/index.ts` | NEW: Security services deployment |
| `infra/stacks/05-security/__tests__/security.test.ts` | NEW: Unit tests |
| `infra/test/e2e/security.e2e.test.ts` | NEW: E2E tests |

#### Step-by-Step

1. Write unit tests first.
2. Write E2E stubs.
3. Implement in Security account:
   - **KMS key** for log encryption (Security account-owned, cross-account decrypt for CloudTrail/Config).
   - **S3 log bucket** with:
     - SSE-KMS encryption
     - Object Lock (compliance mode, 365-day retention)
     - Versioning enabled
     - Block all public access
     - Lifecycle policy: transition to Glacier after 90 days, delete after 7 years
     - Bucket policy: allow only CloudTrail, Config, VPC Flow Logs to write; deny all deletes
   - **CloudTrail** organization trail:
     - Multi-region
     - Log management events + S3 data events + Lambda data events
     - Log file validation enabled
     - KMS encryption
     - Deliver to Security account S3 bucket
     - CloudWatch Logs integration for real-time alerting
   - **GuardDuty** org-wide:
     - Delegated administrator: Security account
     - Auto-enable for all member accounts
     - S3 protection, EKS protection, malware protection enabled
   - **Security Hub** org-wide:
     - Delegated administrator: Security account
     - Auto-enable for all member accounts
     - Standards: AWS Foundational Security Best Practices, CIS AWS Benchmark
   - **AWS Config** org-wide:
     - Delegated administrator: Security account
     - Record all resources
     - Deliver to Security account S3 bucket
     - Conformance pack: Operational Best Practices for CIS
   - **VPC Flow Logs** for every VPC:
     - Deliver to Security account S3 bucket (cross-account)
     - ALL traffic (accept + reject)
     - Format: Parquet for Athena queries
4. Run tests and deploy.

#### BDD Acceptance Scenarios

**Feature: Security Baseline**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| CloudTrail org trail active | happy path | Security stack deployed | Trail status queried | Trail is logging, multi-region, encryption enabled |
| GuardDuty enabled all accounts | happy path | GuardDuty deployed | Detectors queried | Every account has an active detector |
| Security Hub standards enabled | happy path | Security Hub deployed | Standards queried | FSBP and CIS standards are enabled |
| Config recording all resources | happy path | Config deployed | Recorder queried | All resource types recorded |
| VPC Flow Logs active | happy path | Flow logs deployed | Flow log status queried | Every VPC has flow logs in ALL mode |
| Log bucket is immutable | security | Log bucket deployed | Object lock queried | Compliance mode with 365-day retention |
| Log bucket denies deletes | security | Bucket policy deployed | `s3:DeleteObject` attempted | Access denied |
| Workload account cannot disable trail | guardrail | SCP from M2 | `cloudtrail:StopLogging` attempted | Denied by SCP |
| Security Hub findings visible | detection | Resources deployed | Security Hub dashboard queried | Findings aggregated from all accounts |

#### E2E Runtime Validation

**File**: `infra/test/e2e/security.e2e.test.ts`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `cloudtrail_is_logging` | Org-wide audit trail active | `getTrailStatus` returns `IsLogging: true` |
| `guardduty_detectors_active` | Threat detection running | All accounts have enabled detectors |
| `security_hub_standards_enabled` | Compliance checking active | FSBP and CIS subscriptions exist |
| `config_recorders_active` | Configuration recording | All accounts have recording recorders |
| `log_bucket_immutable` | Logs cannot be tampered with | Object lock configuration present |
| `vpc_flow_logs_active` | Network traffic logged | Every VPC has active flow logs |

#### Smoke Tests

- [ ] `aws cloudtrail get-trail-status` shows logging active
- [ ] `aws guardduty list-detectors` in each account returns a detector
- [ ] `aws securityhub describe-hub` in Security account shows aggregated findings
- [ ] `aws configservice describe-configuration-recorders` shows active recorders
- [ ] `aws s3api get-object-lock-configuration` on log bucket shows compliance mode
- [ ] `aws ec2 describe-flow-logs` in each account shows active flow logs

#### Definition of Done

- Org-wide CloudTrail with encryption and data events
- Org-wide GuardDuty with all protection types
- Org-wide Security Hub with FSBP and CIS standards
- Org-wide Config with conformance pack
- VPC Flow Logs on every VPC
- Centralized immutable log bucket in Security account
- All tests pass
- Lessons file written, Milestone Tracker updated

---

### Milestone 5 — CI/CD Pipeline — Hardened & Least-Privilege

**Goal**: Deploy a hardened CI/CD pipeline using GitHub Actions with OIDC federation to AWS (no long-lived credentials), ECR repositories with image scanning and lifecycle policies, and IAM roles scoped per environment — establishing the secure deployment path.

**Context**: There is no CI/CD today. Developers deploy manually. This creates several risks: inconsistent deployments, credential exposure, no audit trail for changes, and no gate for security checks. This milestone creates the pipeline infrastructure. The pipeline itself will run in GitHub Actions, authenticated via OIDC (no static AWS keys). Each environment gets its own deployment role with permissions limited to that account's resources.

**Important design rule**: No long-lived AWS credentials in CI/CD. All access uses OIDC federation with short-lived tokens. The CI/CD role for prod has the minimum permissions needed to deploy to EKS and push to ECR. No role can access the Security or Management accounts.

**Refactor budget**: `No refactor permitted beyond direct implementation`

#### Contract Block

| Field | Value |
|---|---|
| Inputs | Account IDs from M1, GitHub org/repo names, ECR repository names |
| Outputs | OIDC provider ARN, CI/CD role ARNs per account, ECR repository URIs, GitHub Actions workflow files |
| Interfaces touched | IAM (OIDC provider, roles, policies), ECR, GitHub Actions, Pulumi state |
| Files allowed to change | `infra/stacks/06-cicd/index.ts`, `.github/workflows/` |
| New files allowed | Stack configs, tests, GitHub Actions workflows |
| New dependencies allowed | `@aws-sdk/client-ecr`, `@aws-sdk/client-iam` |
| Migration allowed | `no` |
| Compatibility commitments | All prior infrastructure unchanged |
| Forbidden shortcuts | Long-lived AWS access keys, `AdministratorAccess` on CI/CD roles, ECR repos without scanning, `*` resource in IAM policies |

#### Pipeline Architecture

```
Developer PR ──► GitHub Actions ──► Lint + Test + SAST ──► Pulumi Preview
                                           │
Merge to main ──► GitHub Actions ──► Build ──► Scan ──► SBOM ──► Sign ──►
                                           │
                      ┌────────────────────┼────────────────────┐
                      ▼                    ▼                    ▼
                ECR (shared)       Deploy to Dev         Deploy to Staging
                                           │                    │
                                    (auto on merge)     (manual approval)
                                                               │
                                                        Deploy to Prod
                                                        (manual approval)
```

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `infra/stacks/06-cicd/Pulumi.yaml` | NEW: Stack definition |
| `infra/stacks/06-cicd/index.ts` | NEW: OIDC provider, IAM roles, ECR repos |
| `infra/stacks/06-cicd/__tests__/cicd.test.ts` | NEW: Unit tests |
| `infra/test/e2e/cicd.e2e.test.ts` | NEW: E2E tests |
| `.github/workflows/infra-preview.yml` | NEW: PR preview workflow |
| `.github/workflows/infra-deploy.yml` | NEW: Deploy workflow |
| `.github/workflows/app-build-deploy.yml` | NEW: App build/deploy workflow |

#### Step-by-Step

1. Write tests first.
2. Deploy in Shared Services account:
   - **OIDC Identity Provider** for GitHub Actions (`token.actions.githubusercontent.com`).
   - **ECR Repositories** in Shared Services account:
     - One repo per microservice (start with a `app` repo)
     - Image scanning on push enabled (both basic and enhanced with Inspector)
     - Lifecycle policy: keep only last 30 tagged images, expire untagged after 7 days
     - Immutable tags enabled
     - Encryption with KMS
     - Cross-account pull permissions for Dev, Staging, Prod accounts
   - **CI/CD IAM Roles** (one per environment, in each target account):
     - Trust policy: GitHub OIDC provider, scoped to specific repo and branch
       - Dev role: trusts `refs/heads/main` and `refs/heads/develop`
       - Staging role: trusts `refs/heads/main` only
       - Prod role: trusts `refs/heads/main` only, requires environment protection
     - Permissions: limited to EKS deploy, ECR pull, specific S3 buckets, CloudWatch logs
     - Session duration: 1 hour maximum
   - **GitHub Actions Workflows**:
     - `infra-preview.yml`: On PR, run `pulumi preview` with diff
     - `infra-deploy.yml`: On merge to main, run `pulumi up` per stack in order
     - `app-build-deploy.yml`: Build Docker image, scan with Trivy, generate SBOM with Syft, push to ECR, deploy to EKS
3. Run tests and deploy.

#### BDD Acceptance Scenarios

**Feature: CI/CD Pipeline**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| OIDC provider configured | happy path | CI/CD stack deployed | OIDC provider queried | GitHub Actions OIDC provider exists with correct thumbprint |
| ECR repos exist with scanning | happy path | ECR deployed | Repo settings queried | Scanning on push enabled, immutable tags, KMS encryption |
| Dev role trusts correct branches | security | Dev role deployed | Trust policy inspected | Only `main` and `develop` branches accepted |
| Prod role requires main only | security | Prod role deployed | Trust policy inspected | Only `main` branch accepted, environment protection required |
| CI/CD role cannot access Security account | security | CI/CD roles deployed | Assume role attempted to Security | Access denied |
| No static credentials in workflow | security | Workflow files exist | Files inspected | No `AWS_ACCESS_KEY_ID` or `AWS_SECRET_ACCESS_KEY` in workflow files |
| ECR lifecycle cleans old images | maintenance | Images pushed | 31+ tagged images exist | Oldest images beyond 30 are expired |
| Cross-account ECR pull works | happy path | ECR policy deployed | Prod account pulls image | Image pulled successfully |

#### E2E Runtime Validation

**File**: `infra/test/e2e/cicd.e2e.test.ts`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `oidc_provider_exists` | GitHub can authenticate | OIDC provider with correct URL exists |
| `ecr_repos_have_scanning` | Images will be scanned | All repos have `scanOnPush: true` |
| `ecr_repos_have_immutable_tags` | Tags cannot be overwritten | `imageTagMutability: IMMUTABLE` |
| `cicd_roles_are_scoped` | Least privilege | Role policies don't contain `*` resources |
| `no_static_credentials_in_workflows` | No credential leakage | Workflow files contain no AWS key references |

#### Smoke Tests

- [ ] `aws iam list-open-id-connect-providers` shows GitHub OIDC provider
- [ ] `aws ecr describe-repositories` shows repos with scanning enabled
- [ ] GitHub Actions workflow triggers on PR and runs `pulumi preview`
- [ ] GitHub Actions can assume role in dev account via OIDC
- [ ] `aws sts get-caller-identity` inside GitHub Actions shows the CI/CD role

#### Definition of Done

- GitHub OIDC provider deployed
- ECR repositories with scanning, immutable tags, lifecycle policies
- Per-environment CI/CD roles with least-privilege, branch-scoped trust
- GitHub Actions workflows for infra preview, deploy, and app build
- No long-lived AWS credentials anywhere in CI/CD
- All tests pass
- Lessons file written, Milestone Tracker updated

---

### Milestone 6 — EKS Runtime Environment

**Goal**: Deploy a production-grade multi-AZ EKS cluster with managed node groups, Kubernetes network policies, pod security standards, IRSA (IAM Roles for Service Accounts), and cluster add-ons — establishing the runtime platform for workloads.

**Context**: We now have accounts (M1), governance (M2), networking (M3), security monitoring (M4), and CI/CD (M5). But there is nowhere to run workloads. This milestone deploys EKS in the Prod account (and optionally Dev/Staging) with a security-first configuration. EKS represents both the data plane (worker nodes) and the control plane (API server). Both must be secured.

**Important design rule**: The EKS API server endpoint must be private (accessible only within the VPC and via Transit Gateway). No public endpoint. Worker nodes run in private subnets only. All pod-to-pod communication is denied by default and allowed explicitly via Kubernetes NetworkPolicy. IRSA replaces node-level IAM roles for pod-level least privilege.

**Refactor budget**: `Minimal local refactor permitted in listed files only`

#### Contract Block

| Field | Value |
|---|---|
| Inputs | VPC IDs, private subnet IDs from M3, CI/CD role ARNs from M5 |
| Outputs | EKS cluster endpoint, cluster CA, OIDC issuer URL, node group ASG names, kubeconfig |
| Interfaces touched | EKS, EC2 (node groups), IAM (cluster role, node role, IRSA), KMS, Kubernetes API |
| Files allowed to change | `infra/stacks/07-eks/index.ts`, `infra/lib/eks-cluster.ts` |
| New files allowed | Stack configs, tests, E2E tests |
| New dependencies allowed | `@pulumi/eks`, `@pulumi/kubernetes`, `@aws-sdk/client-eks` |
| Migration allowed | `no` |
| Compatibility commitments | All prior infrastructure unchanged |
| Forbidden shortcuts | Public API server endpoint, nodes in public subnets, `cluster-admin` for workloads, node IAM roles with broad permissions |

#### EKS Architecture

```
┌─────────────────────────── Prod Account ──────────────────────────┐
│                                                                    │
│  ┌────────── EKS Control Plane (AWS Managed) ──────────────┐      │
│  │  API Server (private endpoint only)                      │      │
│  │  etcd (encrypted at rest with CMK)                       │      │
│  │  OIDC Issuer (for IRSA)                                  │      │
│  └──────────────────────────────────────────────────────────┘      │
│                              │                                     │
│  ┌──────────── Private Subnets (3 AZs) ───────────────────┐      │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐             │      │
│  │  │Node Group│  │Node Group│  │Node Group│             │      │
│  │  │  AZ-a    │  │  AZ-b    │  │  AZ-c    │             │      │
│  │  │(2 nodes) │  │(2 nodes) │  │(2 nodes) │             │      │
│  │  └──────────┘  └──────────┘  └──────────┘             │      │
│  │                                                        │      │
│  │  Network Policies: deny-all default, allow explicit    │      │
│  │  Pod Security: restricted profile enforced             │      │
│  │  IRSA: per-service IAM roles via ServiceAccount        │      │
│  └────────────────────────────────────────────────────────┘      │
│                                                                    │
│  Add-ons: CoreDNS, kube-proxy, VPC-CNI, EBS CSI,                │
│           AWS Load Balancer Controller, External DNS,             │
│           Cluster Autoscaler, Metrics Server                      │
└────────────────────────────────────────────────────────────────────┘
```

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `infra/lib/eks-cluster.ts` | NEW: Reusable EKS component resource |
| `infra/stacks/07-eks/Pulumi.yaml` | NEW: Stack definition |
| `infra/stacks/07-eks/Pulumi.prod.yaml` | NEW: Prod config (node sizes, counts) |
| `infra/stacks/07-eks/index.ts` | NEW: EKS cluster, node groups, add-ons |
| `infra/stacks/07-eks/__tests__/eks.test.ts` | NEW: Unit tests |
| `infra/test/e2e/eks.e2e.test.ts` | NEW: E2E tests |

#### Step-by-Step

1. Write unit tests first.
2. Write E2E stubs.
3. Create `infra/lib/eks-cluster.ts` — reusable component that creates:
   - EKS cluster with:
     - **Private API endpoint only** (no public endpoint)
     - Kubernetes version: latest stable (e.g., 1.29)
     - Envelope encryption with customer-managed KMS key
     - Logging: api, audit, authenticator, controllerManager, scheduler → CloudWatch
   - Managed node groups:
     - 3 node groups, one per AZ (for blast radius isolation)
     - Instance type: `m6i.large` (prod), `t3.medium` (dev)
     - Min 2, max 10 nodes per group (prod); min 1, max 3 (dev)
     - Amazon Linux 2023 AMI (latest patched)
     - Encrypted EBS root volumes with CMK
     - No SSH key pair — no SSH access (use SSM Session Manager if needed)
   - IAM:
     - Cluster role: `AmazonEKSClusterPolicy` only
     - Node role: `AmazonEKSWorkerNodePolicy`, `AmazonEKS_CNI_Policy`, `AmazonEC2ContainerRegistryReadOnly` only
     - IRSA OIDC provider configured
   - Security group:
     - Cluster SG: ingress from VPC CIDR only (port 443)
     - Node SG: ingress from cluster SG only (all ports), egress to VPC CIDR + HTTPS to internet (for ECR pulls)
4. Deploy cluster add-ons:
   - CoreDNS, kube-proxy, VPC-CNI (latest versions)
   - EBS CSI driver (for persistent volumes, with IRSA)
   - AWS Load Balancer Controller (with IRSA)
   - Cluster Autoscaler (with IRSA)
   - Metrics Server
5. Apply baseline Kubernetes security:
   - Default deny-all NetworkPolicy in every namespace
   - Pod Security Standards: `restricted` profile enforcement
   - Resource quotas per namespace
6. Run tests and deploy.

#### BDD Acceptance Scenarios

**Feature: EKS Runtime Environment**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Cluster created with private endpoint | happy path | EKS stack deployed | Cluster described | API endpoint is private, not public |
| Nodes span 3 AZs | happy path | Node groups deployed | Nodes listed | Nodes exist in AZ-a, AZ-b, AZ-c |
| Secrets are envelope-encrypted | security | Cluster deployed | Encryption config queried | KMS-based encryption configured |
| Control plane logging enabled | observability | Cluster deployed | Logging config queried | All 5 log types enabled |
| No SSH access to nodes | security | Node groups deployed | Key pair queried | No SSH key pair configured |
| Default deny network policy | security | NetworkPolicy applied | New pod deployed | Cannot communicate with other pods without explicit allow |
| Pod security restricted | security | PSS enforced | Pod with `privileged: true` deployed | Pod rejected |
| IRSA works | happy path | OIDC provider configured | ServiceAccount annotated with IAM role | Pod can assume IAM role |
| Cluster autoscaler scales | happy path | Load applied | Nodes counted | Additional nodes launched in overloaded AZ |

#### E2E Runtime Validation

**File**: `infra/test/e2e/eks.e2e.test.ts`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `cluster_endpoint_is_private` | No public API access | `endpointPublicAccess: false` |
| `nodes_in_three_azs` | Multi-AZ deployment | Nodes in 3 distinct AZs |
| `secrets_encrypted_with_kms` | Data at rest encrypted | Encryption config includes KMS key |
| `control_plane_logging_all` | Full audit trail | All 5 log types enabled |
| `no_public_node_ips` | Nodes not internet-accessible | No nodes have public IPs |
| `coredns_running` | DNS resolution works | CoreDNS pods are Running |
| `vpc_cni_running` | Networking works | VPC CNI pods are Running |

#### Smoke Tests

- [ ] `aws eks describe-cluster` shows private endpoint, encryption, logging
- [ ] `kubectl get nodes` shows nodes in 3 AZs, all Ready
- [ ] `kubectl run test --image=nginx` in a default-deny namespace — pod cannot reach other pods
- [ ] `kubectl apply` a pod with `privileged: true` — rejected by pod security
- [ ] `kubectl get pods -n kube-system` shows CoreDNS, kube-proxy, VPC CNI, EBS CSI running
- [ ] EKS API is NOT reachable from the public internet

#### Definition of Done

- EKS cluster with private endpoint only, envelope encryption, full logging
- 3 node groups across 3 AZs with encrypted EBS, no SSH
- IRSA configured
- All add-ons running
- Default deny NetworkPolicy enforced
- Pod Security Standards enforced
- All tests pass
- Lessons file written, Milestone Tracker updated

---

### Milestone 7 — Software Supply Chain Security

**Goal**: Implement end-to-end software supply chain security — container image scanning, SBOM generation, image signing with cosign/AWS Signer, Kubernetes admission control to reject unsigned images, dependency pinning, and SLSA provenance — aligned with NIST SP 800-218 (SSDF), NSA/CISA guidance, and DoD DevSecOps Fundamentals.

**Context**: CI/CD (M5) can build and push images, and EKS (M6) can run them. But there is no verification that the images running in production are the ones that were built, scanned, and approved. An attacker who compromises the registry or the pipeline could inject malicious code. This milestone closes that gap by creating a chain of trust from source code to running container.

**Important design rule**: No container image runs in EKS unless it (1) comes from the internal ECR, (2) has been scanned with zero critical/high vulnerabilities, (3) has an SBOM attached, and (4) has been signed. Admission control (Kyverno or OPA Gatekeeper) enforces this at the Kubernetes API level.

**Reference standards**:
- **NIST SP 800-218 (SSDF)**: Secure Software Development Framework — defines practices for producing well-secured software: PO (Prepare the Organization), PS (Protect the Software), PW (Produce Well-Secured Software), RV (Respond to Vulnerabilities).
- **NSA/CISA Securing the Software Supply Chain (Developer Guide, Sept 2022)**: Recommends SBOM, dependency management, build environment security, artifact signing, and provenance.
- **DoD Enterprise DevSecOps Fundamentals**: Requires continuous scanning, hardened pipelines, signed artifacts, and admission control.
- **SLSA Framework (Supply-chain Levels for Software Artifacts)**: Level 3 — build platform verifiable, source version controlled, build process isolated.

**Refactor budget**: `Minimal local refactor permitted in listed files only`

#### Contract Block

| Field | Value |
|---|---|
| Inputs | ECR repos from M5, EKS cluster from M6, CI/CD pipeline from M5 |
| Outputs | Signed images in ECR, SBOMs attached to images, Kyverno policies enforced, SLSA provenance attestations |
| Interfaces touched | ECR, GitHub Actions, Kyverno (Kubernetes), AWS Signer / cosign, Trivy, Syft, Pulumi state |
| Files allowed to change | `.github/workflows/app-build-deploy.yml`, Kubernetes manifests for Kyverno |
| New files allowed | Kyverno policy YAML, signing key configuration, test files |
| New dependencies allowed | `kyverno` (Helm chart), `cosign`, `syft`, `trivy`, `slsa-verifier` |
| Migration allowed | `no` |
| Compatibility commitments | All prior infrastructure unchanged |
| Forbidden shortcuts | Images without scanning, images without SBOM, unsigned images in prod, admission control in audit-only mode in prod |

#### Supply Chain Security Architecture

```
┌─────────────────── Software Supply Chain ──────────────────────────┐
│                                                                     │
│  Source Code ──► Version Control (signed commits recommended)       │
│       │                                                             │
│       ▼                                                             │
│  Dependency Resolution                                              │
│  ├── Lock file pinned (package-lock.json / Cargo.lock)             │
│  ├── npm audit / cargo audit (fail on critical)                     │
│  └── Renovate/Dependabot for automated updates                     │
│       │                                                             │
│       ▼                                                             │
│  Build (GitHub Actions — isolated, ephemeral runner)                │
│  ├── Reproducible build (pinned base images, deterministic steps)  │
│  ├── SLSA Level 3 provenance (GitHub artifact attestations)        │
│  └── Build log retained                                             │
│       │                                                             │
│       ▼                                                             │
│  Scan                                                               │
│  ├── SAST: Semgrep / CodeQL (in PR, before merge)                  │
│  ├── Container scan: Trivy (critical/high = build fails)           │
│  ├── ECR enhanced scanning (Inspector, post-push)                  │
│  └── License compliance scan                                        │
│       │                                                             │
│       ▼                                                             │
│  SBOM Generation                                                    │
│  ├── Syft generates CycloneDX SBOM                                 │
│  ├── SBOM attached to ECR image as OCI artifact                    │
│  └── SBOM stored in S3 for audit trail                             │
│       │                                                             │
│       ▼                                                             │
│  Signing                                                            │
│  ├── cosign signs image digest (keyless with Sigstore or KMS key) │
│  ├── Signature stored in ECR alongside image                       │
│  └── SLSA provenance attestation attached                          │
│       │                                                             │
│       ▼                                                             │
│  Admission Control (Kyverno in EKS)                                 │
│  ├── Policy: only images from internal ECR allowed                 │
│  ├── Policy: image must have valid cosign signature                │
│  ├── Policy: image must have SBOM attestation                      │
│  ├── Policy: no :latest tags allowed                               │
│  ├── Policy: no privileged containers                              │
│  └── Mode: ENFORCE in prod, AUDIT in dev                           │
│       │                                                             │
│       ▼                                                             │
│  Running Workload (EKS)                                             │
│  ├── Runtime scanning (ECR continuous scan)                        │
│  └── Vulnerability alerts to Security Hub                          │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

#### NIST SSDF Mapping

| SSDF Practice | Implementation in This Milestone |
|---|---|
| PO.1 (Define security requirements) | Kyverno policies define what may run |
| PS.1 (Protect all forms of code) | Signed commits, branch protection, OIDC auth for CI |
| PS.2 (Verify third-party components) | Trivy scan, npm/cargo audit, SBOM generation |
| PW.4 (Reuse secure software) | Pinned base images from ECR, verified dependencies |
| PW.6 (Configure compilation/build) | Isolated CI runners, reproducible builds, SLSA provenance |
| PW.7 (Review and test code) | SAST in PR, required reviews, automated scan gates |
| PW.9 (Configure software to have secure settings) | Default-deny network policy, restricted PSS, no privileged |
| RV.1 (Identify and confirm vulnerabilities) | Continuous ECR scanning, inspector findings to Security Hub |

#### BDD Acceptance Scenarios

**Feature: Software Supply Chain Security**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Image scanned before push | happy path | CI pipeline runs | Image built | Trivy scan runs, blocks on critical/high |
| SBOM generated and attached | happy path | Image scanned clean | SBOM step runs | CycloneDX SBOM attached to image as OCI artifact |
| Image signed with cosign | happy path | SBOM attached | Signing step runs | cosign signature attached to image digest in ECR |
| Signed image admitted to EKS | happy path | Kyverno enforce mode | `kubectl apply` deployment with signed image | Pod created |
| Unsigned image rejected in prod | security | Kyverno enforce mode | `kubectl apply` deployment with unsigned image | Pod creation denied |
| Non-ECR image rejected in prod | security | Kyverno enforce mode | `kubectl apply` deployment with Docker Hub image | Pod creation denied |
| `:latest` tag rejected | security | Kyverno enforce mode | `kubectl apply` deployment with `:latest` tag | Pod creation denied |
| Critical vuln fails build | security | Trivy finds critical CVE | Pipeline evaluates scan | Build fails, image not pushed |
| SLSA provenance verifiable | supply chain | Image pushed with provenance | `slsa-verifier verify-image` runs | Verification passes |
| Dependency audit fails on critical | security | `npm audit` finds critical | Pipeline evaluates audit | Build fails |

#### E2E Runtime Validation

**File**: `infra/test/e2e/supply-chain.e2e.test.ts`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `kyverno_rejects_unsigned` | Admission control works | Unsigned image deployment rejected |
| `kyverno_rejects_external_registry` | Only internal ECR allowed | Docker Hub image rejected |
| `kyverno_allows_signed_ecr` | Signed internal images work | Signed ECR image admitted |
| `ecr_images_have_sbom` | SBOM attached to all prod images | OCI artifact manifest includes SBOM |
| `ecr_images_have_signature` | Signature attached | cosign verify succeeds |

#### Smoke Tests

- [ ] Build a test image via CI/CD pipeline — passes scan, gets SBOM, gets signed
- [ ] `cosign verify` on the pushed image succeeds
- [ ] Deploy signed image to EKS — pod runs
- [ ] Attempt to deploy unsigned image to EKS — rejected by Kyverno
- [ ] Attempt to deploy Docker Hub image to EKS — rejected by Kyverno
- [ ] `syft` SBOM is viewable as OCI artifact on the ECR image
- [ ] `slsa-verifier verify-image` passes for images built in CI

#### Definition of Done

- Trivy scanning integrated in CI pipeline (fail on critical/high)
- SBOM (CycloneDX) generated by Syft and attached to images
- cosign signing integrated in CI pipeline
- SLSA provenance attestation attached
- Kyverno installed on EKS with enforce-mode policies for signed images, internal ECR, no `:latest`
- Dependency audit (npm audit / cargo audit) in CI
- SAST (Semgrep/CodeQL) in PR workflow
- All tests pass
- Lessons file written, Milestone Tracker updated

---

### Milestone 8 — Backup, Recovery & Ransomware Resilience

**Goal**: Deploy AWS Backup with vault lock, cross-account replication to the Security account, cross-region replication to the DR region, immutable retention policies, and tested restore procedures — establishing ransomware-resilient data protection.

**Context**: We have workloads running in EKS (M6) and data in RDS/S3/EBS. If ransomware encrypts our data or an attacker deletes resources, we need to restore to a known-good state. This milestone creates the backup infrastructure that makes recovery possible even if the prod account is fully compromised. The key innovation is cross-account backup replication to the Security account (which workload accounts cannot access) with vault lock (which prevents anyone, including account admins, from deleting backups before the retention period expires).

**Important design rule**: Backup vaults must use AWS Backup Vault Lock in compliance mode (not governance mode). Compliance mode is irreversible and prevents even the root user from deleting backups. Cross-account replication goes to the Security account, which workload accounts cannot assume roles into. This creates an air gap that survives complete compromise of the prod account.

**Refactor budget**: `No refactor permitted beyond direct implementation`

#### Contract Block

| Field | Value |
|---|---|
| Inputs | EKS volumes, RDS instances, S3 buckets, DynamoDB tables (identifiable by tags) |
| Outputs | Backup vault ARNs, backup plan ARNs, replication job ARNs, vault lock configuration |
| Interfaces touched | AWS Backup, KMS, S3 (for S3 backup), IAM, Pulumi state |
| Files allowed to change | `infra/stacks/08-backup/index.ts`, `infra/lib/backup-vault.ts` |
| New files allowed | Stack configs, tests, E2E tests |
| New dependencies allowed | `@aws-sdk/client-backup` |
| Migration allowed | `no` |
| Compatibility commitments | All prior infrastructure unchanged |
| Forbidden shortcuts | Vault lock in governance mode (must be compliance), backup vault without encryption, replication without cross-account, restore procedures not documented |

#### Backup Architecture

```
┌────── Prod Account (us-east-1) ──────┐    ┌─── Security Acct (us-east-1) ──┐
│                                       │    │                                 │
│  EKS PVs ──┐                         │    │  Backup Vault (replica)         │
│  RDS ──────┤   Backup Vault          │───►│  ├── Vault Lock (compliance)    │
│  S3 ───────┤   ├── Vault Lock        │    │  ├── KMS encrypted              │
│  DynamoDB ─┘   ├── KMS encrypted     │    │  └── 1-year min retention       │
│                ├── Daily backups      │    │                                 │
│                ├── Weekly backups     │    └───────────┬─────────────────────┘
│                └── Monthly backups    │                │
│                                       │                ▼
└───────────────────────────────────────┘    ┌─── Security Acct (us-west-2) ──┐
                                             │                                 │
                                             │  Backup Vault (DR)              │
                                             │  ├── Vault Lock (compliance)    │
                                             │  ├── KMS encrypted              │
                                             │  └── 1-year min retention       │
                                             │                                 │
                                             └─────────────────────────────────┘

Backup Schedule:
├── Daily: 35-day retention, runs at 03:00 UTC
├── Weekly: 90-day retention, runs Sunday 04:00 UTC  
└── Monthly: 365-day retention, runs 1st of month 05:00 UTC

Cross-account replication: within 24 hours of backup completion
Cross-region replication: within 24 hours of cross-account copy
```

#### Files Allowed To Change

| File | Planned Change |
|---|---|
| `infra/lib/backup-vault.ts` | NEW: Reusable backup vault component with vault lock |
| `infra/stacks/08-backup/Pulumi.yaml` | NEW: Stack definition |
| `infra/stacks/08-backup/index.ts` | NEW: Backup vaults, plans, replication |
| `infra/stacks/08-backup/__tests__/backup.test.ts` | NEW: Unit tests |
| `infra/test/e2e/backup.e2e.test.ts` | NEW: E2E tests |

#### Step-by-Step

1. Write unit tests first.
2. Write E2E stubs.
3. Create `infra/lib/backup-vault.ts` — reusable component:
   - Creates a Backup Vault with KMS encryption
   - Configures Vault Lock in compliance mode with specified retention
   - Sets up access policy (deny all except AWS Backup service)
4. Implement `infra/stacks/08-backup/index.ts`:
   - **Prod account**:
     - Backup Vault with Vault Lock (compliance, 35-day minimum retention)
     - KMS key for backup encryption
     - Backup Plan with three rules:
       - Daily: 35-day retention, 03:00 UTC, copy to Security account vault
       - Weekly: 90-day retention, Sunday 04:00 UTC, copy to Security account vault
       - Monthly: 365-day retention, 1st of month 05:00 UTC, copy to Security account vault
     - Resource selection by tag: `Backup: true`
     - IAM role for AWS Backup with minimum permissions
   - **Security account (primary region)**:
     - Backup Vault with Vault Lock (compliance, 365-day minimum retention)
     - KMS key (separate from prod)
     - Cross-region copy rule to DR region
   - **Security account (DR region)**:
     - Backup Vault with Vault Lock (compliance, 365-day minimum retention)
     - KMS key (separate)
   - **S3 backup plan**:
     - S3-specific backup for critical buckets (Pulumi state, application data)
     - Point-in-time recovery for S3
   - **DynamoDB backup plan** (if using DynamoDB):
     - Point-in-time recovery enabled
     - On-demand backup to vault
5. Document restore procedures:
   - Restore from prod vault (fastest, for non-security incidents)
   - Restore from Security account vault (for prod compromises)
   - Restore from DR region vault (for region-wide failures)
   - Full disaster recovery procedure (rebuild from scratch using backups + IaC)
6. Run tests and deploy.

#### BDD Acceptance Scenarios

**Feature: Backup and Ransomware Resilience**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Prod vault has vault lock | happy path | Backup stack deployed | Vault lock queried | Compliance mode, 35-day min retention |
| Security vault has vault lock | happy path | Backup stack deployed | Vault lock queried | Compliance mode, 365-day min retention |
| Daily backup runs | happy path | Backup plan active | 03:00 UTC passes | Backup job completes for tagged resources |
| Cross-account replication works | happy path | Backup completes in prod | Replication job queried | Copy exists in Security account vault |
| Cross-region replication works | happy path | Copy exists in Security primary | DR vault queried | Copy exists in Security DR vault |
| Prod cannot delete Security backups | ransomware resilience | Prod account compromised | Attacker assumes any role in Security | Access denied — no trust relationship |
| Vault lock prevents early deletion | ransomware resilience | Vault lock active | `deleteRecoveryPoint` before retention expires | Deletion denied |
| Restore from prod vault | recovery | Backup exists in prod vault | Restore initiated | Resource restored successfully |
| Restore from Security vault | recovery | Prod compromised, backup in Security | Restore from Security initiated | Resource restored to new account |
| Untagged resources not backed up | scope control | Resource without `Backup: true` | Backup plan runs | Resource skipped |

#### E2E Runtime Validation

**File**: `infra/test/e2e/backup.e2e.test.ts`

| E2E Test | What It Proves | Pass Criteria |
|---|---|---|
| `prod_vault_has_lock` | Prod backups are immutable | Vault lock in compliance mode |
| `security_vault_has_lock` | Cross-account backups are immutable | Vault lock in compliance mode, 365-day retention |
| `backup_plan_has_three_rules` | Correct schedule | Daily, weekly, monthly rules exist |
| `cross_account_replication_configured` | Recovery survives prod compromise | Copy action targets Security account vault |
| `cross_region_replication_configured` | Recovery survives region outage | Copy action targets DR region vault |
| `vault_lock_prevents_deletion` | Ransomware resilience | `deleteRecoveryPoint` API call fails |

#### Smoke Tests

- [ ] `aws backup describe-backup-vault` shows vault lock in compliance mode
- [ ] `aws backup list-backup-plans` shows plan with daily/weekly/monthly rules
- [ ] Tag a test EBS volume with `Backup: true` — appears in next backup job
- [ ] Attempt `aws backup delete-recovery-point` — fails due to vault lock
- [ ] Security account has copied backups from prod
- [ ] DR region has copied backups from Security primary

#### Definition of Done

- Backup vaults with vault lock (compliance mode) in prod, Security (primary), and Security (DR)
- Backup plan with daily (35d), weekly (90d), monthly (365d) retention
- Cross-account replication to Security account
- Cross-region replication to DR region
- Tag-based resource selection
- Vault lock prevents deletion
- Restore procedure documented
- All tests pass
- Lessons file written, Milestone Tracker updated

---

### Milestone 9 — Operational Readiness & Validation

**Goal**: Perform end-to-end integration testing of the complete stack, document operational procedures (incident response, on-call, cost monitoring, certificate rotation), create runbooks for common operational tasks, and validate that all nine layers of security are functioning together.

**Context**: All infrastructure layers are deployed. But we have not yet validated that they work together end-to-end. A deployment through CI/CD, landing on EKS, logging to CloudTrail, monitored by GuardDuty, backed up by AWS Backup — end to end. This milestone validates the complete system and fills in the operational gaps needed for day-two operations.

**Important design rule**: This is a validation and documentation milestone. No new infrastructure beyond what is needed for testing. The goal is to prove the system works and document how to operate it.

**Refactor budget**: `No refactor permitted beyond direct implementation`

#### Contract Block

| Field | Value |
|---|---|
| Inputs | All prior milestone outputs |
| Outputs | Integration test results, operational runbooks, cost baseline, incident response plan |
| Interfaces touched | All layers (read-only verification + one end-to-end deploy) |
| Files allowed to change | Documentation files, test files |
| New files allowed | `docs/ops/`, integration test files |
| New dependencies allowed | `none` |
| Migration allowed | `no` |
| Compatibility commitments | All infrastructure unchanged |
| Forbidden shortcuts | Skipping any layer verification |

#### Security Layer Validation Matrix

| Layer | What to Validate | How |
|---|---|---|
| **1. Identity & Access** | SSO login, MFA enforcement, role assumption, SCP enforcement | Attempt unauthorized action → denied |
| **2. DNS (External)** | Public zone resolves, DNSSEC validates, no internal records leak | `dig +dnssec startup.com`, verify from outside VPC |
| **3. DNS (Internal)** | Private zone resolves within VPC, not from internet | `dig internal.startup.com` from inside and outside VPC |
| **4. Network (External)** | Only ports 80/443 open, NACLs block everything else | `nmap` scan from outside, verify only LB ports open |
| **5. Network (Internal)** | Transit Gateway routes work, private subnets isolated | Cross-VPC connectivity test, verify no internet from isolated subnets |
| **6. Cloud Control Plane** | CloudTrail logging, Config recording, no unauthorized API calls | API call → appears in CloudTrail within 15 minutes |
| **7. Cloud Data Plane** | All storage encrypted, all buckets non-public, all EBS encrypted | AWS Config conformance pack results |
| **8. Runtime (EKS)** | Private endpoint, network policies enforce, PSS enforces, admission control enforces | Deploy test workload, attempt policy violations |
| **9. Software Supply Chain** | Image signed, SBOM present, unsigned rejected, provenance verifiable | Full CI/CD pipeline run, verify artifacts, attempt unsigned deploy |
| **10. Backup & Recovery** | Backup job runs, cross-account copy works, restore succeeds, vault lock holds | Trigger backup, verify copy, test restore, attempt deletion |

#### Operational Documentation To Create

| Document | Path | Contents |
|---|---|---|
| Incident Response Plan | `docs/ops/incident-response.md` | Severity levels, escalation paths, containment procedures, evidence preservation |
| On-Call Runbook | `docs/ops/on-call.md` | Common alerts, diagnosis steps, remediation, who to contact |
| Cost Monitoring | `docs/ops/cost-monitoring.md` | Budget alerts, cost anomaly detection, reserved instances plan |
| Certificate Rotation | `docs/ops/cert-rotation.md` | ACM certificate renewal, KMS key rotation, OIDC token rotation |
| DR Playbook | `docs/ops/disaster-recovery.md` | RPO/RTO targets, failover procedure, failback procedure, region evacuation |
| Access Management | `docs/ops/access-management.md` | Onboarding/offboarding, role changes, break-glass procedure |
| Compliance Checklist | `docs/ops/compliance-checklist.md` | Monthly/quarterly checks, SOC 2 prep, NIST alignment |

#### BDD Acceptance Scenarios

**Feature: Operational Readiness**

| Scenario | Category | Given | When | Then |
|---|---|---|---|---|
| Full pipeline deploys to EKS | integration | Complete stack running | App pushed through CI/CD | Image scanned, signed, SBOM generated, deployed to EKS, pods running |
| API call appears in CloudTrail | observability | CloudTrail active | EC2 API call made in prod | Event appears in CloudTrail within 15 mins |
| GuardDuty detects port scan | detection | GuardDuty active | Simulated port scan on EKS node | GuardDuty finding generated |
| Backup restores successfully | recovery | Backup exists | Restore initiated for EBS volume | New volume created with correct data |
| All SCPs enforced | governance | SCPs active | Attempted violations in each OU | All denied |
| All encryption verified | compliance | Resources deployed | Config conformance checked | All resources encrypted |
| Cost alert fires | cost mgmt | Budget configured | Projected cost exceeds threshold | SNS notification sent |

#### Smoke Tests

- [ ] End-to-end: push code → CI builds → scans → signs → deploys to dev EKS → pods healthy
- [ ] CloudTrail: API call in prod → event in Security account S3 within 15 minutes
- [ ] GuardDuty: simulated finding → appears in Security Hub
- [ ] DNS: external resolution works, internal resolution works within VPC, internal DNS does NOT resolve from internet
- [ ] Network: `nmap` from internet shows only 80/443 open on ALB
- [ ] EKS: unsigned image deployment rejected by Kyverno
- [ ] Backup: EBS restore from Security account vault succeeds
- [ ] SCP: `aws ec2 run-instances --region eu-west-1` denied in prod
- [ ] Cost: AWS Budget alert configured with SNS notification

#### Definition of Done

- All 10 security layers validated end-to-end
- Integration test suite passes
- 7 operational documents created
- Cost baseline established with budget alerts
- DR playbook documented with RPO/RTO
- Incident response plan documented
- All smoke tests checked off
- Lessons file written
- Completion summary written with full architecture state
- Milestone Tracker updated — all milestones done

---

## Documentation Update Table

| Milestone | ARCHITECTURE.md Update | README.md Update | Other Docs |
|---|---|---|---|
| 1 | AWS Organization Structure section | Infrastructure / Getting Started section | — |
| 2 | Identity & Governance section | SSO login instructions | — |
| 3 | Network Architecture section, CIDR table, DNS architecture | — | — |
| 4 | Security Baseline section (CloudTrail, GuardDuty, SecHub, Config) | — | — |
| 5 | CI/CD Pipeline section | Deployment instructions | — |
| 6 | EKS Architecture section | Kubectl access instructions | — |
| 7 | Software Supply Chain section, NIST SSDF mapping | — | — |
| 8 | Backup & DR section, RPO/RTO targets | — | DR Playbook |
| 9 | Full architecture diagram update | Complete getting-started guide | All 7 ops docs |

---

## Optional Fast-Fail Review Prompt for Agents

Use this before writing production code:

> Restate the milestone goal, allowed files, forbidden changes, compatibility requirements, tests that must be written first, and the exact Definition of Done. Then list the smallest implementation approach that satisfies the contract without widening scope.

---

## Appendix A — AWS Service Reference

| Service | Purpose | Account |
|---|---|---|
| AWS Organizations | Multi-account management | Management |
| IAM Identity Center | SSO with MFA | Management |
| Service Control Policies | Org-wide guardrails | Management |
| CloudTrail | API audit logging | Security (delegated admin) |
| GuardDuty | Threat detection | Security (delegated admin) |
| Security Hub | Security findings aggregation | Security (delegated admin) |
| AWS Config | Configuration compliance | Security (delegated admin) |
| VPC Flow Logs | Network traffic logging | All workload accounts → Security |
| Route 53 | DNS (public + private) | Shared Services |
| Transit Gateway | Cross-VPC connectivity | Shared Services |
| ECR | Container registry | Shared Services |
| EKS | Kubernetes runtime | Workload accounts |
| AWS Backup | Backup with vault lock | All accounts → Security |
| KMS | Encryption key management | Per-account keys |
| ACM | TLS certificates | Workload accounts |

## Appendix B — Pulumi Configuration Reference

### Initial Setup

```bash
# Install Pulumi CLI
curl -fsSL https://get.pulumi.com | sh

# Login to Pulumi Cloud (recommended for state management)
pulumi login

# Or use S3 backend (self-managed)
pulumi login s3://my-pulumi-state-bucket

# Configure AWS access (see https://www.pulumi.com/docs/iac/get-started/aws/configure/)
export AWS_PROFILE=management
aws sts get-caller-identity  # Verify access

# For CI/CD, use Pulumi ESC with OIDC (no static credentials)
# See: https://www.pulumi.com/docs/esc/integrations/dynamic-login-credentials/aws-login/
```

### Stack Naming Convention

```
<project>-<environment>
# Examples:
org-prod
identity-prod
network-dev
network-staging
network-prod
security-prod
cicd-prod
eks-dev
eks-staging
eks-prod
backup-prod
```

### Secrets Management

```bash
# Use Pulumi secrets (encrypted in state)
pulumi config set --secret dbPassword "hunter2"

# Or use AWS KMS for encryption
pulumi stack init --secrets-provider="awskms://alias/pulumi-secrets"
```

## Appendix C — DoD DevSecOps & Supply Chain Security References

| Reference | Key Requirements | Milestone Coverage |
|---|---|---|
| **NIST SP 800-218 (SSDF)** | Secure development practices, vulnerability management, software integrity | M5 (CI/CD), M7 (Supply Chain) |
| **NSA/CISA Securing the Software Supply Chain** | SBOM, dependency management, build security, signing, provenance | M7 (Supply Chain) |
| **DoD Enterprise DevSecOps Fundamentals** | Continuous scanning, hardened pipelines, signed artifacts, admission control | M5, M6, M7 |
| **DoD Enterprise DevSecOps Reference Design (CNCF Kubernetes)** | Kubernetes-native DevSecOps, namespace isolation, network policy, admission control | M6 (EKS), M7 (Supply Chain) |
| **SLSA Framework** | Build provenance, source integrity, build isolation | M7 (Supply Chain) |
| **DoD Zero Trust Reference Architecture** | Never trust/always verify, micro-segmentation, least privilege | M2 (Identity), M3 (Network), M6 (EKS) |
| **CISA Zero Trust Maturity Model** | Identity, devices, networks, applications, data — all verified | All milestones |
| **DoD Cloud Security Playbook** | Cloud-native security controls, shared responsibility model | M4 (Security), M8 (Backup) |
| **ICT-SCRM (Supply Chain Risk Management) Strategy** | Third-party risk assessment, vendor vetting, component inventory | M7 (Supply Chain) |

## Appendix D — Cost Optimization for Startups

| Strategy | Savings | Implementation |
|---|---|---|
| Reserved Instances for EKS nodes | ~30-40% | After 3 months of stable usage, purchase 1-year RIs |
| Single NAT Gateway per VPC (dev/staging) | ~$30/mo per removed NAT | Use 1 NAT GW in dev/staging instead of 3 |
| Spot instances for dev node groups | ~60-70% | Mixed instance policy with spot for non-critical workloads |
| S3 Intelligent-Tiering for logs | ~40% on cold data | Apply to log buckets |
| Right-size EKS nodes | Varies | Monitor with Kubecost, adjust after 2 weeks |
| AWS Savings Plans for compute | ~20-30% | After baseline established |
| Turn off dev environment nights/weekends | ~65% | Scheduled scaling to 0 |
| Use Graviton (arm64) instances | ~20% | `m7g.large` instead of `m6i.large` |
