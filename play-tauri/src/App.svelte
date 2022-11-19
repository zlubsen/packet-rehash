<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'
    import { open } from '@tauri-apps/api/dialog';
    import { appWindow } from "@tauri-apps/api/window";
    import { asString } from 'date-format';

    interface Settings {
        destination: string,
        source_port: number,
        ttl: number,
    }

    interface PlayerPosition {
        position: number,
        max_position: number,
        time_position: any,
        time_total: any,
    }

    type PlayerState = "" | "Initial" | "Playing" | "Paused" | "Finished" | "Quit";

    appWindow.listen("player_event_error", ({ event, payload }) => {
        // console.log(event);
        // console.log(payload);
    });
    appWindow.listen("player_event_ready", ({ event, payload }) => {
        // console.log(event);
        // console.log(payload);
    });
    appWindow.listen("player_event_state", ({ event, payload }) => {
        player_state = payload.state;
        // console.log(payload);
        // console.log(player_state);
    });
    appWindow.listen("player_event_position", ({ event, payload }) => {
        player_position = {
            position: payload.position,
            max_position: payload.max_position,
            // TODO extract time
            time_position: new Date(1970, 0, 1).setTime(payload.time_position.secs * 1000),
            time_total: new Date(1970, 0, 1).setTime(payload.time_total.secs * 1000),
        };
        // console.log(event);
        // console.log(payload);
    });
    appWindow.listen("player_event_quit", ({ event, payload }) => {
        // console.log(event);
        // console.log(payload);
    });

    let DEFAULT_SETTINGS = {
        destination: "192.168.8.255:3000",
        source_port: 3001,
        ttl: 1
    }

    let has_file_loaded : boolean = false;
    let setting_destination : string = DEFAULT_SETTINGS.destination;
    let setting_source_port : number = DEFAULT_SETTINGS.source_port;
    let setting_ttl : number = DEFAULT_SETTINGS.ttl;
    let current_file : string = "";
    let player_state :PlayerState = "";
    let player_position : PlayerPosition = {
        position: 0,
        max_position: 0,
        time_position: new Date(1970, 0, 1),
        time_total: new Date(1970, 0, 1),
    }

    function update_settings(event: MouseEvent) {
        invoke('cmd_update_settings', DEFAULT_SETTINGS)
            .then((message) => console.log(message))
            .catch((error) => console.error(error));
    }

    async function open_file(event: MouseEvent) {
        const selected = await open({
            multiple: false,
            filters: [{
                name: 'Recordings',
                extensions: ['pcap']
            }]
        });

        if (Array.isArray(selected)) {
            // user selected multiple files
        } else if (selected === null) {
            // user cancelled the selection
        } else {
            current_file = selected.slice(selected.lastIndexOf('/')+1, selected.length);
            invoke('cmd_open', {
                filePath: selected
            })
                .then((message) => has_file_loaded = true)
                .catch((error) => {
                    has_file_loaded = false;
                    console.error(error);
                });
        }
    }

    function play(event: MouseEvent) {
        invoke('cmd_play')
            .then((message) => console.log(message))
            .catch((error) => console.error(error));
    }

    function pause(event: MouseEvent) {
        invoke('cmd_pause')
            .then((message) => console.log(message))
            .catch((error) => console.error(error));
    }

    function rewind(event: MouseEvent) {
        invoke('cmd_rewind')
            .then((message) => console.log(message))
            .catch((error) => console.error(error));
    }

    function isUninitialised(): boolean {
        return player_state === "Uninitialised";
    }

    function isPlaying(): boolean {
        return player_state === "Playing";
    }

    function isPaused(): boolean {
        return player_state === "Paused";
    }

    function isInitial(): boolean {
        return player_state === "Initial";
    }

    function isFinished(): boolean {
        return player_state === "Finished";
    }
</script>

