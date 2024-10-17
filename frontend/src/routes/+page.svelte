<script>
	import { onMount } from 'svelte';
	import logo from '$lib/images/water-logo.png';
  
	/** @type {{ id: number, name: string, level: string }[]} */
	let waterData = [];
  
	onMount(async () => {
	  // Simulate fetching data
	  waterData = await fetchWaterData();
	});
  
	async function fetchWaterData() {
	  return [
		{ id: 1, name: 'Reservoir A', level: '75%' },
		{ id: 2, name: 'Reservoir B', level: '60%' },
		{ id: 3, name: 'Reservoir C', level: '90%' },
	  ];
	}
  </script>
  
  <svelte:head>
	<title>Water Management</title>
	<meta name="description" content="Manage and monitor water resources efficiently." />
  </svelte:head>
  
  <div class="min-h-screen bg-gray-900 text-white">
	<header class="bg-blue-900 p-4 flex items-center justify-between">
	  <img src={logo} alt="Water Management Logo" class="h-10" />
	  <h1 class="text-2xl font-bold">Water Management</h1>
	</header>
  
	<main class="p-6">
	  <section class="mb-8">
		<h2 class="text-xl font-semibold mb-4">Reservoir Levels</h2>
		<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
		  {#each waterData as reservoir}
			<div class="card bg-blue-800 shadow-xl">
			  <div class="card-body">
				<h3 class="card-title">{reservoir.name}</h3>
				<p>Water Level: {reservoir.level}</p>
			  </div>
			</div>
		  {/each}
		</div>
	  </section>
  
	  <section>
		<h2 class="text-xl font-semibold mb-4">Add New Reservoir</h2>
		<form class="space-y-4">
		  <div class="form-control">
			<label class="label">
			  <span class="label-text">Reservoir Name</span>
			</label>
			<input type="text" placeholder="Enter name" class="input input-bordered w-full" />
		  </div>
		  <div class="form-control">
			<label class="label">
			  <span class="label-text">Water Level</span>
			</label>
			<input type="text" placeholder="Enter level" class="input input-bordered w-full" />
		  </div>
		  <button type="submit" class="btn btn-primary">Add Reservoir</button>
		</form>
	  </section>
	</main>
  
	<footer class="bg-blue-900 p-4 text-center">
	  <p>&copy; 2023 Water Management. All rights reserved.</p>
	</footer>
  </div>
  
  <style>
	.card {
	  border-radius: 0.5rem;
	}
  </style>
