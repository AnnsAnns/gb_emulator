import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
	trailingSlash: 'always',
	integrations: [
		starlight({
			title: 'GB Emulator by HAW WP Rust SoSe24',
			social: {
				github: 'https://github.com/HaW-Rust-SoSe24/gb_emulator',
			},
			sidebar: [
				{
					label: 'Guides',
					items: [
						// Each item here is one entry in the navigation menu.
						{ label: 'Example Guide', link: '/guides/example/' },
					],
				},
				{
					label: 'Reference',
					autogenerate: { directory: 'reference' },
				},
			],
			editLink: { baseUrl: 'https://github.com/HAW-Rust-SoSe24/gb_emulator/tree/main/docs/' },
		}),
	],
});
