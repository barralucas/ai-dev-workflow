#!/usr/bin/env bash
# Bootstrap — copia o esqueleto do AI Dev Workflow para um projeto.
# Uso:
#   cd /caminho/para/projeto-novo
#   bash /caminho/para/ai-dev-workflow/scripts/bootstrap.sh
#
# Flags:
#   --update     Atualiza apenas arquivos do workflow (sem mexer em docs/ existentes).
#   --stack X    Pré-seleciona stack (nextjs|node-backend|python|mobile|none).
#   --name NAME  Pré-define nome do projeto (substitui {{PROJECT_NAME}}).

set -euo pipefail

# --- Localiza diretório do workflow (origem) ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKFLOW_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
TARGET_DIR="$(pwd)"

UPDATE_MODE=false
STACK=""
PROJECT_NAME=""

# --- Parse args ---
while [[ $# -gt 0 ]]; do
  case "$1" in
    --update) UPDATE_MODE=true; shift ;;
    --stack) STACK="$2"; shift 2 ;;
    --name) PROJECT_NAME="$2"; shift 2 ;;
    *) echo "Flag desconhecida: $1" >&2; exit 1 ;;
  esac
done

echo "→ Workflow source: $WORKFLOW_DIR"
echo "→ Target project:  $TARGET_DIR"
echo

# --- Confirmação ---
if [[ "$TARGET_DIR" == "$WORKFLOW_DIR" ]]; then
  echo "ERRO: você está rodando o bootstrap dentro do próprio repositório do workflow." >&2
  exit 1
fi

if [[ "$UPDATE_MODE" == false ]]; then
  read -r -p "Bootstrap em $TARGET_DIR? [y/N] " ans
  [[ "$ans" =~ ^[Yy]$ ]] || { echo "Abortado."; exit 0; }
fi

# --- Nome do projeto ---
if [[ -z "$PROJECT_NAME" ]]; then
  default_name="$(basename "$TARGET_DIR")"
  read -r -p "Nome do projeto [${default_name}]: " input_name
  PROJECT_NAME="${input_name:-$default_name}"
fi
echo "→ Nome do projeto: $PROJECT_NAME"

# --- Stack ---
if [[ -z "$STACK" ]]; then
  echo
  echo "Escolha a stack principal:"
  echo "  1) nextjs"
  echo "  2) node-backend"
  echo "  3) python"
  echo "  4) mobile"
  echo "  5) none (multi-stack ou outra — manter todos os adendos)"
  read -r -p "Opção [1]: " stack_opt
  case "${stack_opt:-1}" in
    1) STACK="nextjs" ;;
    2) STACK="node-backend" ;;
    3) STACK="python" ;;
    4) STACK="mobile" ;;
    5) STACK="none" ;;
    *) echo "Opção inválida"; exit 1 ;;
  esac
fi
echo "→ Stack: $STACK"
echo

# --- Copia .github (preservando arquivos existentes do projeto) ---
echo "→ Copiando .github/..."
mkdir -p "$TARGET_DIR/.github"
cp -R "$WORKFLOW_DIR/.github/." "$TARGET_DIR/.github/"

# --- Filtra stacks: mantém só a escolhida ---
if [[ "$STACK" != "none" ]]; then
  STACKS_DIR="$TARGET_DIR/.github/instructions/stacks"
  for f in "$STACKS_DIR"/*.instructions.md; do
    base="$(basename "$f" .instructions.md)"
    if [[ "$base" != "$STACK" ]]; then
      rm -f "$f"
    fi
  done
fi

# --- AGENTS.md e CLAUDE.md ---
echo "→ Copiando AGENTS.md e CLAUDE.md..."
cp "$WORKFLOW_DIR/AGENTS.md" "$TARGET_DIR/AGENTS.md"
cp "$WORKFLOW_DIR/CLAUDE.md" "$TARGET_DIR/CLAUDE.md"

# --- Templates de docs ---
if [[ "$UPDATE_MODE" == false ]]; then
  if [[ -d "$TARGET_DIR/docs" ]]; then
    echo "⚠️  docs/ já existe — pulando cópia (use git diff para mesclar manualmente)."
  else
    echo "→ Copiando docs/ a partir de templates..."
    mkdir -p "$TARGET_DIR/docs"
    cp -R "$WORKFLOW_DIR/templates/docs/." "$TARGET_DIR/docs/"
  fi
fi

# --- Substitui placeholders ---
echo "→ Substituindo {{PROJECT_NAME}} → $PROJECT_NAME..."
# macOS sed precisa de '' após -i; usa fallback portável
SED_INPLACE=( -i )
case "$(uname)" in
  Darwin) SED_INPLACE=( -i '' ) ;;
esac

# Apenas em arquivos .md
find "$TARGET_DIR/docs" -type f -name '*.md' -exec sed "${SED_INPLACE[@]}" "s/{{PROJECT_NAME}}/${PROJECT_NAME}/g" {} \; 2>/dev/null || true
find "$TARGET_DIR/.github" -type f -name '*.md' -exec sed "${SED_INPLACE[@]}" "s/{{PROJECT_NAME}}/${PROJECT_NAME}/g" {} \; 2>/dev/null || true

# --- README sugerido (não sobrescreve) ---
if [[ ! -f "$TARGET_DIR/README.md" ]]; then
  echo "→ Criando README.md a partir do template..."
  cp "$WORKFLOW_DIR/templates/README.template.md" "$TARGET_DIR/README.md"
  sed "${SED_INPLACE[@]}" "s/{{PROJECT_NAME}}/${PROJECT_NAME}/g" "$TARGET_DIR/README.md"
fi

# --- .env.example mínimo ---
if [[ ! -f "$TARGET_DIR/.env.example" ]]; then
  echo "→ Criando .env.example vazio..."
  cat > "$TARGET_DIR/.env.example" <<EOF
# Variáveis de ambiente — copie para .env.local e preencha.
# NUNCA comite .env.local com valores reais.
EOF
fi

# --- gitignore mínimo (apenas se não existir) ---
if [[ ! -f "$TARGET_DIR/.gitignore" ]]; then
  echo "→ Criando .gitignore mínimo..."
  cat > "$TARGET_DIR/.gitignore" <<'EOF'
# OS
.DS_Store
Thumbs.db

# Editor
.idea/
.vscode/*
!.vscode/settings.json
!.vscode/extensions.json

# Env
.env
.env.local
.env.*.local

# Logs
*.log
npm-debug.log*
pnpm-debug.log*
yarn-debug.log*
EOF
fi

echo
echo "✅ Bootstrap concluído."
echo
echo "Próximos passos:"
echo "  1. Edite docs/progress/PROGRESS.md (defina sprint atual)."
echo "  2. Edite docs/architecture/tech-stack.md (versões reais)."
echo "  3. Finalize docs/adr/0001-stack-inicial.md."
echo "  4. Adicione primeiras stories em docs/user-stories/backlog.md."
echo "  5. git add -A && git commit -m \"chore: bootstrap ai-dev-workflow\""
echo
