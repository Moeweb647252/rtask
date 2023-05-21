export interface Entry {
  id: number
  name: string | null
  action: {}
  env: string[] | null
  working_dir: string | null
  logger: string | null | {string: string}
  timer: {}
}
