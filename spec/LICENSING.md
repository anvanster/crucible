# Crucible Licensing

Crucible uses a **dual licensing** approach to maximize adoption of the standard while protecting contributors to the implementation code.

## Quick Summary

- **Specification files**: [CC0 1.0 Universal](LICENSE-SPEC) (Public Domain)
- **Implementation code**: [Apache License 2.0](LICENSE-CODE)

## What This Means

### You Can Freely Use the Specification

The Crucible specification, schema, examples, and documentation are released under CC0 1.0, which means:
- ✅ Use it commercially
- ✅ Modify it however you want
- ✅ No attribution required (though appreciated!)
- ✅ Implement it in any language
- ✅ Create competing implementations
- ✅ Fork the specification

**No restrictions whatsoever on the specification.**

### Implementation Code is Apache 2.0

When we release implementation code (CLI tools, libraries, generators), it will be under Apache 2.0, which means:
- ✅ Use it commercially
- ✅ Modify and distribute it
- ✅ Patent grant included
- ⚠️ Attribution required (must include license and copyright notices)
- ⚠️ Changes must be documented

**Still very permissive, but with contributor protections.**

## Detailed Breakdown

### Files Under CC0 1.0 (Public Domain)

These files are completely unrestricted:

```
spec/
├── LICENSE-SPEC              # CC0 1.0 license text
├── SPEC.md                   # Technical specification
├── schema.json               # JSON Schema definitions
├── README.md                 # Overview and introduction
├── GETTING-STARTED.md        # Tutorial documentation
├── PROJECT-STRUCTURE.md      # Organization guide
├── BRANDING.md              # Branding guidelines
├── INDEX.md                 # Navigation guide
└── examples/
    ├── example-manifest.json
    ├── example-module-*.json
    └── example-rules.json
```

**Why CC0?**
- Specifications benefit from maximum adoption
- No barriers for companies to implement
- Allows anyone to improve and extend the spec
- Follows precedent of W3C, IETF, and other standards bodies
- Encourages ecosystem growth

### Files Under Apache 2.0

These files (when created) will require attribution:

```
crucible-cli/
├── LICENSE-CODE              # Apache 2.0 license text
├── Cargo.toml
└── src/
    └── *.rs

crucible-core/
├── LICENSE-CODE              # Apache 2.0 license text
├── Cargo.toml
└── src/
    └── *.rs

crucible-codegen/
├── LICENSE-CODE              # Apache 2.0 license text
└── ...
```

**Why Apache 2.0?**
- Industry-standard for open source software
- Explicit patent grant protects users
- Protects contributors from liability
- Corporate-friendly (widely trusted and understood)
- Allows commercial use while ensuring attribution

## Rationale for Dual Licensing

### Specifications Need Maximum Freedom

Standards succeed when they're adopted widely. CC0 removes all possible barriers:
- **No legal uncertainty** - Companies don't need to review license terms
- **No attribution burden** - Implementers don't need to track credits
- **Complete flexibility** - Anyone can fork, extend, or compete
- **Clear public domain** - No risk of license changes

### Code Needs Contributor Protection

Software implementations benefit from Apache 2.0:
- **Patent protection** - Contributors grant patent licenses to users
- **Liability protection** - Contributors aren't liable for bugs
- **Contributor clarity** - Clear terms for accepting contributions
- **Trademark protection** - Brand can be protected separately
- **Industry standard** - Trusted by enterprises worldwide

## Real-World Precedents

Many successful projects use this dual approach:

**Rust**
- Language specification: Freely available
- Implementation (rustc): Apache 2.0 / MIT

**HTTP/2**
- Protocol specification: IETF (free)
- Implementations: Various licenses (Apache, MIT, etc.)

**JSON**
- Specification: Public domain
- Libraries: Various licenses

**Protocol Buffers**
- Format specification: Public
- Compiler: BSD 3-Clause

## FAQ

### Can I implement Crucible in my own tools?

**Yes!** The specification is CC0 - you can implement it however you want, under any license you want.

### Do I need to credit Crucible if I use the spec?

**No.** CC0 requires no attribution. However, we'd appreciate it if you mention you're Crucible-compatible!

