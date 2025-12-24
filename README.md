# chronical

A high-performance implementation of RRule (RFC 5545) using the Temporal API, written in Rust with Node.js bindings.

The name _chronical_ comes from _chrono_ (a Rust crate for handling date and time) and _iCalendar_ (RFC 5545).

## Installation

```bash
npm install chronical
```

## Quick Start

```typescript
import { RRuleSet } from "chronical";
import { Temporal } from "temporal-polyfill";

const dtStart = Temporal.ZonedDateTime.from("2025-01-01T00:00:00[UTC]");
const rruleSet = new RRuleSet(dtStart);

// Get all occurrences
const dates = rruleSet.all();
console.log(dates); // [ ZonedDateTime [Temporal.ZonedDateTime] {} ]
console.log(dates[0].toString()); // "2025-01-01T00:00:00+00:00[UTC]"
```

## License

MIT
