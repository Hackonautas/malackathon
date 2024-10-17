<script>
    import Card from "$lib/components/Card.svelte";

    export let data;
</script>

<svelte:head>
    <title>About</title>
    <meta name="description" content="About this app" />
</svelte:head>

<div class="field">
    <p class="control has-icons-left has-icons-right">
        <input
            class="input is-rounded"
            type="text"
            placeholder="Nombre del pantano o embalse"
        />
        <span class="icon is-left">
            <i class="fa-solid fa-magnifying-glass"></i>
        </span>
    </p>
</div>

<div class="columns is-multiline">
    {#each data.embalses as embalse}
        <div class="column is-3">
            <Card
                name={embalse.embalse}
                provincia={embalse.provincia}
                ccaa={embalse.ccaa}
            />
        </div>
    {/each}
</div>
