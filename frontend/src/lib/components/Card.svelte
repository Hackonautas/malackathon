<script lang="ts">
    export let name: string;
    export let distance: number;
</script>

<div class="card bg-base-100 w-96 shadow-xl">
    <div class="card-body">
        <h2 class="card-title">{name}</h2>
        <p>{distance} km</p>
    </div>
</div>
