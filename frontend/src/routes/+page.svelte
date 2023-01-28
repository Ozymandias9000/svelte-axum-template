<script lang="ts">
	import { onMount } from 'svelte';
	import { getApi } from '../api';

	let token = '123456789';
	let response = '';

	onMount(async () => {
		response = await getApi(token);
	});
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
<p class="mono">RESPONSE: {response}</p>
