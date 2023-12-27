import { utimes } from 'fs/promises'
import { isPackageExists } from 'local-pkg'
import { dirname, resolve } from 'path'
import { debounce } from 'perfect-debounce'
import { argv } from 'process'
import UnoCss from 'unocss/vite'
import AutoImport from 'unplugin-auto-import/vite'
import {
	HeadlessUiResolver,
	NaiveUiResolver,
	PrimeVueResolver,
	QuasarResolver,
	TDesignResolver,
	VueUseComponentsResolver,
	Vuetify3Resolver,
} from 'unplugin-vue-components/resolvers'
import Components from 'unplugin-vue-components/vite'
import { VueRouterAutoImports } from 'unplugin-vue-router'
import Router from 'unplugin-vue-router/vite'
import { fileURLToPath } from 'url'
import { loadEnv } from 'vite'
import { AutoGenerateImports, vue3Presets } from 'vite-auto-import-resolvers'
import EnvTypes from 'vite-plugin-env-types'
import Removelog from 'vite-plugin-removelog'
import Modules from 'vite-plugin-use-modules'
import Layouts from 'vite-plugin-vue-meta-layouts'

import Legacy from '@vitejs/plugin-legacy'
import Vue from '@vitejs/plugin-vue'
import Jsx from '@vitejs/plugin-vue-jsx'

import type { ComponentResolver } from 'unplugin-vue-components/types'
import type { Plugin } from 'vite'

export const _dirname = dirname(fileURLToPath(import.meta.url))

export default function () {
	const env = useEnv()
	const plugins = [
		/**
		 * 兼容不支持 esmModule 的浏览器
		 * https://www.npmjs.com/package/@vitejs/plugin-legacy
		 */
		Legacy(),
		/**
		 * 环境变量类型提示
		 * https://github.com/dishait/vite-plugin-env-types
		 */
		EnvTypes({
			dts: 'presets/types/env.d.ts',
		}),
		/**
		 * 内置的预热，可以加快冷启动
		 */
		Warmup(),
		/**
		 * 文件路由
		 * https://github.com/posva/unplugin-vue-router
		 */
		Router({
			routesFolder: 'src/pages',
			dts: 'presets/types/type-router.d.ts',
			extensions: ['.vue', '.tsx', '.jsx'],
		}),
		/**
		 * 自动安装 vue 插件
		 * https://github.com/dishait/vite-plugin-use-modules
		 */
		Modules({
			auto: true,
			target: 'src/plugins',
		}),
		/**
		 * vue 官方插件，用来解析 sfc 单文件组件
		 * https://www.npmjs.com/package/@vitejs/plugin-vue
		 */
		Vue({
			include: [/\.vue$/],
		}),
		/**
		 * 布局系统
		 * https://github.com/dishait/vite-plugin-vue-meta-layouts
		 */
		Layouts(),
		/**
		 * 组件自动按需引入
		 * https://github.com/antfu/unplugin-vue-components
		 */
		Components({
			directoryAsNamespace: true,
			include: [/\.vue$/, /\.vue\?vue/, /\.[tj]sx$/],
			extensions: [ 'vue', 'tsx', 'jsx'],
			dts: resolve(_dirname, './types/components.d.ts'),
			types: [
				{
					from: 'vue-router',
					names: ['RouterLink', 'RouterView'],
				},
			],
			resolvers: normalizeResolvers({
				onlyExist: [
					[QuasarResolver(), 'quasar'],
					[NaiveUiResolver(), 'naive-ui'],
					[Vuetify3Resolver(), 'vuetify'],
					[PrimeVueResolver(), 'primevue'],
					[HeadlessUiResolver(), '@headlessui/vue'],
					[VueUseComponentsResolver(), '@vueuse/components'],
					[TDesignResolver({ library: 'vue-next' }), 'tdesign-vue-next'],
				],
			}),
		}),

		/**
		 * jsx 和 tsx 支持
		 * https://www.npmjs.com/package/@vitejs/plugin-vue-jsx
		 */
		Jsx(),

		/**
		 * 生产环境下移除 console.log, console.warn, console.error
		 * https://github.com/dishait/vite-plugin-removelog
		 */
		process.env.NODE_ENV !== 'debug' && Removelog(),
		/**
		 * 别名插件 (内置)
		 * 支持 `~` 和 `@` 别名到 `src`
		 */
		Alias(),
		/**
		 * 强制重启 (内置)
		 * 如果 package.json 或 pnpm-lock.yaml 更新的话，强制重启
		 */
		ForceRestart(),
	]

	if (env.VITE_APP_API_AUTO_IMPORT) {
		const dirs = env.VITE_APP_DIR_API_AUTO_IMPORT
			? ['src/stores/**', 'src/composables/**', 'src/api/**']
			: undefined
		/**
		 * api 自动按需引入
		 * https://github.com/antfu/unplugin-auto-import
		 */
		plugins.push(
			AutoImport({
				dirs,
				vueTemplate: true,
				dts: './presets/types/auto-imports.d.ts',
				imports: [
					...AutoGenerateImports({
						include: [...vue3Presets],
						exclude: ['vue-router'],
					}),
					VueRouterAutoImports,
				],
				resolvers: normalizeResolvers({
					onlyExist: [
						[TDesignResolver({ library: 'vue-next' }), 'tdesign-vue-next'],
					],
				}),
			}),
		)
	}

	/**
	 * css 原子引擎
	 * https://github.com/unocss/unocss
	 */
	plugins.push(
		UnoCss(),
	)

	return plugins
}

