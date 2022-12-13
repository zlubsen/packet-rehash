<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'
    import { open } from '@tauri-apps/api/dialog';
    import { listen } from '@tauri-apps/api/event'

    import { onMount, onDestroy } from 'svelte';

    import { themeChange } from 'theme-change';

    import type { PlayerPosition, PlayerState, RecordingInfo, Settings } from "./model";
    import Controls from "./lib/Controls.svelte";
    import SettingsPanel from "./lib/SettingsPanel.svelte";
    import PlayerInfo from "./lib/PlayerInfo.svelte";
    import ProgressBar from "./lib/ProgressBar.svelte";

    onMount(() => {
        themeChange(false);
    })

    onDestroy(async () => {
        (await unlisten_error)();
        (await unlisten_ready)();
        (await unlisten_state)();
        (await unlisten_position)();
        (await unlisten_quit)();
    })

    const ALLOWED_FILES : string[] = ['pcap'];

    const handlers = new Map();
    let handlers_tip: string;
    set_key_handlers();

    let recording_info: RecordingInfo = {
        is_loaded: false,
        filePath: "",
        shortFileName: "",
    };
    let player_state : PlayerState = "Uninitialised";
    let player_position : PlayerPosition = {
        position: 0,
        max_position: 0,
        time_position_secs: 0,
        time_total_secs: 0,
    };

    const unlisten_error = listen("player_event_error", ({ event, payload }) => {
        // TODO display error to user. Use toaster.
        console.error(payload);
    });
    const unlisten_ready = listen("player_event_ready", ({ event, payload }) => {});
    const unlisten_state = listen("player_event_state", ({ event, payload }) => {
        player_state = payload.state;
    });
    const unlisten_position = listen("player_event_position", ({ event, payload }) => {
        player_position = {
            position: payload.position,
            max_position: payload.max_position,
            time_position_secs: payload.time_position.secs,
            time_total_secs: payload.time_total.secs,
        };
    });
    const unlisten_quit = listen("player_event_quit", ({ event, payload }) => {});

    function set_key_handlers() {
        handlers.set('KeyO', { handler: key_open_file, key: "Ctrl+O", description: "Open file" });
        handlers.set('Space', { handler: key_play_pause, key: "Space", description: "Play/Pause" });
        handlers.set('KeyR', { handler: key_rewind, key: "r", description: "Rewind" });
        handlers_tip = "";
        handlers.forEach((v,k) => {
            handlers_tip += `${v.key} => ${v.description}</br>`
        });
    }

    function file_name_from_path(path: string) : string {
        const lastNixSlash = path.lastIndexOf('/');
        if(lastNixSlash>-1)
            return path.slice(lastNixSlash+1, path.length);
        else
            return path.slice(path.lastIndexOf('\\')+1, path.length);
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

    async function cmd_open_file() {
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
            recording_info.filePath = selected;
            recording_info.shortFileName = file_name_from_path(recording_info.filePath);
            invoke('cmd_open', {
                filePath: recording_info.filePath
            })
                .then((message) => recording_info.is_loaded = true)
                .catch((error) => {
                    recording_info.is_loaded = false;
                    console.error(error);
                });
        }
    }

    function cmd_play() {
        invoke('cmd_play')
            .then((message) => console.log(message))
            .catch((error) => console.error(error));
    }

    function cmd_pause() {
        invoke('cmd_pause')
            .then((message) => console.log(message))
            .catch((error) => console.error(error));
    }

    function cmd_rewind() {
        invoke('cmd_rewind')
            .then((message) => console.log(message))
            .catch((error) => console.error(error));
    }

    function click_open_file(event: PointerEvent) {
        cmd_open_file();
    }

    function click_play(event: PointerEvent) {
        cmd_play();
    }

    function click_pause(event: PointerEvent) {
        cmd_pause();
    }

    function click_rewind(event: PointerEvent) {
        cmd_rewind();
    }

    function key_handler(event: KeyboardEvent) {
        if (handlers.has(event.code)) {
            let handler = handlers.get(event.code);
            handler.handler(event);
        }
    }

    function key_open_file(event: KeyboardEvent) {
        if (event.ctrlKey) cmd_open_file();
    }

    function key_play_pause(event: KeyboardEvent) {
        switch(player_state) {
            case "Uninitialised":
                cmd_play();
                break;
            case "Initial":
                cmd_play()
                break;
            case "Playing":
                cmd_pause()
                break;
            case "Paused":
                cmd_play()
                break;
            case "Finished":
                cmd_rewind();
                break;
            case "Quit":
            default:
                break;
        }
    }

    function key_rewind(event: KeyboardEvent) {
        cmd_rewind();
    }
