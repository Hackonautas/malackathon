<script>
    export let name;
    export let ccaa;
    export let provincia;
</script>

<div class="card card-height">
    <div class="card-content">
        <div class="media">
            <div class="media-content">
                <p class="title is-4">{name}</p>
                <p>Ubicación: {provincia}, {ccaa}</p>
            </div>
        </div>
    </div>
</div>

<style>
    .card-height {
        height: 12em;
    }
</style>
