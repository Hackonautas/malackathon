<script>
    import Card from "$lib/components/Card.svelte";

    const dirección = "presa.png";
</script>

<svelte:head>
    <title>Water Management</title>
    <meta
        name="description"
        content="Manage and monitor water resources efficiently."
    />
</svelte:head>

<section class="hero is-large is-link is-bold">
    <div class="hero-body">
        <p class="title">Alveus: Plataforma de embalses de España</p>
        <p class="subtitle">Por un uso y administración sostenible</p>
        <a class="button is-inverted is-link" href="/listPage"
            >Ir al listado de embalses</a
        >
    </div>
</section>

<div class="spacer"></div>

<div class="spacer"></div>

<style>
    .card-container {
        display: flex;
        flex-wrap: wrap;
        gap: 20px; /* Aumenta el espacio entre las tarjetas */
    }
    .spacer {
        height: 20px; /* Altura del espacio vacío */
    }
</style>
