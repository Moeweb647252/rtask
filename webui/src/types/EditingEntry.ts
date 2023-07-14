export type EditingEntry = {
  id: number,
  name: string,
  action: {
    type: "Exec",
    content: {
      Exec: {
        env: Array<[string, string]>,
        working_dir: string,
        executable: string,
        user: {
          type: "Unix" | "Windows",
          content: {
            Unix: {
              uid: number,
              gid: number,
              username: string,
            },
            Windows: {
              username: string,
              group_windows: Array<string>,
            }
          }
        },
        args: Array<string>
      }
    }
  },
  logger: {
    type: "Default" | "Off" | "File",
    content: {
      File: string,
    }
  },
  trigger: {
    type: "Timer" | "None",
    content: {
      Timer: {
        type: "Repeat" | "Once" | "ManyTimes" | "Never",
        content: {
          year: number,
          month: number,
          day: number,
          hour: number,
          min: number,
          sec: number,
          timestamp: bigint,
          time_zone: {
            type: "Utc" | "Local" | "Offset",
            content: {
              Offset: number
            }
          },
          ManyTimes: number,
          total_sec: bigint,
        }
      }
    }
  },
  status: "Error" | "Running" | "Paused" | "Pending",
  DoIfRunning: "StartNew" | "Stop" | "Restart" | "Continue",
  enabled: boolean
}