// 获取环境变量
function useEnv() {
	function detectMode() {
		const { NODE_ENV } = process.env
		const hasModeIndex = argv.findIndex((a) => a === '--mode' || a === '-m')
		if (hasModeIndex !== -1) {
			return argv[hasModeIndex + 1]
		}
		return NODE_ENV || 'development'
	}

	function stringToBoolean(v: string) {
		return Boolean(v === 'true' || false)
	}

	const {
		VITE_APP_TITLE,
		VITE_APP_API_AUTO_IMPORT,
		VITE_APP_DIR_API_AUTO_IMPORT,
	} = loadEnv(detectMode(), '.')

	return {
		VITE_APP_TITLE,
		VITE_APP_API_AUTO_IMPORT: stringToBoolean(VITE_APP_API_AUTO_IMPORT),
		VITE_APP_DIR_API_AUTO_IMPORT: stringToBoolean(VITE_APP_DIR_API_AUTO_IMPORT),
	}
}

type Arrayable<T> = T | Array<T>

interface Options {
	onlyExist?: [Arrayable<ComponentResolver>, string][]
	include?: ComponentResolver[]
}

/**
 * 规范化 resolvers
 */
export function normalizeResolvers(options: Options = {}) {
	const { onlyExist = [], include = [] } = options

	const existedResolvers = []
	for (let i = 0; i < onlyExist.length; i++) {
		const [resolver, packageName] = onlyExist[i]
		if (isPackageExists(packageName)) {
			existedResolvers.push(resolver)
		}
	}
	existedResolvers.push(...include)

	return existedResolvers
}

/**
 * 别名插件
 * @description 支持 `~` 和 `@` 别名到 `src`
 */
function Alias(): Plugin {
	const src = resolve(_dirname, '../src')
	return {
		name: 'vite-alias',
		enforce: 'pre',
		config(config) {
			config.resolve ??= {}
			config.resolve.alias = [
				{
					find: /^~/,
					replacement: src,
				},
				{
					find: /^@\//,
					replacement: src + '/',
				},
			]
		},
	}
}

/**
 * 强制重启
 * @description 如果 package.json 或 pnpm-lock.yaml 更新的话，强制重启项目
 */
function ForceRestart(paths = ['package.json', 'pnpm-lock.yaml']): Plugin {
	const restart = debounce(async function touch() {
		const time = new Date()
		await utimes('vite.config.ts', time, time)
	}, 1000)
	return {
		name: 'vite-plugin-force-restart',
		apply: 'serve',
		configureServer({ watcher }) {
			watcher.add(paths).on('all', async (_, path) => {
				if (paths.includes(path)) {
					await restart()
				}
			})
		},
	}
}

/**
 * 预热
 * @description 内置的预热，可以加快冷启动
 */
function Warmup(): Plugin {
	return {
		name: 'vite-plugin-warmup',
		config(config) {
			config?.server?.warmup?.clientFiles?.push('./src/**/*')
		},
	}
}
