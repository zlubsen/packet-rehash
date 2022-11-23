<script lang="ts">
    import {disableBtn, canPlay, canPause, canRewind} from "../utils";
    import type {PlayerState} from "../model";
    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher();

    function play() {
        dispatch('play', {});
    }

    function pause() {
        dispatch('play', {});
    }

    function rewind() {
        dispatch('play', {});
    }

    function open_file() {
        dispatch('open', {});
    }

    export let player_state: PlayerState;
</script>

<div class="grow-0 flex flex-row flex-no-wrap justify-evenly w-full py-5">
    <div class="tooltip tooltip-primary" data-tip="Replay recording">
        <button on:click={play} class="btn btn-primary btn-lg" disabled="{disableBtn(!canPlay(player_state))}">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="player-control-icon">
                <path fill-rule="evenodd" d="M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12zm14.024-.983a1.125 1.125 0 010 1.966l-5.603 3.113A1.125 1.125 0 019 15.113V8.887c0-.857.921-1.4 1.671-.983l5.603 3.113z" clip-rule="evenodd" />
            </svg>
        </button>
    </div>

    <div class="tooltip tooltip-primary" data-tip="Pause playback">
        <button on:click={pause} class="btn btn-primary btn-lg" disabled="{disableBtn(!canPause(player_state))}">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="player-control-icon">
                <path fill-rule="evenodd" d="M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12zM9 8.25a.75.75 0 00-.75.75v6c0 .414.336.75.75.75h.75a.75.75 0 00.75-.75V9a.75.75 0 00-.75-.75H9zm5.25 0a.75.75 0 00-.75.75v6c0 .414.336.75.75.75H15a.75.75 0 00.75-.75V9a.75.75 0 00-.75-.75h-.75z" clip-rule="evenodd" />
            </svg>
        </button>
    </div>

    <div class="tooltip tooltip-primary" data-tip="Rewind to beginning">
        <button on:click={rewind} class="btn btn-primary btn-lg" disabled="{disableBtn(!canRewind(player_state))}">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="player-control-icon">
                <path fill-rule="evenodd" d="M9.195 18.44c1.25.713 2.805-.19 2.805-1.629v-2.34l6.945 3.968c1.25.714 2.805-.188 2.805-1.628V8.688c0-1.44-1.555-2.342-2.805-1.628L12 11.03v-2.34c0-1.44-1.555-2.343-2.805-1.629l-7.108 4.062c-1.26.72-1.26 2.536 0 3.256l7.108 4.061z" clip-rule="evenodd" />
            </svg>
        </button>
    </div>

    <div class="tooltip tooltip-primary" data-tip="Open a recording">
        <button on:click={open_file} class="btn btn-primary btn-lg">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="-2 -2 20 20" fill="currentColor" class="player-control-icon">
                <path fill-rule="evenodd" d="M7.27 1.047a1 1 0 0 1 1.46 0l6.345 6.77c.6.638.146 1.683-.73 1.683H1.656C.78 9.5.326 8.455.926 7.816L7.27 1.047zM.5 11.5a1 1 0 0 1 1-1h13a1 1 0 0 1 1 1v1a1 1 0 0 1-1 1h-13a1 1 0 0 1-1-1v-1z" clip-rule="evenodd" />
            </svg>
        </button>
    </div>
</div>

<style>
    .player-control-icon {
        @apply w-10 h-10;
    }
</style>