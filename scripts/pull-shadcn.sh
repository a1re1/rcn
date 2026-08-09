#!/bin/zsh
# Pull a real shadcn component (and its dependency components) into the
# reference project so it can be audited next to the underlying libraries
# (@base-ui-components/react, tw-animate-css) in node_modules.
#
# Usage: tools/pull-shadcn.sh <component> [ref-project-dir]
set -e
NAME="$1"
REF="${2:-${SHADCN_REF:-/tmp/rcn-shadcn-ref}}"
if [ ! -d "$REF" ]; then
  echo "setting up reference project at $REF"
  mkdir -p "$REF/src" && cd "$REF"
  bun init -y > /dev/null
  bun add react react-dom @base-ui-components/react tw-animate-css tailwindcss > /dev/null
  echo '@import "tailwindcss";' > src/index.css
  cat > tsconfig.json <<'JSON'
{ "compilerOptions": { "baseUrl": ".", "paths": { "@/*": ["./src/*"] } } }
JSON
  cat > components.json <<'JSON'
{
  "$schema": "https://ui.shadcn.com/schema.json",
  "style": "base-vega",
  "rsc": false,
  "tsx": true,
  "tailwind": { "config": "", "css": "src/index.css", "baseColor": "neutral", "cssVariables": true },
  "aliases": { "components": "@/components", "utils": "@/lib/utils", "ui": "@/components/ui", "lib": "@/lib", "hooks": "@/hooks" }
}
JSON
fi
cd "$REF"
bunx --bun shadcn@latest add "$NAME" --yes --overwrite
echo "--- $REF/src/components/ui/$NAME.tsx"
