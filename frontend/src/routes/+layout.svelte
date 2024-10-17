<script>
    import "../app.css";
</script>

<style>
    .navbar {
        background-color: #405D72; /* Color de fondo de la barra de navegación */
        color: #FFF8F3; /* Color del texto de la barra de navegación */
    }

    .navbar-item {
        color: #FFF8F3; /* Color del texto de los elementos de la barra de navegación */
    }

    .navbar-item:hover {
        background-color: #758694; /* Color de fondo al pasar el ratón por encima */
    }

    main.container {
        background-color: #f0f0f0; /* Color de fondo del contenedor principal */
        color: #333; /* Color del texto del contenedor principal */
    }
    
</style>

<nav class="navbar" role="navigation" aria-label="main navigation">
    <div class="navbar-brand">
        <a class="navbar-item" href="/">
            ALVEUS
        </a>

        <a
            role="button"
            class="navbar-burger"
            aria-label="menu"
            aria-expanded="false"
            data-target="navbarBasicExample"
        >
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
        </a>
    </div>

    <div id="navbarBasicExample" class="navbar-menu">
        <div class="navbar-start">
            <a class="navbar-item" href="/listPage"> Listado </a>

            <a class="navbar-item" href="/about"> Información </a>

            <a class="navbar-item"> Ajustes y Accesibilidad </a>

            <a class="navbar-item" href="/contact"> Contacto </a>

        </div>

        <div class="navbar-end">
            <div class="navbar-item">
                <div class="buttons">
                    <a class="button is-primary">
                        <strong>Sign up</strong>
                    </a>
                    <a class="button is-light"> Log in </a>
                </div>
            </div>
        </div>
    </div>
</nav>

<main class="container mx-auto px-4">
    <slot />
</main>
