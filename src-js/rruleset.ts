import { Temporal } from "temporal-polyfill";
import { RRuleSet as NativeRRuleSet } from "./generated/index.js";

export class RRuleSet {
  private native: NativeRRuleSet;

  constructor(dtStart: Temporal.ZonedDateTime);
  constructor(native: NativeRRuleSet);
  constructor(arg: Temporal.ZonedDateTime | NativeRRuleSet) {
    this.native =
      arg instanceof NativeRRuleSet ? arg : new NativeRRuleSet(arg.toString());
  }

  private static fromNative(native: NativeRRuleSet): RRuleSet {
    return new RRuleSet(native);
  }

  static fromString(s: string): RRuleSet {
    return RRuleSet.fromNative(NativeRRuleSet.fromString(s));
  }

  toString(): string {
    return this.native.toString();
  }

  all(): Temporal.ZonedDateTime[] {
    return this.native.all().map((date) => Temporal.ZonedDateTime.from(date));
  }

  between(
    after: Temporal.ZonedDateTime,
    before: Temporal.ZonedDateTime,
    inclusive?: boolean | undefined | null,
  ): Temporal.ZonedDateTime[] {
    return this.native
      .between(after.toString(), before.toString(), inclusive)
      .map((date) => Temporal.ZonedDateTime.from(date));
  }
}
