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

const rruleSet = RRuleSet.fromString(
  "DTSTART;TZID=Asia/Tokyo:20250101T000000\nRRULE:FREQ=DAILY;COUNT=3",
);

// Get all occurrences
const dates = rruleSet.all();
console.log(dates.map((d) => d.toString()));
// [
//   "2025-01-01T00:00:00+09:00[Asia/Tokyo]",
//   "2025-01-02T00:00:00+09:00[Asia/Tokyo]",
//   "2025-01-03T00:00:00+09:00[Asia/Tokyo]",
// ]
```

## License

MIT