### Can I fork the specification?

**Yes!** You can create your own variant, no permission needed. Though we'd prefer you contribute improvements back to help the ecosystem.

### Can I use crucible-cli commercially?

**Yes!** Apache 2.0 allows commercial use. You just need to include the license and copyright notices.

### What if I want to contribute code?

Your contributions to implementation code will be under Apache 2.0. By submitting a PR, you agree to license your contribution under Apache 2.0.

### Can I build a proprietary tool using Crucible?

**Yes!**
- For the specification: No restrictions (CC0)
- For implementation code: Yes, but must follow Apache 2.0 terms (attribution, license inclusion)

### What about modifications to the spec?

Under CC0, you can modify and redistribute the specification without any restrictions. You could even create a competing standard.

### Can companies use this?

**Absolutely!** Both licenses are corporate-friendly:
- CC0: No restrictions whatsoever
- Apache 2.0: Industry standard, widely approved by legal teams

### What about patents?

- **Specification (CC0)**: No patent grant, but specifications typically don't involve patents
- **Implementation (Apache 2.0)**: Includes explicit patent grant from contributors

## Contributing

### Contributing to the Specification

By contributing to specification files (under `spec/`), you agree to release your contribution under CC0 1.0 (public domain).

This means you're giving up all rights to your contribution. We do this to keep the specification completely open and unrestricted.

### Contributing to Implementation Code

By contributing to implementation code (under `crucible-cli/`, `crucible-core/`, etc.), you agree to license your contribution under Apache 2.0.

This provides you and other contributors with patent and liability protections.

## License Headers

### For Specification Files

No header required (CC0 doesn't require it), but you may optionally include:

```
# This file is part of the Crucible specification
# Released under CC0 1.0 Universal (Public Domain)
# https://creativecommons.org/publicdomain/zero/1.0/
```

### For Implementation Code

Include in source files:

```rust
// Copyright 2025 Crucible Contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
```

## Repository Structure

When the project is on GitHub:

```
crucible-spec/crucible/
│
├── spec/                     # CC0 1.0
│   ├── LICENSE-SPEC          # CC0 1.0 full text
│   ├── SPEC.md
│   ├── schema.json
│   └── examples/
│
├── crucible-cli/             # Apache 2.0
│   ├── LICENSE-CODE          # Apache 2.0 full text
│   └── src/
│
├── crucible-core/            # Apache 2.0
│   ├── LICENSE-CODE          # Apache 2.0 full text
│   └── src/
│
├── LICENSING.md              # This file
└── README.md                 # Main project readme
```

## Getting Help

If you have questions about licensing:
- **General questions**: Open a GitHub Discussion
- **Specific legal questions**: Consult your own legal counsel (we're not lawyers!)
- **Clarifications**: File a GitHub issue

## Changes to Licensing

The dual licensing approach is part of Crucible's core philosophy:
- **Specification will always be public domain** (or most permissive possible)
- **Implementation code will remain Apache 2.0** (or similarly permissive)

We believe this combination maximizes both adoption and sustainability.

## Attribution (Optional but Appreciated)

While CC0 doesn't require attribution, if you implement Crucible or use it in your project, we'd love to hear about it!

Consider:
- Adding a "Crucible-compatible" badge to your tool
- Mentioning Crucible in your documentation
- Linking back to the specification
- Sharing your implementation with the community

## Summary Table

| Component | License | Attribution Required? | Patent Grant? | Commercial Use? |
|-----------|---------|----------------------|---------------|-----------------|
| Specification | CC0 1.0 | No | N/A | ✅ Yes |
| Documentation | CC0 1.0 | No | N/A | ✅ Yes |
| Examples | CC0 1.0 | No | N/A | ✅ Yes |
| Schema | CC0 1.0 | No | N/A | ✅ Yes |
| CLI Tool | Apache 2.0 | Yes | Yes | ✅ Yes |
| Core Library | Apache 2.0 | Yes | Yes | ✅ Yes |
| Code Generators | Apache 2.0 | Yes | Yes | ✅ Yes |

---

**TL;DR**: Use the spec however you want (it's public domain). Use the code however you want (just keep the license notice). Build cool things! 🚀
