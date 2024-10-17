<script>
	import { page } from '$app/stores';
	import logo from '$lib/images/svelte-logo.svg';
  </script>
  
  <header class="bg-blue-900 p-4 flex items-center justify-between">
	<div class="flex items-center space-x-4">
	  <a href="https://kit.svelte.dev">
		<img src={logo} alt="SvelteKit" class="h-10" />
	  </a>
	  <h1 class="text-2xl font-bold text-white">Water Management</h1>
	</div>
  
	<nav class="flex items-center space-x-4">
	  <ul class="flex space-x-4">
		<li aria-current={$page.url.pathname === '/' ? 'page' : undefined}>
		  <a href="/" class="text-white hover:text-blue-400">Home</a>
		</li>
		<li aria-current={$page.url.pathname === '/about' ? 'page' : undefined}>
		  <a href="/about" class="text-white hover:text-blue-400">About</a>
		</li>
		<li aria-current={$page.url.pathname === '/contact' ? 'page' : undefined}>
		  <a href="/contact" class="text-white hover:text-blue-400">Contact</a>
		</li>
	  </ul>
	</nav>
  </header>
  
  <style>
	header {
	  border-bottom: 1px solid #2a4365;
	}
  </style>