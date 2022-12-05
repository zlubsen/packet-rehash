<script lang="ts">
    import {formatSecs} from "../utils";

    export let player_position;
</script>

<div class="py-2 grow-0 px-4">
    <progress class="progress progress-secondary" value="{player_position.position}" max="{player_position.max_position}"></progress>
    <span class="text-secondary">[{formatSecs(player_position.time_position_secs)}] / [{formatSecs(player_position.time_total_secs)}]</span>
</div>