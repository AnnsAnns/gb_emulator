import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
	base: '/gb_emulator',
	trailingSlash: 'always',
	site: 'https://haw-rust-sose24.github.io',
	integrations: [
		starlight({
			title: 'GB Emulator by HAW WP Rust SoSe24',
			social: {
				github: 'https://github.com/HaW-Rust-SoSe24/gb_emulator',
			},
			sidebar: [
				{
					label: 'Software Architecture (Arc42)',
					autogenerate: { directory: 'arc42' },
				},
			],
			editLink: { baseUrl: 'https://github.com/HAW-Rust-SoSe24/gb_emulator/tree/main/docs/' },
		}),
	],
});
