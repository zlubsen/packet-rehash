<script lang="ts">
    import {formatSecs} from "../utils";
    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher();

    export let player_position;
    let slider;
    let slider_input = false;
    let slider_position = 0;
    $: if (slider_input) {
        slider_position = player_position.position;
    };
    let seek_preview = 0;

    function change(event) {
        dispatch('seek', slider.value);
    }

    function input(event) {
        seek_preview = slider.value;
    }

    function drag_start(event) {
        slider_input = true;
    }
    function drag_end(event) {
        slider_input = false;
    }
</script>

<div class="pt-4 py-2 grow-0 px-4">
    <div class="group min-h-12">
        <div class="align-middle block group-hover:hidden">
            <progress class="align-middle progress progress-secondary" value="{player_position.position}" max="{player_position.max_position}"></progress>
        </div>
        <div class="align-middle hidden group-hover:block">
            <input type="range" id="seek_slider" bind:this={slider} class="align-middle range range-secondary" min="0" max="{player_position.max_position}" value="{player_position.position}"
                   on:change={change}
                   on:input={input}
                   on:mousedown={drag_start}
                   on:mouseup={drag_end}/>
        </div>
    </div>
    {#if slider_input}
        <span class="text-primary">Move to packet {seek_preview}</span>
    {/if}
    <span class="text-secondary">[{formatSecs(player_position.time_position_secs)}] / [{formatSecs(player_position.time_total_secs)}]</span>
</div>