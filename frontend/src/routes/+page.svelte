<script>
    import Card from "$lib/components/Card.svelte";

    const dirección = "presa.png"
    let cards = [
        {
            name: "patata",
            capacity: 30,
            image: dirección, // Ruta de la imagen
            place: "Málaga, Andalucía",
        },
        {
            name: "tomate",
            capacity: 40,
            image: dirección,
            place: "Málaga, Andalucía",
        },
        {
            name: "cebolla",
            capacity: 50,
            image: dirección,
            place: "Málaga, Andalucía",
        },
        {
            name: "zanahoria",
            capacity: 20,
            image: dirección,
            place: "Málaga, Andalucía",
        },
        {
            name: "lechuga",
            capacity: 25,
            image: dirección,
            place: "Málaga, Andalucía",
        },
        {
            name: "pepino",
            capacity: 15,
            image: dirección,
            place: "Málaga, Andalucía",
        },
    ];
</script>

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

<style>
    .card-container {
        display: flex;
        flex-wrap: wrap;
        gap: 20px; /* Aumenta el espacio entre las tarjetas */
    }

</style>

<div class="card-container">
    {#each cards as card}
        <Card
            name={card.name}
            capacity={card.capacity}
            image={card.image}
            place={card.place}
        />
    {/each}
</div>