</script>

<svelte:window on:keypress={key_handler}></svelte:window>


<div class="drawer drawer-end">
    <input id="controls-info-drawer" type="checkbox" class="drawer-toggle" />
    <div class="drawer-content">
        <main class="main-viewport flex flex-col justify-between bg-base-100 border">
            <header class="p-2">
                <h1 class="grow-0 text-primary">Packet Play</h1>

                <div class="absolute top-0 right-0 text-primary-focus p-2 flex">
                    <label class="swap swap-rotate">
                        <input type="checkbox" data-toggle-theme="bumblebee,coffee" data-act-class="ACTIVECLASS"/>
                        <svg class="swap-on fill-current w-8 h-8" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M5.64,17l-.71.71a1,1,0,0,0,0,1.41,1,1,0,0,0,1.41,0l.71-.71A1,1,0,0,0,5.64,17ZM5,12a1,1,0,0,0-1-1H3a1,1,0,0,0,0,2H4A1,1,0,0,0,5,12Zm7-7a1,1,0,0,0,1-1V3a1,1,0,0,0-2,0V4A1,1,0,0,0,12,5ZM5.64,7.05a1,1,0,0,0,.7.29,1,1,0,0,0,.71-.29,1,1,0,0,0,0-1.41l-.71-.71A1,1,0,0,0,4.93,6.34Zm12,.29a1,1,0,0,0,.7-.29l.71-.71a1,1,0,1,0-1.41-1.41L17,5.64a1,1,0,0,0,0,1.41A1,1,0,0,0,17.66,7.34ZM21,11H20a1,1,0,0,0,0,2h1a1,1,0,0,0,0-2Zm-9,8a1,1,0,0,0-1,1v1a1,1,0,0,0,2,0V20A1,1,0,0,0,12,19ZM18.36,17A1,1,0,0,0,17,18.36l.71.71a1,1,0,0,0,1.41,0,1,1,0,0,0,0-1.41ZM12,6.5A5.5,5.5,0,1,0,17.5,12,5.51,5.51,0,0,0,12,6.5Zm0,9A3.5,3.5,0,1,1,15.5,12,3.5,3.5,0,0,1,12,15.5Z"/></svg>
                        <svg class="swap-off fill-current w-8 h-8" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z"/></svg>
                    </label>
                    <label for="controls-info-drawer" class="drawer-button">
                        <svg class="w-8 h-8 fill-current" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" >
                            <path fill-rule="evenodd" d="M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12zm8.706-1.442c1.146-.573 2.437.463 2.126 1.706l-.709 2.836.042-.02a.75.75 0 01.67 1.34l-.04.022c-1.147.573-2.438-.463-2.127-1.706l.71-2.836-.042.02a.75.75 0 11-.671-1.34l.041-.022zM12 9a.75.75 0 100-1.5.75.75 0 000 1.5z" clip-rule="evenodd" />
                        </svg>
                    </label>
                </div>
            </header>

            <div class="grow flex justify-between p-0">
                <PlayerInfo
                        bind:recording_info={recording_info}
                        bind:player_state={player_state}
                        bind:player_position={player_position} />
                <SettingsPanel
                        on:update={(event)=>cmd_update_settings(event.detail)} />
            </div>

            <ProgressBar bind:player_position={player_position} />

            <Controls on:play={click_play}
                      on:pause={click_pause}
                      on:rewind={click_rewind}
                      on:open={click_open_file}
                      bind:player_state={player_state} />
        </main>
    </div>
    <div class="drawer-side">
        <label for="controls-info-drawer" class="drawer-overlay"></label>
        <div class="overflow-x-auto w-1/2 h-screen bg-base-100">
            <table class="table table-zebra table-compact w-full">
                <thead>
                <tr>
                    <th>Key</th>
                    <th>Description</th>
                </tr>
                </thead>
                <tbody>
                    {#each [...handlers] as [key, value] }
                        <tr>
                            <td class="text-primary">{value.key}</td>
                            <td>{value.description}</td>
                        </tr>
                    {/each}
                </tbody>
            </table>
<!--            <div class="h-full bg-base-100" ></div>-->
        </div>
    </div>
</div>

<style>
    .main-viewport {
        @apply h-screen w-screen overflow-x-hidden overflow-y-hidden;
        min-height: -webkit-fill-available;
    }
</style>