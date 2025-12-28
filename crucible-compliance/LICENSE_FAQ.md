# Crucible Compliance Module - Licensing FAQ

## General Questions

### What license is the Compliance Module under?

The Compliance Module is licensed under the Business Source License 1.1 (BSL).
This means the source code is publicly available and you can use it freely for
most purposes, with some restrictions on commercial competitive use. After the
Change Date (December 31, 2029), it converts automatically to Apache 2.0.

### Why BSL instead of a traditional open source license?

We believe compliance validation logic should be transparent and auditable.
BSL lets us:

- Ship source code you can inspect, audit, and verify
- Allow broad internal and non-commercial use
- Sustain development by preventing direct commercial competition
- Commit to eventual full open source release

### Is Crucible "open source"?

The core validation engine and specs are open source (Apache 2.0 and CC0).
The Compliance Module is "source available" under BSL, which is not OSI-approved
but provides similar transparency benefits. It becomes fully open source on the
Change Date.

---

## What's Allowed

### Can I use this to validate my company's infrastructure?

**Yes.** Internal use for validating your own systems is explicitly permitted,
regardless of company size or revenue.

### Can I integrate this into our CI/CD pipeline?

**Yes.** Automated validation as part of your internal development and
deployment processes is permitted.

### Can I modify the rules for our specific needs?

**Yes.** You may fork and modify the compliance rules for your organization's
internal use.

### Can I use this for a client project if I'm a consultancy?

**Yes**, provided you're helping the client validate *their* infrastructure.
You're not offering a competing compliance platform—you're providing
consulting services that happen to use this tool.

### Can I use this for academic research or teaching?

**Yes.** Academic and research use is explicitly permitted.

### Can I contribute improvements back?

**Yes, and please do.** Contributions are welcome under our Contributor
License Agreement (CLA).

---

## What's NOT Allowed (Without a Commercial License)

### Can I build a SaaS compliance platform using these rules?

**No.** Offering compliance validation as a service to third parties using
the Licensed Work requires a commercial license.

### Can I embed this in a product we sell?

**No.** Redistributing the compliance rules as part of a commercial product
requires a commercial license.

### Can I extract the rules and use them in our own validation engine?

**No.** The rule definitions themselves are part of the Licensed Work. Using
them in a competing tool or service requires a commercial license.

### Can I offer "compliance-as-a-service" using this?

**No.** Managed compliance offerings to third parties require a commercial
license.

---

## Edge Cases

### We're a small startup—do restrictions apply to us?

If your organization has fewer than 50 employees AND less than $5M in annual
revenue, you may use the Licensed Work for any purpose without restriction,
including commercial use. We want to support early-stage companies.

### What if we're a non-profit?

Non-profit use for internal validation is permitted. If you're offering
compliance services to other organizations (even as a non-profit), please
contact us to discuss licensing.

### We're a government agency. Can we use this?

**Yes.** Government use for internal validation is permitted. Contractors
building compliance platforms for government clients should contact us.

### What about using it in an open source project?

If your project is non-commercial and open source, reach out. We're generally
supportive of open source ecosystem development and can discuss appropriate
licensing.

### What happens on the Change Date?

On December 31, 2029, the license automatically converts to Apache 2.0. At that
point, all restrictions are lifted and you may use the code for any purpose,
including commercial competition.

---

## Commercial Licensing

### How do I get a commercial license?

Contact us at anvanster@gmail.com with:

- Company name and size
- Intended use case
- Expected scale (users, validations/month, etc.)

We typically respond within 2 business days.

### How much does a commercial license cost?

Pricing depends on use case and scale. We offer:

- **Startup tier** — Reduced pricing for companies under $10M ARR
- **Enterprise tier** — Volume pricing for large deployments
- **OEM/Embedded** — Custom terms for product integration

### Can I get a trial commercial license?

Yes. We offer 90-day evaluation licenses for commercial use cases at no cost.

### What if I'm not sure whether my use case requires a license?

When in doubt, ask. Email anvanster@gmail.com with a description of
your use case. We'd rather clarify upfront than create confusion.

---

## Enforcement

### How do you enforce the license?

We rely primarily on good faith and the professional integrity of our users.
The compliance industry is small; reputation matters.

For clear violations, we start with a conversation. Intentional, sustained
violations may result in legal action.

### What if we accidentally violated the license?

Contact us. If you come to us proactively and in good faith, we'll work with
you to find a solution—whether that's a commercial license, a change in your
usage, or something else.

---

## Contact

- **Licensing questions:** anvanster@gmail.com
- **General inquiries:** anvanster@gmail.com
- **Security issues:** anvanster@gmail.com