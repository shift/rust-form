# Component System Architecture Research

## Research Overview
This document contains research findings for implementing a component system architecture for Rust-form, inspired by Nix flakes, NPM, and Cargo package management systems.

## 1. Nix Flakes Architecture Analysis

### Key Features Learned:
- **URI-based identification**: Uses formats like `github:owner/repo`, `git+https://...`, `path:./local`
- **Lockfile system**: `flake.lock` provides reproducible builds with exact revision hashes
- **Content addressing**: Uses NAR (Nix Archive) hashes for integrity verification
- **Dependency graph**: Supports complex circular dependencies through graph representation
- **Registry system**: Indirect references resolved through flake registries
- **Version constraints**: Supports semver-like constraints and exact pinning

### Lockfile Structure:
```json
{
  "version": 7,
  "root": "n1",
  "nodes": {
    "n1": {
      "inputs": {
        "nixpkgs": "n2"
      }
    },
    "n2": {
      "locked": {
        "owner": "NixOS",
        "repo": "nixpkgs", 
        "rev": "7f8d4b088e2df7fdb6b513bc2d6941f1d422a013",
        "type": "github",
        "lastModified": 1580555482,
        "narHash": "sha256-OnpEWzNxF/AU4KlqBXM2s5PWvfI5/BS6xQrPvkF5tO8="
      },
      "original": {
        "id": "nixpkgs",
        "type": "indirect"
      }
    }
  }
}
```

### URI Patterns Identified:
- `github:org/repo@version`
- `git+https://example.com/repo.git?ref=branch&rev=commit`
- `path:./relative/path`
- `tarball+https://example.com/archive.tar.gz`
- `file+https://example.com/file.txt`

## 2. NPM Package Management Analysis

### Key Features:
- **Semver versioning**: Extensive version constraint system (`^`, `~`, exact, ranges)
- **Dependencies structure**: Multiple dependency types (deps, devDeps, peerDeps, optionalDeps)
- **Registry ecosystem**: Centralized registry with scoped packages (@org/package)
- **Workspaces**: Monorepo support with workspace configurations
- **Overrides**: Ability to override transitive dependencies

### Package.json Structure:
```json
{
  "name": "@scope/package-name",
  "version": "1.2.3",
  "dependencies": {
    "lodash": "^4.17.21",
    "express": ">=4.0.0 <5.0.0"
  },
  "devDependencies": {
    "typescript": "~4.9.0"
  },
  "peerDependencies": {
    "react": ">=16.8.0"
  },
  "overrides": {
    "vulnerable-package": "2.0.0"
  }
}
```

### Version Constraint Patterns:
- `^1.2.3` - Compatible with version (>=1.2.3 <2.0.0)
- `~1.2.3` - Approximately equivalent (~1.2.3 := >=1.2.3 <1.3.0)
- `1.2.x` - Patch-level changes allowed
- `>=1.0.0 <2.0.0` - Range specification
- `git+https://...` - Git repository dependencies
- `file:../local` - Local path dependencies

## 3. Cargo Package Management Analysis

### Key Features:
- **TOML configuration**: Clean, readable manifest format
- **Edition system**: Rust edition compatibility handling
- **Feature flags**: Conditional compilation features
- **Build scripts**: Native code compilation support
- **Registry flexibility**: Multiple registry support
- **Workspace support**: Multi-package projects

### Cargo.toml Structure:
```toml
[package]
name = "my-package"
version = "0.1.0"
edition = "2021"
description = "A sample package"
license = "MIT OR Apache-2.0"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
local-crate = { path = "../local-crate" }

[dev-dependencies]
proptest = "1.0"

[features]
default = ["std"]
std = []
async = ["tokio"]
```

### Version Specification:
- `"1.0"` - Caret requirements (^1.0.0)
- `"=1.0.0"` - Exact version
- `">= 1.2, < 1.5"` - Multiple requirements
- `{ git = "https://...", branch = "main" }` - Git dependencies
- `{ path = "../local" }` - Path dependencies

## 4. Dependency Resolution Algorithms Research

