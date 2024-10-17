<script>
    import "../app.css";
</script>

<div class="navbar bg-blue-50">
    <div class="flex-1">
        <a href="/" class="btn btn-ghost text-xl">Alveus</a>
    </div>
    <div class="flex-none">
        <ul class="menu menu-horizontal px-1">
            <li>
                <a href="/login" class="btn btn-ghost">Lorem Ipsum</a>
            </li>
            <li>
                <a href="/register" class="btn btn-ghost">Involucrate</a>
            </li>
        </ul>
    </div>
</div>

<main class="container mx-auto px-4">
    <slot />
</main>
