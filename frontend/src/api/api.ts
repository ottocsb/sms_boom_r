import { useRequest } from 'vue-request'

enum url {
	jsonData = '/getJson',
}

export const getJson = () => {
	const { data, loading, error } = useRequest(
		() => http.get(url.jsonData))
	return { data, loading, error }
}
