<script>
    import Card from "$lib/components/Card.svelte";

    export let data;

    let txt = "";

    $: filtered_data = data.embalses.filter((e) =>
        e.embalse.toLowerCase().startsWith(txt),
    );
    let page = 1;
</script>

<svelte:head>
    <title>About</title>
    <meta name="description" content="About this app" />
</svelte:head>

<div class="section">
    <div class="container">
        <div class="spacer"></div>

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

        <nav class="pagination" role="navigation" aria-label="pagination">
            <ul class="pagination-list">
                <li>
                    <a
                        href="#"
                        class="pagination-link"
                        aria-label="Goto page {page - 1}">{page - 1}</a
                    >
                </li>
                <li>
                    <a
                        class="pagination-link is-current"
                        aria-label="Page {page}"
                        aria-current="page">{page}</a
                    >
                </li>
                <li>
                    <a
                        href="/#"
                        class="pagination-link"
                        aria-label="Goto page {page + 1}">{page + 1}</a
                    >
                </li>
            </ul>
        </nav>
    </div>
</div>

<style>
    .spacer {
        height: 20px; /* Altura del espacio vacío */
    }

    .pagination {
        background-color: #f8f9fa; /* Color de fondo de la paginación */
        padding: 10px;
        border-radius: 5px;
    }

    .pagination-list {
        list-style: none;
        display: flex;
        gap: 10px;
        padding: 0;
        margin: 0;
    }

    .pagination-link {
        color: #405d72; /* Color del texto de los enlaces */
        text-decoration: none;
        padding: 5px 10px;
        border: 1px solid #405d72;
        border-radius: 3px;
        transition:
            background-color 0.3s,
            color 0.3s;
    }
    .pagination-link:hover {
        background-color: #405d72;
        color: #fff8f3; /* Color del texto al pasar el ratón por encima */
    }

    .pagination-link.is-current {
        background-color: #405d72;
        color: #fff8f3; /* Color del texto del enlace actual */
        border-color: #405d72;
    }
</style>
