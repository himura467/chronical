import { Temporal } from "temporal-polyfill";
import { RRuleSet as NativeRRuleSet } from "./generated";

export class RRuleSet {
  private native: NativeRRuleSet;

  constructor(dtStart: Temporal.ZonedDateTime) {
    this.native = new NativeRRuleSet(dtStart.toString());
  }
}
