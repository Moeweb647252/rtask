export interface Command {
  cmd: string
}

export enum LoggerType {
  Console,
  File,
  None
}

export interface Logger {
  type: LoggerType,
  path: string | null
}

export enum TimerType {
  Repeat,
  Once,
  ManyTimes,
  None,
}

export interface Timer {
  type: TimerType,
  time: Date | null,
}

export interface Entry {
  id: number
  name: string | null
  action: Command | null
  env: string | null
  working_dir: string | null
  logger: Logger | null

}