### Common Patterns:
1. **SAT solving**: Convert dependencies to boolean satisfiability problem
2. **Backtracking**: Try combinations and backtrack on conflicts
3. **Heuristics**: Use scoring to prefer certain solutions
4. **Conflict resolution**: Strategies for handling incompatible requirements

### Resolution Strategies:
- **Latest-first**: Prefer newest compatible versions
- **Minimal-first**: Prefer oldest compatible versions  
- **User preference**: Allow manual overrides and preferences
- **Conflict detection**: Early detection of unsolvable constraints

## 5. Integrity and Security Patterns

### Content Addressing:
- **SHA-256 hashes**: Standard content verification
- **SRI (Subresource Integrity)**: Web standard hash format
- **Merkle trees**: Efficient verification of large datasets
- **Signed releases**: Cryptographic signatures for authenticity

### Security Considerations:
- **Dependency confusion**: Prevent malicious package substitution
- **Supply chain attacks**: Verify package integrity and source
- **Sandboxing**: Isolate build processes
- **Audit trails**: Track dependency changes and updates

## 6. Component System Design Implications

### URI Scheme Design:
Based on research, our component URIs should support:
```
rust-form/react-crud-table@^2.0.0
github:acme/custom-components@v1.2.3
git+https://gitlab.com/team/components.git?ref=main
npm:@material-ui/core@^5.0.0
path:./local/components/dashboard
```

### Manifest Format:
```yaml
# component.yml
name: "rust-form/react-crud-table"
version: "1.2.0" 
type: "core" # core | commercial | community
license: "MIT"
author: "Rust-form Team"

dependencies:
  - "rust-form/react-query-hooks@^2.0.0"
  - "@tanstack/react-query@^5.0.0"
  - "tailwindcss@^3.0.0"

provides:
  templates:
    - "components/CrudTable.tsx.tera"
    - "components/CrudForm.tsx.tera"

configuration:
  pagination: { type: "boolean", default: true }
  search: { type: "boolean", default: true }
  sorting: { type: "array", items: "string" }

targets: ["react", "vue"]
```

### Lockfile Format:
```yaml
# rust-form.lock
version: 1
components:
  "rust-form/react-crud-table":
    version: "1.2.0"
    resolved: "github:rust-form/react-crud-table@1.2.0"
    integrity: "sha256-abc123..."
    dependencies:
      "@tanstack/react-query": "^5.0.0"

generated_from:
  config_hash: "sha256-def456..."
  rust_form_version: "0.1.0"
  timestamp: "2025-09-08T14:48:55Z"
```

## 7. Implementation Strategy

### Phase 1: Core Infrastructure
1. URI parsing and validation system
2. Component manifest schema and validation
3. Basic dependency resolution algorithm
4. Local component storage and caching

### Phase 2: Registry Integration
1. Remote component fetching (Git, HTTP)
2. Component registry protocol design
3. Authentication and authorization
4. Integrity verification and signing

### Phase 3: Advanced Features
1. Lockfile generation and validation
2. Conflict resolution and overrides
3. Workspace and monorepo support
4. Build caching and optimization

## 8. URI Parsing Research

### RFC 3986 Insights:
- **Generic URI Syntax**: `scheme ":" hier-part [ "?" query ] [ "#" fragment ]`
- **Authority component**: `[ userinfo "@" ] host [ ":" port ]`
- **Path variations**: path-abempty, path-absolute, path-noscheme, path-rootless, path-empty
- **Percent-encoding**: Required for unsafe characters, `%20` for space, etc.
- **Reserved characters**: `:/?#[]@` (gen-delims) and `!$&'()*+,;=` (sub-delims)
- **Unreserved characters**: `ALPHA / DIGIT / "-" / "." / "_" / "~"`

### Component URI Design:
Based on research, our URI scheme should support:
```
scheme:[//authority]path[?query][#fragment]

Examples:
- rust-form/react-crud-table@^2.0.0    # registry shorthand
- github:org/repo@v1.2.3               # GitHub hosting
- git+https://gitlab.com/team/components.git?ref=main  # full Git URI
- npm:@scope/package@^5.0.0           # NPM registry
- path:./local/components             # local filesystem
- http://registry.example.com/component.tar.gz  # HTTP tarball
```

