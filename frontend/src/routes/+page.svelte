<script lang="ts">
	import Counter from './Counter.svelte';
	import { onMount } from 'svelte';

	interface Data {
		name: string;
		// add other fields as necessary
	}

	let data: Data | null = null;

	onMount(async () => {
		const response = await fetch('backend:8080/');
		if (response.ok) {
			data = (await response.json()) as Data;
		}
	});
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Custom brynjar container app" />
</svelte:head>

<section>
	<h1 class="welcome">Welcome to Brynjar's page!</h1>

	<Counter />
</section>
<section>
	{#if data}
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
