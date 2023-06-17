export interface Exec {
  Exec: {
    env: { [key: string]: string } | null,
    working_dir: string | null,
    executable: string,
    user: string | null,
    args: string[],
  }
}

export type TimeZone = "Utc" | "Local" | { Offset: number };

export interface DateTime { year: number, month: number, day: number, hour: number, min: number, sec: number, timestamp: bigint, time_zone: TimeZone, }

export interface Duration { year: number, month: number, day: number, hour: number, min: number, sec: number, total_sec: bigint, }

export type Timer = { Repeat: Duration } | { Once: DateTime } | { ManyTimes: [Duration, number] } | "Never";

export enum Option {
  None = "None",
}

export interface Entry {
  id: number
  name: string | null
  action: Exec | Option
  env: string[] | null
  working_dir: string | null
  logger: {}
  trigger: {}
}
