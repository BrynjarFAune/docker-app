<script>
	import ComponetEx from './ComponetEx.svelte';
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" />
</svelte:head>
<section>
	<h1>
		<span class="welcome"> Hello. </span>
	</h1>
	<ComponetEx />
</section>

<style>
	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 0.6;
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
