export interface Exec {
  Exec: {
    env: { [key: string]: string } | null,
    working_dir: string | null,
    executable: string,
    user: string | null,
    args: string[],
  }
}

export interface Timer {

}

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
