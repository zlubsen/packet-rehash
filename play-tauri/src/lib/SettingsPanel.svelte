<script lang="ts">
    import type {Settings} from "../model";
    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher();

    const DEFAULT_SETTINGS : Settings = {
        destination: "192.168.8.255:3000",
        source_port: 33000,
        ttl: 1
    }

    let can_edit: boolean = false;

    let destination : string = DEFAULT_SETTINGS.destination;
    let source_port : string = DEFAULT_SETTINGS.source_port.toString();
    let ttl : string = DEFAULT_SETTINGS.ttl.toString();

    let stash: Settings = {
        destination: destination,
        source_port: parseInt(source_port),
        ttl: parseInt(ttl),
    };

    function update(event: MouseEvent) {
        let new_settings: Settings = {
            destination: destination,
            source_port: parseInt(source_port),
            ttl: parseInt(ttl),
        };
        dispatch('update', new_settings);
        can_edit = false;
    }

    function enableEdit(event: MouseEvent) {
        stashSettings();
        can_edit = true;
    }

    function disableEdit(event: MouseEvent) {
        unstashSettings();
        can_edit = false;
    }

    function stashSettings() {
        stash = {
            destination: destination,
            source_port: parseInt(source_port),
            ttl: parseInt(ttl),
        };
    }

    function unstashSettings() {
        destination = stash.destination;
        source_port = stash.source_port.toString();
        ttl = stash.ttl.toString();
    }
</script>

<div class="flex-auto w-1/2 p-2 mx-1 bg-base-200 rounded">
    <span class="text-primary">Settings</span>
    <div class="grid grid-cols-2 gap-4 form-control py-1">
        {#if can_edit}
<!--            <div class="form-control">-->
<!--            <label class="input-group input-group-sm">-->
<!--                <span>Destination</span>-->
<!--                <input type="text" id="input-destination" bind:value={destination} placeholder="{DEFAULT_SETTINGS.destination}" class="input input-secondary input-sm" />-->
<!--            </label>-->
<!--            </div>-->
<!--            <div class="form-control">-->
<!--            <label class="input-group input-group-sm">-->
<!--                <span>Source port</span>-->
<!--                <input type="text" disabled={!can_edit} id="input-port" bind:value={source_port} placeholder="{DEFAULT_SETTINGS.source_port}" class="input input-secondary input-sm" />-->
<!--            </label>-->
<!--            </div>-->
<!--            <div class="form-control">-->
<!--            <label class="input-group input-group-sm">-->
<!--                <span>Time To Live</span>-->
<!--                <input type="text" id="input-ttl" bind:value={ttl} placeholder="{DEFAULT_SETTINGS.ttl}" class="input input-secondary input-sm" />-->
<!--            </label>-->
<!--            </div>-->
            <label class="label"><span class="label-text">Destination</span></label>
            <input type="text" id="input-destination" bind:value={destination} placeholder="{DEFAULT_SETTINGS.destination}" class="input input-secondary text-primary input-sm">
            <label class="label"><span class="label-text">Source port</span></label>
            <input type="text" id="input-port" bind:value={source_port} placeholder="{DEFAULT_SETTINGS.source_port}" class="input input-secondary text-primary input-sm">
            <label class="label"><span class="label-text">Time To Live</span></label>
            <input type="text" id="input-ttl" bind:value={ttl} placeholder="{DEFAULT_SETTINGS.ttl}" class="input input-secondary text-primary input-sm">
            <div class="col-span-2 text-right">
                <button class="btn btn-outline btn-sm" on:click={disableEdit}>Cancel</button>
                <button class="btn btn-outline btn-sm" on:click={update}>Save</button>
            </div>
        {:else}
            <label class="label"><span class="label-text">Destination</span></label>
            <label class="label"><span class="label-text">{destination}</span></label>
            <label class="label"><span class="label-text">Source port</span></label>
            <label class="label"><span class="label-text">{source_port}</span></label>
            <label class="label"><span class="label-text">Time To Live</span></label>
            <label class="label"><span class="label-text">{ttl}</span></label>
            <div class="col-span-2 text-right"><button class="btn btn-outline btn-sm" on:click={enableEdit}>Edit</button></div>
        {/if}
    </div>
</div>