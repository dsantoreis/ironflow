import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

export default defineConfig({
  site: 'https://dsantoreis.github.io/ironflow',
  base: '/ironflow',
  integrations: [
    starlight({
      title: 'Ironflow Docs',
      description: 'Production-grade ingest → transform → export pipeline in Rust.',
      defaultLocale: 'en',
      social: {
        github: 'https://github.com/dsantoreis/ironflow'
      },
      sidebar: [
        {
          label: 'Get Started',
          autogenerate: { directory: '.' }
        }
      ]
    })
  ]
});