### Parsing Strategy:
1. **Regex-based parsing**: Use RFC 3986 appendix B regex for initial breakdown
2. **Scheme-specific validation**: Each scheme validates its own authority/path
3. **Normalization**: Consistent case handling and percent-encoding cleanup
4. **Resolution**: Convert relative references to absolute URIs

## 9. Dependency Resolution Algorithm Research

### NP-Completeness Analysis:
Version selection is **NP-complete** due to:
1. **Diamond dependency problem**: A needs B&C, B needs D@1, C needs D@2
2. **SAT reduction proof**: Can encode 3-SAT boolean satisfiability into package constraints
3. **Real-world impact**: All major package managers face this complexity

### Resolution Strategies:
1. **SAT Solvers**: Convert to boolean satisfiability (used by FreeBSD pkg, OCaml OPAM)
2. **Backtracking**: Try combinations and backtrack on conflicts (Cargo, Swift PM)
3. **Heuristics**: Use scoring and preferences (APT default mode)
4. **Constraint Programming**: Use Gecode-style solvers (Chef)

### Practical Approaches:
- **Minimal version selection**: Always pick oldest compatible version (Go modules)
- **Latest version preference**: Pick newest compatible version (most package managers)
- **User preference integration**: Allow manual override of choices (OPAM, APT)
- **Conflict explanation**: Provide understandable error messages when resolution fails

### Optimization Techniques:
- **Early conflict detection**: Fail fast on obviously unsolvable constraints
- **Dependency ordering**: Resolve most constrained packages first
- **Caching**: Remember resolution decisions across runs
- **Incremental resolution**: Only re-resolve changed dependencies

### Avoidance Strategies:
To reduce NP-completeness impact:
1. **Minimum version dependencies**: Only specify minimum compatible versions
2. **Allow multiple versions**: Install different major versions simultaneously
3. **Semantic versioning**: Treat major versions as different packages
4. **Restricted dependency syntax**: Limit complexity of version expressions

## 10. Implementation Roadmap

### Phase 1: Core URI and Manifest System
1. URI parsing with scheme-specific validation
2. Component manifest schema (YAML-based)
3. Basic version constraint parsing (semantic versioning)
4. Component storage and local caching

### Phase 2: Dependency Resolution Engine
1. Dependency graph construction
2. Backtracking resolver with conflict detection
3. Lockfile generation and validation
4. Error reporting and conflict explanation

### Phase 3: Registry and Remote Fetching
1. Git repository fetching
2. HTTP/HTTPS tarball downloading
3. Integrity verification (SHA-256 hashes)
4. Registry protocol design

### Phase 4: Advanced Features
1. Multiple version installation
2. Workspace/monorepo support
3. SAT solver integration (optional)
4. Component publishing tools

## 11. Lockfile Formats and Integrity Research

### NPM Package-lock.json Analysis:
- **Lockfile version**: Progressive format evolution (v1→v2→v3)
- **Dual structure**: Both `packages` (v2+) and `dependencies` (v1 compat) sections
- **Integrity verification**: SHA-512/SHA-1 SRI format (`sha512-...`)
- **Resolution metadata**: `resolved` URL and `version` for each package
- **Dependency classification**: `dev`, `optional`, `devOptional` flags
- **Hidden lockfiles**: Performance optimization with `.package-lock.json`

### Cargo Lockfile Analysis:
- **TOML format**: Human-readable, structured format
- **Exact pinning**: Full commit SHAs for Git dependencies
- **Version resolution**: Cargo resolves and pins compatible versions
- **Update mechanism**: `cargo update` for controlled updates
- **Source tracking**: Git repositories, registries, and local paths

### Subresource Integrity (SRI) Standards:
- **Hash algorithms**: SHA-256, SHA-384, SHA-512 (with priority order)
- **Format**: `sha384-[base64-encoded-hash]`
- **Multiple hashes**: Allow algorithm agility for future-proofing
- **Cross-origin requirements**: CORS required for integrity checking
- **Error handling**: Network error on integrity mismatch

