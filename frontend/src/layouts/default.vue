<template>
	<n-space vertical>
		<n-layout position = "absolute">
			<n-layout-header>
				<Navigation />
			</n-layout-header>
			<n-layout-content content-style = "padding: 24px;" class = "w-full flex flex-col items-center justify-center">
				<router-view v-slot = "{ Component }">
					<transition name = "fade" mode = "out-in">
						<component :is = "Component" />
					</transition>
				</router-view>
			</n-layout-content>
		</n-layout>
	</n-space>
</template>

<style>
.fade-enter-active,
.fade-leave-active {
	transition: opacity 0.25s ease;
}

.fade-enter-from,
.fade-leave-to {
	opacity: 0;
}
</style>

