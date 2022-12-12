<script lang="ts">
    import type {PlayerPosition, PlayerState, RecordingInfo} from "../model";
    import {formatSecs} from "../utils.js";

    export let recording_info: RecordingInfo = {
        is_loaded: false,
        filePath: "",
        shortFileName: "",
    };
    export let player_state: PlayerState = "Uninitialised";
    export let player_position: PlayerPosition = {
        position: 0,
        max_position: 0,
        time_position_secs: 0,
        time_total_secs: 0,
    };

</script>

<div class="flex-auto w-1/2 p-2 mx-1 bg-base-200 rounded">
    <span class="text-primary">Player status</span>
    {#if recording_info.is_loaded}
        <div class="grid grid-cols-3 gap-4 py-1">
            <label class="label"><span class="label-text">Recording</span></label>
            <label class="tooltip label text-left col-span-2" data-tip="{recording_info.filePath}"><span class="label-text whitespace-pre-wrap">{recording_info.shortFileName}</span></label>
            <label class="label"><span class="label-text">State</span></label>
            <label class="label col-span-2"><span class="label-text">{player_state}</span></label>
            <label class="label"><span class="label-text">Packets</span></label>
            <label class="label col-span-2"><span class="label-text">{player_position.position}/{player_position.max_position}</span></label>
            <label class="label"><span class="label-text">Length</span></label>
            <label class="label col-span-2"><span class="label-text">{formatSecs(player_position.time_total_secs)}</span></label>
        </div>
    {:else}
        <div class="label-text py-1">
            <span class="text-info">No recording loaded</span>
        </div>
    {/if}
</div>