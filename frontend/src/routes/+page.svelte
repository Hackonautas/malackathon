<script>
    import Card from "$lib/components/Card.svelte";

    const dirección = "presa.png"
</script>

<div class="spacer"></div>

<div class="field">
    <p class="control has-icons-left has-icons-right">
        <input class="input is-rounded" type="text" placeholder="Nombre del pantano o embalse" />
      <span class="icon is-left">
        <i class="fa-solid fa-magnifying-glass"></i>
      </span>
    </p>
  </div>

<svelte:head>
    <title>Water Management</title>
    <meta
        name="description"
        content="Manage and monitor water resources efficiently."
    />
</svelte:head>

<section class="hero is-large is-primary">
    <div class="hero-body">
      <p class="title">Alveus: Plataforma de embalses de España</p>
      <p class="subtitle">Por un uso y administración sostenible</p>
    </div>
  </section>

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

<div class="spacer"></div>


