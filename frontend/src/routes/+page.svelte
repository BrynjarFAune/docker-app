<script lang="ts">
	import { onMount } from 'svelte';

	let data: any = null;
	let error: string | null = null;

	onMount(async () => {
		try {
			const res = await fetch('http://backend:8000/');
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
			<h1>Data</h1>
			<pre>{JSON.stringify(data, null, 2)}</pre>
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
