import { Temporal } from "temporal-polyfill";
import { RRuleSet as NativeRRuleSet } from "./generated/index.js";

export class RRuleSet {
  private native: NativeRRuleSet;

  constructor(dtStart: Temporal.ZonedDateTime) {
    this.native = new NativeRRuleSet(dtStart.toString());
  }

  all(): Temporal.ZonedDateTime[] {
    return this.native.all().map((date) => Temporal.ZonedDateTime.from(date));
  }
}