### Lockfile Design for Components:
```yaml
# rust-form.lock
version: 1
lockfile_version: 1
rust_form_version: "0.1.0"

# Metadata about lock generation
metadata:
  generated_at: "2025-09-08T14:48:55Z"
  config_hash: "sha256-def456..."
  generator: "rustform@0.1.0"

# Component resolution tree
components:
  "rust-form/react-crud-table":
    version: "1.2.0"
    resolved: "github:rust-form/react-crud-table@1.2.0"
    integrity: "sha384-Li9vy3DqF8tnTXuiaAJuML3ky+er10rcgNR/VqsVpcw+ThHmYcwiB1pbOxEbzJr7"
    dependencies:
      "@tanstack/react-query": "^5.0.0"
      "tailwindcss": "^3.0.0"
    
  "@tanstack/react-query":
    version: "5.8.1"
    resolved: "npm:@tanstack/react-query@5.8.1"
    integrity: "sha512-abc123..."
    dependencies: {}

# Transitive dependency resolution
resolution:
  "@tanstack/react-query@^5.0.0": "5.8.1"
  "tailwindcss@^3.0.0": "3.3.7"
```

### Integrity Checking Strategy:
1. **SHA-384 primary**: Strong, widely supported algorithm
2. **Multi-hash support**: Include SHA-256 for compatibility, SHA-512 for future
3. **Content verification**: Check integrity before template extraction
4. **Chain of trust**: Verify component manifest integrity separately from content
5. **Graceful degradation**: Allow operation without integrity for local components

### Error Handling and Recovery:
- **Integrity mismatch**: Clear error messages with expected vs actual hashes
- **Missing dependencies**: Suggest resolution strategies
- **Version conflicts**: Explain conflicting constraints clearly
- **Network failures**: Implement retry logic with exponential backoff
- **Corruption detection**: Verify lockfile integrity on load

## 12. Component System Implementation Design

### Core Architecture:
```rust
// rustform-core/src/component/mod.rs
pub struct ComponentSystem {
    cache: ComponentCache,
    resolver: DependencyResolver,
    fetcher: ComponentFetcher,
    integrity: IntegrityVerifier,
}

pub struct ComponentManifest {
    pub name: ComponentUri,
    pub version: Version,
    pub dependencies: HashMap<ComponentUri, VersionReq>,
    pub provides: ComponentInterface,
    pub integrity: Option<IntegrityMetadata>,
}

pub struct ComponentLockfile {
    pub version: u32,
    pub components: HashMap<ComponentUri, ResolvedComponent>,
    pub metadata: LockfileMetadata,
}
```

### URI Parsing System:
```rust
// rustform-core/src/component/uri.rs
pub enum ComponentUri {
    Registry { name: String, version: Option<VersionReq> },
    GitHub { owner: String, repo: String, version: Option<String> },
    Git { url: Url, reference: Option<GitRef> },
    Path { path: PathBuf },
    Http { url: Url },
    Npm { name: String, version: Option<VersionReq> },
}

impl FromStr for ComponentUri {
    // Implement RFC 3986 compliant parsing
}
```

### Dependency Resolution Engine:
```rust
// rustform-core/src/component/resolver.rs
pub struct DependencyResolver {
    strategy: ResolutionStrategy,
}

pub enum ResolutionStrategy {
    BacktrackingResolver,
    SatSolver, // Future: integrate with SAT solver
    HeuristicResolver,
}

impl DependencyResolver {
    pub fn resolve(&self, manifest: &ComponentManifest) -> Result<ResolutionGraph> {
        // Implement backtracking with conflict detection
    }
}
```

### Integrity Verification:
```rust
// rustform-core/src/component/integrity.rs
pub struct IntegrityVerifier;

impl IntegrityVerifier {
    pub fn verify_content(&self, content: &[u8], metadata: &IntegrityMetadata) -> bool {
        // Implement SRI verification with multiple hash support
    }
    
    pub fn generate_hash(&self, content: &[u8], algorithm: HashAlgorithm) -> String {
        // Generate SRI-compliant hash strings
    }
}
```

## Next Steps
1. Complete lockfile format design and implementation
2. Design component manifest validation
3. Implement basic dependency graph resolver
4. Create component storage and caching system
5. Build integration with existing template engine