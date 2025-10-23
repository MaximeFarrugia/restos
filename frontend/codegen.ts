import { CodegenConfig } from '@graphql-codegen/cli'
import { loadEnv } from 'vite';

const env = loadEnv(process.env.NODE_ENV || 'development', process.cwd());

const config: CodegenConfig = {
  schema: env.VITE_GRAPHQL_CODEGEN_URL,
  documents: ['src/**/*.svelte'],
  ignoreNoDocuments: true,
  generates: {
    './src/lib/graphql/': {
      preset: 'client',
      plugins: [],
      config: {
        useTypeImports: true,
      },
    },
  },
}

export default config
