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

  between(
    after: Temporal.ZonedDateTime,
    before: Temporal.ZonedDateTime,
    inclusive?: boolean | undefined | null
  ): Temporal.ZonedDateTime[] {
    return this.native
      .between(after.toString(), before.toString(), inclusive)
      .map((date) => Temporal.ZonedDateTime.from(date));
  }
}
