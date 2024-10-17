<script>
    import Card from "$lib/components/Card.svelte";

    let cards = [
        {
            name: "patata",
            distance: 30,
        },
        {
            name: "tomate",
            distance: 40,
        },
        {
            name: "cebolla",
            distance: 50,
        },
    ];
</script>

<!-- <svelte:head>
    <title>Water Management</title>
    <meta
        name="description"
        content="Manage and monitor water resources efficiently."
    />
</svelte:head> -->

<!-- <h1 class="text-3xl">Tus muertos</h1>
<h3 class="text-xl">Por si acaso</h3> -->

{#each cards as card}
    <Card />
{/each}

<div class="buttons">
    <button class="button is-info">Info</button>
    <button class="button is-success">Success</button>
    <button class="button is-warning">Warning</button>
    <button class="button is-danger">Danger</button>
</div>
