import type {PlayerState} from "./model";

function canPlay(state: PlayerState) : boolean {
    return state === "Initial" ||
        state === "Paused";
}

function canPause(state: PlayerState) : boolean {
    return state === "Playing";
}

function canRewind(state: PlayerState) : boolean {
    return state === "Playing" ||
        state === "Paused" ||
        state === "Finished";
}

// function isUninitialised(state: PlayerState): boolean {
//     return state === "Uninitialised";
// }
//
// function isPlaying(state: PlayerState): boolean {
//     return state === "Playing";
// }
//
// function isPaused(state: PlayerState): boolean {
//     return state === "Paused";
// }
//
// function isInitial(state: PlayerState): boolean {
//     return state === "Initial";
// }
//
// function isFinished(state: PlayerState): boolean {
//     return state === "Finished";
// }

function disableBtn(state: boolean): string {
    return state ? "disabled" : "";
}

function formatSecs(seconds: number): string {
    if (seconds < 0) {
        return ""
    }
    const hrs = Math.floor(seconds / 3600);
    const mins = Math.floor((seconds % 3600) / 60);
    const secs = Math.floor((seconds % 3600) % 60);
    const hrs_fmt = hrs < 10 ? `0${hrs}` : `${hrs}`;
    const mins_fmt = mins < 10 ? `0${mins}` : `${mins}`;
    const secs_fmt = secs < 10 ? `0${secs}` : `${secs}`;

    return `${hrs_fmt}:${mins_fmt}:${secs_fmt}`;
}

export {canPlay, canPause, canRewind,
    /*isUninitialised, isInitial, isPlaying, isPaused, isFinished,*/
    disableBtn, formatSecs};
