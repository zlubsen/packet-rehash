<script lang="ts">
    import type {Settings} from "../model";
    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher();

    const DEFAULT_SETTINGS : Settings = {
        destination: "192.168.8.255:3000",
        source_port: 33000,
        ttl: 1
    }

    let destination : string = DEFAULT_SETTINGS.destination;
    let source_port : number = DEFAULT_SETTINGS.source_port;
    let ttl : number = DEFAULT_SETTINGS.ttl;

    function update(event: MouseEvent) {
        dispatch('update', {
            destination: destination,
            source_port: source_port,
            ttl: ttl,
        })
    }
</script>

<div class="flex-auto w-1/2 p-2">
    <span class="text-primary">Settings</span>
    <div class="grid grid-cols-2 gap-4 form-control py-1">
        <label class="label"><span class="label-text">Destination</span></label>
        <input type="text" id="input-destination" bind:value={destination} placeholder="{DEFAULT_SETTINGS.destination}" class="input input-secondary input-sm">
        <label class="label"><span class="label-text">Source port</span></label>
        <input type="text" id="input-port" bind:value={source_port} placeholder="{DEFAULT_SETTINGS.source_port}" class="input input-secondary input-sm">
        <label class="label"><span class="label-text">Time To Live</span></label>
        <input type="text" id="input-ttl" bind:value={ttl} placeholder="{DEFAULT_SETTINGS.ttl}" class="input input-secondary input-sm">
        <div class="col-span-2 text-right"><button class="btn btn-sm" on:click={update}>Save</button></div>
    </div>
</div>