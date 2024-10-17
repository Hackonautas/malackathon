<script>
    import Card from "$lib/components/Card.svelte";

    export let data;

    let txt = "";

    $: filtered_data = data.embalses.filter((e) =>
        e.embalse.toLowerCase().startsWith(txt),
    );
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
            bind:value={txt}
        />
        <span class="icon is-left">
            <i class="fa-solid fa-magnifying-glass"></i>
        </span>
    </p>
</div>

<div class="columns is-multiline">
    {#each filtered_data as embalse}
        <div class="column is-3">
            <Card
                name={embalse.embalse}
                provincia={embalse.provincia}
                ccaa={embalse.ccaa}
            />
        </div>
    {/each}
</div>
