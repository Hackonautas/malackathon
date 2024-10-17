<script>
  export let image;
  export let name;
  export let capacity;
  export let place;

</script>

<div class="card">
  <img src={image} alt={name} />
  <h3>{name}</h3>
  <p>Capacidad: {capacity} hm<sup>3</p>
  <p>Ubicación: {place} </p>
</div>

<style>
      .card {
        width: 200px; /* Ancho más grande */
        height: 300px; /* Alto más grande */
        padding: 10px;
        border: 1px solid #ddd;
        box-shadow: 2px 2px 10px rgba(0, 0, 0, 0.1);
        text-align: center;
        background-color: #fff;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
    }

    .card img {
        width: 100%;
        height: 150px; /* Asegura que las imágenes tengan una altura constante */
        object-fit: cover; /* Las imágenes se ajustarán al espacio sin distorsión */
        border-radius: 5px;
    }

    .card h3 {
        margin: 10px 0 5px;
    }

    .card p {
        margin: 0;
    }
</style>