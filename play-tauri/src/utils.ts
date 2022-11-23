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

export {canPlay, canPause, canRewind, /*isUninitialised, isInitial, isPlaying, isPaused, isFinished,*/ disableBtn};