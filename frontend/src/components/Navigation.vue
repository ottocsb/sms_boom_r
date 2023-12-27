<script setup lang="ts">
import { SwitchIcon } from 'vue-dark-switch'
</script>

<template>
	<nav
		aria-label="Site Nav"
		class="mx-auto h-80px max-w-3xl flex items-center justify-between p-4"
	>
		<div class="flex items-center justify-center space-x-5">
			<SwitchIcon unmount-persets />
		</div>
	</nav>
</template>
