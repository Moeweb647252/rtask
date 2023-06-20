import type { Entry } from "./Entry";
import type { Timer } from "./Timer";
import type { Execute } from "./Execute";
import type { Duration } from "./Duration";

export type { Entry, Timer, Execute, Duration };

export interface ExecAction {
  Exec: Execute
}