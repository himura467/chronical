# chronical

A high-performance implementation of RRule (RFC 5545) using the Temporal API, written in Rust with Node.js bindings.

The name _chronical_ comes from _chrono_ (a Rust crate for handling date and time) and _iCalendar_ (RFC 5545).

## Installation

```bash
npm install chronical
```

## Quick Start

```typescript
import { RRule, Frequency } from "chronical";

// Create a daily recurring event
const rule = new RRule(Frequency.Daily);
console.log(rule.freq); // 'Daily'
```

## License

MIT
