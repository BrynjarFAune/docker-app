<script lang="ts">
	import { onMount } from 'svelte';

	interface Data {
		message: String;
	}
	let data: Data;
	let error: string | null = null;

	const backendUrl = import.meta.env.VITE_BACKEND_URL || 'http://localhost:8000/';

	onMount(async () => {
		try {
			const res = await fetch(`${backendUrl}`);
			data = await res.json();
			console.log(data);
		} catch (err) {
			error = 'Failed to load data';
		}
	});
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Custom brynjar container app" />
</svelte:head>

<section>
	<h1 class="welcome">Welcome to Brynjar's page!</h1>
</section>
<section>
	<h1>Data section:</h1>
	{#if error}
		<p class="err">{error}</p>
	{:else if data}
		<div>
			<p>{data.message}</p>
		</div>
	{:else}
		<p>Loading...</p>
	{/if}
</section>

<style>
	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		text-align: center;
		flex: 0.6;
		text-align: center;
	}

	h1 {
		width: 100%;
	}

	.welcome {
		display: block;
		position: relative;
		width: 100%;
		height: 0;
		padding: 0 0 calc(100% * 495 / 2048) 0;
	}
</style>
