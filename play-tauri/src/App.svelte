<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'
    import { open } from '@tauri-apps/api/dialog';
    import { appWindow } from "@tauri-apps/api/window";
    import { asString } from 'date-format';
    // const format = require('date-format');

    appWindow.listen("player_event_error", ({ event, payload }) => {
        console.log(event);
        console.log(payload);
    });
    appWindow.listen("player_event_ready", ({ event, payload }) => {
        console.log(event);
        console.log(payload);
    });
    appWindow.listen("player_event_state", ({ event, payload }) => {
        player_state = payload;
        console.log(event);
        console.log(payload);
    });
    appWindow.listen("player_event_position", ({ event, payload }) => {
        player_position = {
            position: payload.position,
            max_position: payload.max_position,
            // TODO extract time
            time_position: new Date(1970, 0, 1).setTime(payload.time_position.secs * 1000),
            time_total: new Date(1970, 0, 1).setTime(payload.time_total.secs * 1000)
        };
        console.log(event);
        console.log(payload);
    });
    appWindow.listen("player_event_quit", ({ event, payload }) => {
        console.log(event);
        console.log(payload);
    });

    let SETTINGS = {
        destination: "192.168.8.255:3000",
        source_port: 3001,
        ttl: 1
    }
    let player_state = {
        state: 'Load a file'
    };
    let player_position = {
        position: 0,
        max_position: 0,
        time_position: 0,
        time_total: 0
    }

    function update_settings() {
        invoke('cmd_update_settings', SETTINGS)
            .then((message) => console.log(message))
            .catch((error) => console.error(error));
    }

    async function open_file() {
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
            invoke('cmd_open', {
                filePath: selected
            })
                .then((message) => console.log(message))
                .catch((error) => console.error(error));
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
</script>

<main class="main-viewport border m-0">
    <h1 class="py-2">Packet Play</h1>

    <div class="flex justify-between p-0 border">
        <div class="flex-auto w-1/2 p-0">
            Player status
            <div class="grid grid-cols-2 gap-4">
                <div class="">State</div>
                <div class="">{player_state.state}</div>
                <div class="">Position</div>
                <div class="">{player_position.position}/{player_position.max_position}</div>
                <div class="">Time</div>
                <div class="">{asString('hh:mm:ss', player_position.time_position)}/{asString('hh:mm:ss', player_position.time_total)}</div>
            </div>
        </div>
        <div class="flex-auto w-1/2 p-0">
            Settings
            <div class="grid grid-cols-2 gap-4">
                <div class="">Destination</div>
                <div class="">value</div>
                <div class="">Source port</div>
                <div class="">value</div>
                <div class="">TTL</div>
                <div class="">value</div>
            </div>
        </div>
    </div>

    <div class="py-2">
        Progress bar
    </div>

    <div class="flex flex-row flex-no-wrap justify-evenly w-full py-2">
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="player-control hover:bg-blue-100" on:click={play}>
            <path fill-rule="evenodd" d="M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12zm14.024-.983a1.125 1.125 0 010 1.966l-5.603 3.113A1.125 1.125 0 019 15.113V8.887c0-.857.921-1.4 1.671-.983l5.603 3.113z" clip-rule="evenodd" />
        </svg>

        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="player-control hover:bg-blue-100" on:click={pause}>
            <path fill-rule="evenodd" d="M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12zM9 8.25a.75.75 0 00-.75.75v6c0 .414.336.75.75.75h.75a.75.75 0 00.75-.75V9a.75.75 0 00-.75-.75H9zm5.25 0a.75.75 0 00-.75.75v6c0 .414.336.75.75.75H15a.75.75 0 00.75-.75V9a.75.75 0 00-.75-.75h-.75z" clip-rule="evenodd" />
        </svg>

        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="player-control hover:bg-blue-100" on:click={rewind}>
            <path fill-rule="evenodd" d="M9.195 18.44c1.25.713 2.805-.19 2.805-1.629v-2.34l6.945 3.968c1.25.714 2.805-.188 2.805-1.628V8.688c0-1.44-1.555-2.342-2.805-1.628L12 11.03v-2.34c0-1.44-1.555-2.343-2.805-1.629l-7.108 4.062c-1.26.72-1.26 2.536 0 3.256l7.108 4.061z" clip-rule="evenodd" />
        </svg>

        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="player-control hover:bg-blue-100" on:click={open_file}>
            <path fill-rule="evenodd" d="M19.906 9c.382 0 .749.057 1.094.162V9a3 3 0 00-3-3h-3.879a.75.75 0 01-.53-.22L11.47 3.66A2.25 2.25 0 009.879 3H6a3 3 0 00-3 3v3.162A3.756 3.756 0 014.094 9h15.812zM4.094 10.5a2.25 2.25 0 00-2.227 2.568l.857 6A2.25 2.25 0 004.951 21H19.05a2.25 2.25 0 002.227-1.932l.857-6a2.25 2.25 0 00-2.227-2.568H4.094z" />
        </svg>
<!--        <svg xmlns="http://www.w3.org/2000/svg" viewBox="-2 -2 20 20" fill="currentColor" class="player-control w-16 h-16 hover:bg-blue-100" on:click={open_file}>-->
<!--            <path fill-rule="evenodd" d="M7.27 1.047a1 1 0 0 1 1.46 0l6.345 6.77c.6.638.146 1.683-.73 1.683H1.656C.78 9.5.326 8.455.926 7.816L7.27 1.047zM.5 11.5a1 1 0 0 1 1-1h13a1 1 0 0 1 1 1v1a1 1 0 0 1-1 1h-13a1 1 0 0 1-1-1v-1z" clip-rule="evenodd" />-->
<!--        </svg>-->
    </div>
</main>

<style>
    .main-viewport {
        width: 90vw;
        height: 90vh;
    }

    .player-control {
        @apply w-12 h-12 cursor-pointer;
    }
</style>