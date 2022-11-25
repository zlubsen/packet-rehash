<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'
    import { open } from '@tauri-apps/api/dialog';
    import { appWindow } from "@tauri-apps/api/window";

    import { onMount } from 'svelte';

    import { themeChange } from 'theme-change';

    import type {PlayerPosition, PlayerState, Settings} from "./model";
    import Controls from "./lib/Controls.svelte";
    import SettingsPane from "./lib/SettingsPane.svelte";

    onMount(() => {
        themeChange(false);
    })

    const ALLOWED_FILES : string[] = ['pcap'];

    let has_file_loaded : boolean = false;
    let current_file_path : string = "";
    let current_file : string = "";
    let player_state : PlayerState = "Uninitialised";
    let player_position : PlayerPosition = {
        position: 0,
        max_position: 0,
        time_position_secs: 0,
        time_total_secs: 0,
    }

    appWindow.listen("player_event_error", ({ event, payload }) => {
        // TODO display error to user.
        console.error(payload);
    });
    appWindow.listen("player_event_ready", ({ event, payload }) => {});
    appWindow.listen("player_event_state", ({ event, payload }) => {
        player_state = payload.state;
    });
    appWindow.listen("player_event_position", ({ event, payload }) => {
        player_position = {
            position: payload.position,
            max_position: payload.max_position,
            // TODO extract time
            time_position_secs: payload.time_position.secs,
            time_total_secs: payload.time_total.secs,
        };
    });
    appWindow.listen("player_event_quit", ({ event, payload }) => {});

    function file_name_from_path(path: string) : string {
        return path.slice(path.lastIndexOf('/')+1, path.length);
    }

    function cmd_update_settings(settings: Settings) {
        invoke('cmd_update_settings', {
            destination: settings.destination,
            sourcePort: settings.source_port,
            ttl: settings.ttl,
        })
            .then((message) => console.log(message))
            .catch((error) => console.error(error));
    }

    async function cmd_open_file(event: MouseEvent) {
        const selected = await open({
            multiple: false,
            filters: [{
                name: 'Recordings',
                extensions: ALLOWED_FILES
            }]
        });

        if (Array.isArray(selected)) {
            // user selected multiple files
        } else if (selected === null) {
            // user cancelled the selection
        } else {
            current_file_path = selected;
            current_file = file_name_from_path(current_file_path);
            invoke('cmd_open', {
                filePath: current_file_path
            })
                .then((message) => has_file_loaded = true)
                .catch((error) => {
                    has_file_loaded = false;
                    console.error(error);
                });
        }
    }

    function cmd_play(event: Event) {
        invoke('cmd_play')
            .then((message) => console.log(message))
            .catch((error) => console.error(error));
    }

    function cmd_pause(event: Event) {
        invoke('cmd_pause')
            .then((message) => console.log(message))
            .catch((error) => console.error(error));
    }

    function cmd_rewind(event: Event) {
        invoke('cmd_rewind')
            .then((message) => console.log(message))
            .catch((error) => console.error(error));
    }

    function formatSecs(seconds: number): string {
        console.log(seconds);
        if(seconds<0) {
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
</script>

<main class="main-viewport flex flex-col justify-between bg-base-100 border">
    <header class="p-2">
        <h1 class="grow-0 text-primary">Packet Play</h1>

        <div class="absolute top-0 right-0 text-primary-focus p-2">
            <label>help</label>
            <label class="swap swap-rotate">
                <input type="checkbox" data-toggle-theme="bumblebee,coffee" data-act-class="ACTIVECLASS"/>
                <svg class="swap-on fill-current w-8 h-8" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M5.64,17l-.71.71a1,1,0,0,0,0,1.41,1,1,0,0,0,1.41,0l.71-.71A1,1,0,0,0,5.64,17ZM5,12a1,1,0,0,0-1-1H3a1,1,0,0,0,0,2H4A1,1,0,0,0,5,12Zm7-7a1,1,0,0,0,1-1V3a1,1,0,0,0-2,0V4A1,1,0,0,0,12,5ZM5.64,7.05a1,1,0,0,0,.7.29,1,1,0,0,0,.71-.29,1,1,0,0,0,0-1.41l-.71-.71A1,1,0,0,0,4.93,6.34Zm12,.29a1,1,0,0,0,.7-.29l.71-.71a1,1,0,1,0-1.41-1.41L17,5.64a1,1,0,0,0,0,1.41A1,1,0,0,0,17.66,7.34ZM21,11H20a1,1,0,0,0,0,2h1a1,1,0,0,0,0-2Zm-9,8a1,1,0,0,0-1,1v1a1,1,0,0,0,2,0V20A1,1,0,0,0,12,19ZM18.36,17A1,1,0,0,0,17,18.36l.71.71a1,1,0,0,0,1.41,0,1,1,0,0,0,0-1.41ZM12,6.5A5.5,5.5,0,1,0,17.5,12,5.51,5.51,0,0,0,12,6.5Zm0,9A3.5,3.5,0,1,1,15.5,12,3.5,3.5,0,0,1,12,15.5Z"/></svg>
                <svg class="swap-off fill-current w-8 h-8" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z"/></svg>
            </label>
        </div>
    </header>

    <div class="grow flex justify-between p-0 bg-base-200">
        <div class="flex-auto w-1/2 p-2">
            <span class="text-primary">Player status</span>
            {#if has_file_loaded}
                <div class="grid grid-cols-2 gap-4 py-1">
                    <label class="label"><span class="label-text">Recording</span></label>
                    <label class="tooltip label text-left" data-tip="{current_file_path}"><span class="label-text">{current_file}</span></label>
                    <label class="label"><span class="label-text">State</span></label>
                    <label class="label"><span class="label-text">{player_state}</span></label>
                    <label class="label"><span class="label-text">Packets</span></label>
                    <label class="label"><span class="label-text">{player_position.position}/{player_position.max_position}</span></label>
                    <label class="label"><span class="label-text">Length</span></label>
                    <label class="label"><span class="label-text">{formatSecs(player_position.time_total_secs)}</span></label>
                </div>
            {:else}
                <div class="label-text py-1">
                    <span class="text-secondary">Open a file</span>
                </div>
            {/if}
        </div>
        <SettingsPane on:update={(settings)=>cmd_update_settings(settings)} />
    </div>

    <div class="py-2 grow-0 px-4">
        <progress class="progress progress-secondary" value="{player_position.position}" max="{player_position.max_position}"></progress>
        <span class="text-secondary">[{formatSecs(player_position.time_position_secs)}] / [{formatSecs(player_position.time_total_secs)}]</span>
    </div>

    <Controls on:play={cmd_play} on:pause={cmd_pause} on:rewind={cmd_rewind} on:open={cmd_open_file} bind:player_state={player_state} />
</main>

<style>
    .main-viewport {
        @apply h-screen w-screen overflow-x-hidden overflow-y-hidden;
        /*@apply p-0 m-0;*/
        min-height: -webkit-fill-available;
        /*width: 90vw;*/
        /*height: 90vh;*/
    }
</style>