<script setup lang="ts">
defineProps<{ msg: string }>()
</script>

<template>
	<div class="mt-10 flex flex-col items-center space-y-7">
		<h1 class="text-4xl">{{ msg }}</h1>
	</div>
</template>

<style scoped>
@keyframes slide-up {
	from {
		transform: translateY(20px);
		opacity: 0;
	}
	to {
		transform: translateY(0);
		opacity: 1;
	}
}
</style>