<main class="main-viewport border flex flex-col justify-between">
    <h1 class="py-2 grow-0">Packet Play</h1>

    <div class="grow flex justify-between p-0 border">
        <div class="flex-auto w-1/2 p-0">
            Player status
            {#if has_file_loaded}
                <div class="grid grid-cols-2 gap-4">
                    <div class="">Recording</div>
                    <div class="">{current_file}</div>
                    <div class="">State</div>
                    <div class="">{player_state}</div>
                    <div class="">Position</div>
                    <div class="">{player_position.position}/{player_position.max_position}</div>
                    <div class="">Time</div>
                    <div class="">{player_position.time_position}/{player_position.time_total}</div>
<!--                    <div class="">{asString('hh:mm:ss', player_position.time_position)}/{asString('hh:mm:ss', player_position.time_total)}</div>-->
                </div>
            {:else}
                <div>
                    No file loaded
                </div>
            {/if}
        </div>
        <div class="flex-auto w-1/2 p-0">
            Settings
            <div class="grid grid-cols-2 gap-4">
                <div class="">Destination</div>
                <div class="">{setting_destination}</div>
                <div class="">Source port</div>
                <div class="">{setting_source_port}</div>
                <div class="">TTL</div>
                <div class="">{setting_ttl}</div>
            </div>
        </div>
    </div>

    <div class="py-2 grow-0 px-4">
        Progress bar
        <progress class="progress progress-primary" value="{player_position.position}" max="{player_position.max_position}"></progress>
    </div>

    <div class="grow-0 flex flex-row flex-no-wrap justify-evenly w-full py-2">
        <button on:click={play} class="btn btn-primary btn-lg" class:btn-active={isPlaying()}>
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="player-control-icon">
                <path fill-rule="evenodd" d="M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12zm14.024-.983a1.125 1.125 0 010 1.966l-5.603 3.113A1.125 1.125 0 019 15.113V8.887c0-.857.921-1.4 1.671-.983l5.603 3.113z" clip-rule="evenodd" />
            </svg>
        </button>

        <button on:click={pause} class="btn btn-primary btn-lg" class:btn-active={isPaused()}>
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="player-control-icon">
                <path fill-rule="evenodd" d="M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12zM9 8.25a.75.75 0 00-.75.75v6c0 .414.336.75.75.75h.75a.75.75 0 00.75-.75V9a.75.75 0 00-.75-.75H9zm5.25 0a.75.75 0 00-.75.75v6c0 .414.336.75.75.75H15a.75.75 0 00.75-.75V9a.75.75 0 00-.75-.75h-.75z" clip-rule="evenodd" />
            </svg>
        </button>

        <button on:click={rewind} class="btn btn-primary btn-lg" class:btn-disabled={isInitial()}>
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="player-control-icon">
                <path fill-rule="evenodd" d="M9.195 18.44c1.25.713 2.805-.19 2.805-1.629v-2.34l6.945 3.968c1.25.714 2.805-.188 2.805-1.628V8.688c0-1.44-1.555-2.342-2.805-1.628L12 11.03v-2.34c0-1.44-1.555-2.343-2.805-1.629l-7.108 4.062c-1.26.72-1.26 2.536 0 3.256l7.108 4.061z" clip-rule="evenodd" />
            </svg>
        </button>

        <button on:click={open_file} class="btn btn-primary btn-lg">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="-2 -2 20 20" fill="currentColor" class="player-control-icon">
                <path fill-rule="evenodd" d="M7.27 1.047a1 1 0 0 1 1.46 0l6.345 6.77c.6.638.146 1.683-.73 1.683H1.656C.78 9.5.326 8.455.926 7.816L7.27 1.047zM.5 11.5a1 1 0 0 1 1-1h13a1 1 0 0 1 1 1v1a1 1 0 0 1-1 1h-13a1 1 0 0 1-1-1v-1z" clip-rule="evenodd" />
            </svg>
        </button>
    </div>
</main>

<style>
    .main-viewport {
        @apply h-screen w-screen overflow-x-hidden overflow-y-hidden;
        @apply p-0 m-0;
        min-height: -webkit-fill-available;
        width: 90vw;
        height: 90vh;
    }

    .player-control-icon {
        @apply w-10 h-10;
    }
</style>