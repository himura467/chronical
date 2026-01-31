import { Temporal } from "temporal-polyfill";
import { RRule as NativeRRule, type Frequency } from "./generated/index.js";

export class RRule {
  private native: NativeRRule;

  constructor(freq: Frequency) {
    this.native = new NativeRRule(freq);
  }

  private static fromNative(native: NativeRRule): RRule {
    const instance = new RRule(native.freq);
    instance.native = native;
    return instance;
  }

  static fromString(s: string): RRule {
    return RRule.fromNative(NativeRRule.fromString(s));
  }

  get freq(): Frequency {
    return this.native.freq;
  }

  get until(): Temporal.ZonedDateTime | null {
    const until = this.native.until;
    return until ? Temporal.ZonedDateTime.from(until) : null;
  }

  get count(): number | null {
    return this.native.count;
  }

  get interval(): number | null {
    return this.native.interval;
  }

  get bySecond(): number[] {
    return this.native.bySecond;
  }

  get byMinute(): number[] {
    return this.native.byMinute;
  }

  get byHour(): number[] {
    return this.native.byHour;
  }

  get byDay(): string[] {
    return this.native.byDay;
  }

  get byMonthDay(): number[] {
    return this.native.byMonthDay;
  }

  get byYearDay(): number[] {
    return this.native.byYearDay;
  }

  get byWeekNo(): number[] {
    return this.native.byWeekNo;
  }

  get byMonth(): number[] {
    return this.native.byMonth;
  }

  get bySetPos(): number[] {
    return this.native.bySetPos;
  }

  get wkst(): string | null {
    return this.native.wkst;
  }
}
