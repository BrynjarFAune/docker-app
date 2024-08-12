<script lang="ts">
	import { onMount } from 'svelte';

	interface Data {
		id: number;
		name: string;
		// add other fields as necessary
	}

	let data: Data | null = null;

	onMount(async () => {
		const response = await fetch('backend:8080');
		if (response.ok) {
			data = (await response.json()) as Data;
		}
	});
</script>

{#if data}
	<div>
		<h1>Data</h1>
		<pre>{JSON.stringify(data, null, 2)}</pre>
	</div>
{:else}
	<p>Loading...</p>
{/if}
