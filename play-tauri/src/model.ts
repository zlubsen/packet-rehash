interface Settings {
    destination: string,
    source_port: number,
    ttl: number,
}

interface PlayerPosition {
    position: number,
    max_position: number,
    time_position_secs: any,
    time_total_secs: any,
}

interface RecordingInfo {
    is_loaded: boolean,
    filePath: string,
    shortFileName: string,
}

type PlayerState = "Uninitialised" | "Initial" | "Playing" | "Paused" | "Finished" | "Quit";

export type {Settings, PlayerPosition, PlayerState, RecordingInfo};