import type { Entry } from "./Entry";
import type { Timer } from "./Timer";
import type { Execute } from "./Execute";
import type { Duration } from "./Duration";
import type { EditingEntry } from "./EditingEntry";

export type { Entry, Timer, Execute, Duration, EditingEntry };

export interface ExecAction {
  Exec: Execute
}