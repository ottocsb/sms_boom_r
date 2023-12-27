<script lang="ts" setup>
const {data,loading,error}=getJson()

</script>

<template>
	<div>
		{{data}}
		<HelloWorld msg="Tov + Vite + Vue" />
		<Counter />
	</div>
</template>

<style>
a {
	color: rgba(37, 99, 235);
}

p {
	padding: 0 10px;
}

</style>
